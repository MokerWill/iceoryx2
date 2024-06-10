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

use core::time::Duration;
use iceoryx2::{prelude::*, service::static_config::Property};

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new("Service/With/Properties")?;

    let service = zero_copy::Service::new(&service_name)
        // define a set of properties that are static for the lifetime
        // of the service
        .add_property("dds_service_mapping", "my_funky_service_name")
        .add_property("tcp_serialization_format", "cdr")
        .add_property("someip_service_mapping", "1/2/3")
        .add_property("camera_resolution", "1920x1080")
        //
        .publish_subscribe::<u64>()
        .create()?;

    let publisher = service.publisher().create()?;

    println!("defined service properties: {:?}", service.properties());
    for property in service.properties().iter() {
        println!("{} = {}", property.key(), property.value());
    }

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        let sample = publisher.loan_uninit()?;
        let sample = sample.write_payload(0);
        sample.send()?;
    }

    println!("exit");

    Ok(())
}
