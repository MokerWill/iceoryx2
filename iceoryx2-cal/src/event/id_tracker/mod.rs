// Copyright (c) 2024 Contributors to the Eclipse Foundation
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

pub mod bitset;
pub mod queue;

use super::TriggerId;
use iceoryx2_bb_elementary::relocatable_container::RelocatableContainer;

pub trait IdTracker: RelocatableContainer {
    fn trigger_id_max(&self) -> TriggerId;
    fn set(&self, id: TriggerId);
    fn reset_next(&self) -> TriggerId;
    fn reset_all<F: FnMut(TriggerId)>(&self, callback: F);
}
