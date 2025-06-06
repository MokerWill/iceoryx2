// Copyright (c) 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides the trait [`GroupExt`] to create groups from strings by interpreting them as group
//! name or from unsigned integers by interpreting them as group id. The [`Group`] struct provides
//! access to the properties of a POSIX group.
//!
//! # Example
//!
//! ## Working with groups
//!
//! ```rust,ignore
//! use iceoryx2_bb_posix::group::*;
//! use iceoryx2_bb_system_types::group_name::GroupName;
//! use iceoryx2_bb_container::semantic_string::*;
//!
//! let myself = Group::from_self().expect("failed to get group");
//! let root = Group::from_name(&GroupName::new(b"root").unwrap())
//!                     .expect("failed to get root group");
//!
//! println!("I am in group {:?} and the root group is {:?}", myself, root);
//!
//! println!("Members of my group:");
//! for member in myself.members() {
//!     println!("{}", member);
//! }
//! ```
//!
//! ## Use the trait
//!
//! ```rust,ignore
//! use iceoryx2_bb_posix::group::*;
//!
//! println!("Members of group root");
//! for member in "root".as_group().unwrap().members() {
//!     println!("{}", member);
//! }
//! ```

use iceoryx2_bb_container::byte_string::strnlen;
use iceoryx2_bb_container::semantic_string::*;
use iceoryx2_bb_elementary::enum_gen;
use iceoryx2_bb_system_types::{group_name::GroupName, user_name::UserName};
use iceoryx2_pal_posix::posix::{errno::Errno, MemZeroedStruct};
use iceoryx2_pal_posix::*;

use crate::{config::GROUP_BUFFER_SIZE, system_configuration::*};
use iceoryx2_bb_log::fail;

enum_gen! { GroupError
  entry:
    Interrupt,
    IOerror,
    PerProcessFileHandleLimitReached,
    SystemWideFileHandleLimitReached,
    InsufficientBufferSize,
    GroupNotFound,
    SystemGroupNameLengthLongerThanSupportedLength,
    SystemUserNameLengthLongerThanSupportedLength,
    InvalidGroupName,
    UnknownError(i32)
}

/// Trait to create a [`Group`] from an integer by interpreting it as the gid or from a [`String`]
/// or [`str`] by interpreting the value as group name.
pub trait GroupExt {
    fn as_group(&self) -> Result<Group, GroupError>;
}

impl GroupExt for u32 {
    fn as_group(&self) -> Result<Group, GroupError> {
        Group::from_gid(*self)
    }
}

impl GroupExt for String {
    fn as_group(&self) -> Result<Group, GroupError> {
        Group::from_name(
            &fail!(from "String::as_group()", when GroupName::new(self.as_bytes()),
                        with GroupError::InvalidGroupName,
                        "Failed to create group object since the name \"{}\" contains invalid characters.",
                        self),
        )
    }
}

impl GroupExt for &str {
    fn as_group(&self) -> Result<Group, GroupError> {
        Group::from_name(
            &fail!(from "&str::as_group()", when GroupName::new(self.as_bytes()),
                        with GroupError::InvalidGroupName,
                        "Failed to create group object since the name \"{}\" contains invalid characters.",
                        self),
        )
    }
}

impl GroupExt for GroupName {
    fn as_group(&self) -> Result<Group, GroupError> {
        Group::from_name(self)
    }
}

/// Contains additional details of the [`Group`] that might be not available on every platform or
/// on every platform configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupDetails {
    name: GroupName,
    members: Vec<UserName>,
}

impl GroupDetails {
    /// Return the group name
    pub fn name(&self) -> &GroupName {
        &self.name
    }

    /// Returns a list of all the group members as string
    pub fn members(&self) -> Vec<UserName> {
        self.members.clone()
    }
}

/// Represents a group in a POSIX system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    gid: u32,
    details: Option<GroupDetails>,
}

impl Group {
    /// Create an group object from the owners group of the process
    pub fn from_self() -> Result<Group, GroupError> {
        Self::from_gid(unsafe { posix::getgid() })
    }

    /// Create an group object from a given gid. If the gid does not exist an error will be
    /// returned.
    pub fn from_gid(gid: u32) -> Result<Group, GroupError> {
        let mut group = posix::group::new_zeroed();
        let mut group_ptr: *mut posix::group = &mut group;
        let mut buffer: [posix::c_char; GROUP_BUFFER_SIZE] = [0; GROUP_BUFFER_SIZE];

        let origin = format!("Group::from_gid({})", gid);
        let msg = "Unable to acquire group entry";
        let errno_value = unsafe {
            posix::getgrgid_r(
                gid,
                &mut group,
                buffer.as_mut_ptr(),
                GROUP_BUFFER_SIZE,
                &mut group_ptr,
            )
            .into()
        };

        Self::handle_errno(errno_value, msg, &origin)?;
        Self::extract_group_details(msg, &origin, group_ptr, &mut group)
    }

    fn handle_errno(errno_value: Errno, msg: &str, origin: &str) -> Result<(), GroupError> {
        handle_errno!(GroupError, from origin,
            errno_source errno_value, continue_on_success,
            success Errno::ESUCCES => (),
            Errno::EINTR => (Interrupt, "{} since an interrupt signal was received", msg ),
            Errno::EIO => (IOerror, "{} due to an I/O error.", msg),
            Errno::EMFILE => (PerProcessFileHandleLimitReached, "{} since the per-process file handle limit is reached.", msg ),
            Errno::ENFILE => (SystemWideFileHandleLimitReached, "{} since the system-wide file handle limit is reached.", msg),
            Errno::ERANGE => (InsufficientBufferSize, "{} since insufficient storage was provided. Max buffer size should be: {}", msg, Limit::MaxSizeOfPasswordBuffer.value()),
            v => (UnknownError(v as i32), "{} due to an unknown error ({}).", msg, v)
        );

        Ok(())
    }

    /// Create an group object from a given group-name. If the group-name does not exist an error will
    /// be returned
    pub fn from_name(group_name: &GroupName) -> Result<Group, GroupError> {
        let mut group = posix::group::new_zeroed();
        let mut group_ptr: *mut posix::group = &mut group;
        let mut buffer: [posix::c_char; GROUP_BUFFER_SIZE] = [0; GROUP_BUFFER_SIZE];

        let origin = format!("Group::from_name({})", group_name);
        let msg = "Unable to acquire group entry";

        let errno_value = unsafe {
            posix::getgrnam_r(
                group_name.as_c_str(),
                &mut group,
                buffer.as_mut_ptr(),
                GROUP_BUFFER_SIZE,
                &mut group_ptr,
            )
            .into()
        };

        Self::handle_errno(errno_value, msg, &origin)?;
        Self::extract_group_details(msg, &origin, group_ptr, &mut group)
    }

    /// Return the group id
    pub fn gid(&self) -> u32 {
        self.gid
    }

    /// Returns the optional [`GroupDetails`] that might be not available on every platform or
    /// on every platform configuration.
    pub fn details(&self) -> Option<&GroupDetails> {
        self.details.as_ref()
    }

    fn extract_group_details(
        msg: &str,
        origin: &str,
        group_ptr: *mut posix::group,
        group: &mut posix::group,
    ) -> Result<Self, GroupError> {
        if group_ptr.is_null() {
            fail!(from origin, with GroupError::GroupNotFound, "{} since the group does not exist.", msg);
        }

        let gid = group.gr_gid;
        let name = fail!(from origin, when unsafe{ GroupName::from_c_str(group.gr_name) },
                            with GroupError::SystemGroupNameLengthLongerThanSupportedLength,
                            "{} since the group name length ({}) is greater than the supported group name length of {}.",
                            msg, unsafe { strnlen(group.gr_name, GroupName::max_len()) }, GroupName::max_len() );

        let mut counter: isize = 0;
        let mut members = vec![];
        loop {
            let group_member = unsafe { *group.gr_mem.offset(counter) };
            if group_member.is_null() {
                break;
            }

            members
                .push(fail!(from origin, when unsafe { UserName::from_c_str(group_member) },
                        with GroupError::SystemUserNameLengthLongerThanSupportedLength,
                        "{} since the user name length ({}) is greater than the support user name length of {}.",
                        msg, unsafe { strnlen(group_member, UserName::max_len()) }, UserName::max_len() ));
            counter += 1;
        }

        Ok(Group {
            gid,
            details: Some(GroupDetails { name, members }),
        })
    }
}
