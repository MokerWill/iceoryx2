// Copyright (c) 2025 Contributors to the Eclipse Foundation
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

use core::ops::Deref;
use core::{fmt::Debug, marker::PhantomData};

use crate::service;

/// todo
pub struct Response<Service: crate::service::Service, ResponsePayload: Debug, ResponseHeader: Debug>
{
    _service: PhantomData<Service>,
    _response_payload: PhantomData<ResponsePayload>,
    _response_header: PhantomData<ResponseHeader>,
}

impl<Service: crate::service::Service, ResponsePayload: Debug, ResponseHeader: Debug> Debug
    for Response<Service, ResponsePayload, ResponseHeader>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Response<{}, {}, {}> {{ }}",
            core::any::type_name::<Service>(),
            core::any::type_name::<ResponsePayload>(),
            core::any::type_name::<ResponseHeader>()
        )
    }
}

impl<Service: crate::service::Service, ResponsePayload: Debug, ResponseHeader: Debug> Deref
    for Response<Service, ResponsePayload, ResponseHeader>
{
    type Target = ResponsePayload;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<Service: crate::service::Service, ResponsePayload: Debug, ResponseHeader: Debug>
    Response<Service, ResponsePayload, ResponseHeader>
{
    /// todo
    pub fn header(&self) -> &service::header::request_response::ResponseHeader {
        todo!()
    }

    /// todo
    pub fn user_header(&self) -> &ResponseHeader {
        todo!()
    }

    /// todo
    pub fn payload(&self) -> &ResponsePayload {
        todo!()
    }
}
