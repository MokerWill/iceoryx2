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

#[generic_tests::define]
mod client {
    use std::ops::Deref;
    use std::sync::atomic::Ordering;
    use std::sync::Barrier;
    use std::time::Duration;

    use iceoryx2::port::client::RequestSendError;
    use iceoryx2::port::LoanError;
    use iceoryx2::prelude::*;
    use iceoryx2::service::port_factory::request_response::PortFactory;
    use iceoryx2::testing::*;
    use iceoryx2_bb_testing::assert_that;
    use iceoryx2_bb_testing::lifetime_tracker::LifetimeTracker;
    use iceoryx2_bb_testing::watchdog::Watchdog;
    use iceoryx2_pal_concurrency_sync::iox_atomic::IoxAtomicBool;

    const TIMEOUT: Duration = Duration::from_millis(50);

    fn create_node<Sut: Service>() -> Node<Sut> {
        let config = generate_isolated_config();
        NodeBuilder::new().config(&config).create::<Sut>().unwrap()
    }

    fn create_node_and_service<Sut: Service>() -> (Node<Sut>, PortFactory<Sut, u64, (), u64, ()>) {
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .create()
            .unwrap();

        (node, service)
    }

    #[test]
    fn loan_and_send_request_works<Sut: Service>() {
        const PAYLOAD: u64 = 2873421;
        let (_node, service) = create_node_and_service::<Sut>();

        let sut = service.client_builder().create().unwrap();
        let mut request = sut.loan().unwrap();
        *request = PAYLOAD;

        let pending_response = request.send();
        assert_that!(pending_response, is_ok);
        let pending_response = pending_response.unwrap();
        assert_that!(*pending_response.payload(), eq PAYLOAD);
    }

    #[test]
    fn can_loan_at_most_max_supported_amount_of_requests<Sut: Service>() {
        const MAX_LOANED_REQUESTS: usize = 29;
        const ITERATIONS: usize = 3;
        let (_node, service) = create_node_and_service::<Sut>();

        let sut = service
            .client_builder()
            .max_loaned_requests(MAX_LOANED_REQUESTS)
            .create()
            .unwrap();

        for _ in 0..ITERATIONS {
            let mut requests = vec![];
            for _ in 0..MAX_LOANED_REQUESTS {
                let request = sut.loan_uninit();
                assert_that!(request, is_ok);
                requests.push(request);
            }
            let request = sut.loan_uninit();
            assert_that!(request.err(), eq Some(LoanError::ExceedsMaxLoans));
        }
    }

    #[test]
    fn can_loan_at_most_max_supported_amount_of_requests_when_holding_max_pending_responses<
        Sut: Service,
    >() {
        const MAX_LOANED_REQUESTS: usize = 29;
        const MAX_PENDING_RESPONSES: usize = 7;
        const ITERATIONS: usize = 3;

        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .max_active_requests_per_client(MAX_PENDING_RESPONSES)
            .create()
            .unwrap();

        let sut = service
            .client_builder()
            .max_loaned_requests(MAX_LOANED_REQUESTS)
            .create()
            .unwrap();

        for _ in 0..ITERATIONS {
            let mut pending_responses = vec![];
            for _ in 0..MAX_PENDING_RESPONSES {
                pending_responses.push(sut.send_copy(123).unwrap());
            }

            let mut requests = vec![];
            for _ in 0..MAX_LOANED_REQUESTS {
                let request = sut.loan_uninit();
                assert_that!(request, is_ok);
                requests.push(request);
            }
            let request = sut.loan_uninit();
            assert_that!(request.err(), eq Some(LoanError::ExceedsMaxLoans));
        }
    }

    #[test]
    fn unable_to_deliver_strategy_block_blocks_when_server_buffer_is_full<Sut: Service>() {
        let _watchdog = Watchdog::new();
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .enable_safe_overflow_for_requests(false)
            .max_active_requests_per_client(1)
            .create()
            .unwrap();
        let server = service.server_builder().create().unwrap();
        let has_sent_request = IoxAtomicBool::new(false);
        let barrier = Barrier::new(2);

        std::thread::scope(|s| {
            s.spawn(|| {
                let sut = service
                    .client_builder()
                    .unable_to_deliver_strategy(UnableToDeliverStrategy::Block)
                    .create()
                    .unwrap();

                let request = sut.send_copy(123);
                assert_that!(request, is_ok);
                drop(request);
                barrier.wait();

                let request = sut.send_copy(123);
                has_sent_request.store(true, Ordering::Relaxed);
                assert_that!(request, is_ok);
            });

            barrier.wait();
            std::thread::sleep(TIMEOUT);
            assert_that!(has_sent_request.load(Ordering::Relaxed), eq false);
            let data = server.receive();
            assert_that!(data, is_ok);
            assert_that!(|| has_sent_request.load(Ordering::Relaxed), block_until true);
        });
    }

    #[test]
    fn unable_to_deliver_strategy_discard_discards_sample<Sut: Service>() {
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .enable_safe_overflow_for_requests(false)
            .max_active_requests_per_client(1)
            .create()
            .unwrap();
        let server = service.server_builder().create().unwrap();

        let sut = service
            .client_builder()
            .unable_to_deliver_strategy(UnableToDeliverStrategy::DiscardSample)
            .create()
            .unwrap();

        let request = sut.send_copy(123);
        assert_that!(request, is_ok);
        let _request = sut.send_copy(456);

        let data = server.receive();
        assert_that!(data, is_ok);
        let data = data.ok().unwrap();
        assert_that!(data, is_some);
        let data = data.unwrap();
        assert_that!(*data, eq 123);
    }

    #[test]
    fn loan_request_is_initialized_with_default_value<Sut: Service>() {
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<LifetimeTracker, u64>()
            .create()
            .unwrap();

        let sut = service.client_builder().create().unwrap();

        let tracker = LifetimeTracker::start_tracking();
        let request = sut.loan();
        assert_that!(tracker.number_of_living_instances(), eq 1);

        drop(request);
        assert_that!(tracker.number_of_living_instances(), eq 0);
    }

    #[test]
    fn loan_uninit_request_is_not_initialized<Sut: Service>() {
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<LifetimeTracker, u64>()
            .create()
            .unwrap();

        let sut = service.client_builder().create().unwrap();

        let tracker = LifetimeTracker::start_tracking();
        let request = sut.loan_uninit();
        assert_that!(tracker.number_of_living_instances(), eq 0);

        drop(request);
        assert_that!(tracker.number_of_living_instances(), eq 0);
    }

    #[test]
    fn sending_requests_reduces_loan_counter<Sut: Service>() {
        let (_node, service) = create_node_and_service::<Sut>();

        let sut = service
            .client_builder()
            .max_loaned_requests(1)
            .create()
            .unwrap();

        let request = sut.loan().unwrap();

        let request2 = sut.loan();
        assert_that!(request2.err(), eq Some(LoanError::ExceedsMaxLoans));

        request.send().unwrap();

        let request2 = sut.loan();
        assert_that!(request2, is_ok);
    }

    #[test]
    fn dropping_requests_reduces_loan_counter<Sut: Service>() {
        let (_node, service) = create_node_and_service::<Sut>();

        let sut = service
            .client_builder()
            .max_loaned_requests(1)
            .create()
            .unwrap();

        let request = sut.loan().unwrap();

        let request2 = sut.loan();
        assert_that!(request2.err(), eq Some(LoanError::ExceedsMaxLoans));

        drop(request);

        let request2 = sut.loan();
        assert_that!(request2, is_ok);
    }

    #[test]
    fn request_is_correctly_aligned<Sut: Service>() {
        const MAX_LOAN: usize = 9;
        const ALIGNMENT: usize = 512;
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .request_payload_alignment(Alignment::new(ALIGNMENT).unwrap())
            .create()
            .unwrap();

        let sut = service
            .client_builder()
            .max_loaned_requests(MAX_LOAN)
            .create()
            .unwrap();

        let mut requests = vec![];

        for _ in 0..MAX_LOAN {
            let request = sut.loan().unwrap();
            let request_addr = (request.deref() as *const u64) as usize;
            assert_that!(request_addr % ALIGNMENT, eq 0);
            requests.push(request);
        }
    }

    #[test]
    fn send_request_fails_when_already_active_requests_is_at_max<Sut: Service>() {
        const MAX_ACTIVE_REQUESTS: usize = 9;
        const ITERATIONS: usize = 5;
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .max_active_requests_per_client(MAX_ACTIVE_REQUESTS)
            .create()
            .unwrap();

        let sut = service.client_builder().create().unwrap();

        for _ in 0..ITERATIONS {
            let mut requests = vec![];

            for _ in 0..MAX_ACTIVE_REQUESTS {
                requests.push(sut.send_copy(123).unwrap());
            }

            assert_that!(sut.send_copy(123).err(), eq Some(RequestSendError::ExceedsMaxActiveRequests));

            let request = sut.loan().unwrap();
            assert_that!(request.send().err(), eq Some(RequestSendError::ExceedsMaxActiveRequests));
        }
    }

    fn client_never_goes_out_of_memory_impl<Sut: Service>(
        max_active_requests_per_client: usize,
        max_servers: usize,
        max_loaned_requests: usize,
    ) {
        const ITERATIONS: usize = 5;

        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .max_clients(1)
            .max_servers(max_servers)
            .max_active_requests_per_client(max_active_requests_per_client)
            .create()
            .unwrap();

        let sut = service
            .client_builder()
            .max_loaned_requests(max_loaned_requests)
            .create()
            .unwrap();

        let mut servers = vec![];
        for _ in 0..max_servers {
            let sut_server = service.server_builder().create().unwrap();
            servers.push(sut_server);
        }

        for _ in 0..ITERATIONS {
            // max out pending responses
            let mut pending_responses = vec![];
            let mut active_requests = vec![];
            for _ in 0..max_active_requests_per_client {
                pending_responses.push(sut.send_copy(123).unwrap());

                for server in &servers {
                    let active_request = server.receive().unwrap();
                    assert_that!(active_request, is_some);
                    active_requests.push(active_request);
                }
            }

            pending_responses.clear();
            // max out request buffer on server side
            for _ in 0..max_active_requests_per_client {
                pending_responses.push(sut.send_copy(456).unwrap());
            }

            // max out loaned requests
            let mut loaned_requests = vec![];
            for _ in 0..max_loaned_requests {
                let request = sut.loan();
                assert_that!(request, is_ok);
                loaned_requests.push(request);
            }

            let request = sut.loan();
            assert_that!(request.err(), eq Some(LoanError::ExceedsMaxLoans));

            // cleanup
            pending_responses.clear();
            loaned_requests.clear();
            for server in &servers {
                while let Ok(Some(_)) = server.receive() {}
            }
        }
    }

    #[test]
    fn client_never_goes_out_of_memory_with_huge_max_pending_responses<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 100;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 1;

        client_never_goes_out_of_memory_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn client_never_goes_out_of_memory_with_huge_max_servers<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 100;
        const MAX_LOANED_REQUESTS: usize = 1;

        client_never_goes_out_of_memory_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn client_never_goes_out_of_memory_with_huge_max_loaned_requests<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 100;

        client_never_goes_out_of_memory_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn client_never_goes_out_of_memory_with_smallest_possible_values<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 1;

        client_never_goes_out_of_memory_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn client_never_goes_out_of_memory_with_huge_values<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 12;
        const MAX_SERVERS: usize = 15;
        const MAX_LOANED_REQUESTS: usize = 19;

        client_never_goes_out_of_memory_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    fn retrieve_channel_capacity_is_never_exceeded_impl<Sut: Service>(
        max_active_requests_per_client: usize,
        max_servers: usize,
        max_loaned_requests: usize,
    ) {
        const ITERATIONS: usize = 5;

        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .max_clients(1)
            .max_servers(max_servers)
            .max_active_requests_per_client(max_active_requests_per_client)
            .create()
            .unwrap();

        let sut = service
            .client_builder()
            .max_loaned_requests(max_loaned_requests)
            .create()
            .unwrap();

        let mut servers = vec![];
        for _ in 0..max_servers {
            let sut_server = service.server_builder().create().unwrap();
            servers.push(sut_server);
        }

        for _ in 0..ITERATIONS {
            // max out pending responses
            let mut pending_responses = vec![];
            let mut active_requests = vec![];
            for _ in 0..max_active_requests_per_client {
                pending_responses.push(sut.send_copy(123).unwrap());

                for server in &servers {
                    let active_request = server.receive().unwrap();
                    assert_that!(active_request, is_some);
                    active_requests.push(active_request);
                }
            }

            pending_responses.clear();
            // max out request buffer on server side
            for _ in 0..max_active_requests_per_client {
                pending_responses.push(sut.send_copy(456).unwrap());
            }

            // receive and return everything
            active_requests.clear();
            for server in &servers {
                while let Ok(Some(_)) = server.receive() {}
            }
        }
    }

    #[test]
    fn retrieve_channel_capacity_is_never_exceeded_with_huge_active_requests<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 100;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 1;

        retrieve_channel_capacity_is_never_exceeded_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn retrieve_channel_capacity_is_never_exceeded_with_huge_max_servers<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 100;
        const MAX_LOANED_REQUESTS: usize = 1;

        retrieve_channel_capacity_is_never_exceeded_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn retrieve_channel_capacity_is_never_exceeded_with_huge_max_loaned_requests<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 100;

        retrieve_channel_capacity_is_never_exceeded_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn retrieve_channel_capacity_is_never_exceeded_with_huge_values<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 23;
        const MAX_SERVERS: usize = 12;
        const MAX_LOANED_REQUESTS: usize = 10;

        retrieve_channel_capacity_is_never_exceeded_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn retrieve_channel_capacity_is_never_exceeded_with_smallest_possible_values<Sut: Service>() {
        const MAX_ACTIVE_REQUEST_PER_CLIENT: usize = 1;
        const MAX_SERVERS: usize = 1;
        const MAX_LOANED_REQUESTS: usize = 1;

        retrieve_channel_capacity_is_never_exceeded_impl::<Sut>(
            MAX_ACTIVE_REQUEST_PER_CLIENT,
            MAX_SERVERS,
            MAX_LOANED_REQUESTS,
        );
    }

    #[test]
    fn reclaims_all_requests_after_disconnect<Sut: Service>() {
        const MAX_ACTIVE_REQUESTS: usize = 4;
        const ITERATIONS: usize = 5;
        const MAX_SERVER: usize = 4;
        let service_name = generate_service_name();
        let node = create_node::<Sut>();
        let service = node
            .service_builder(&service_name)
            .request_response::<u64, u64>()
            .max_active_requests_per_client(MAX_ACTIVE_REQUESTS)
            .max_servers(MAX_SERVER)
            .create()
            .unwrap();

        let sut = service.client_builder().create().unwrap();

        for n in 0..MAX_SERVER {
            for _ in 0..ITERATIONS {
                let mut requests = vec![];
                let mut servers = vec![];
                for _ in 0..n {
                    servers.push(service.client_builder().create().unwrap());
                }

                for _ in 0..MAX_ACTIVE_REQUESTS {
                    requests.push(sut.send_copy(123).unwrap());
                }

                assert_that!(sut.send_copy(123).err(), eq Some(RequestSendError::ExceedsMaxActiveRequests));
            }
        }
    }

    #[instantiate_tests(<iceoryx2::service::ipc::Service>)]
    mod ipc {}

    #[instantiate_tests(<iceoryx2::service::local::Service>)]
    mod local {}
}
