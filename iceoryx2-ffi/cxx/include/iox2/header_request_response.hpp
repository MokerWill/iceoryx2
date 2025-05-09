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

#ifndef IOX2_HEADER_REQUEST_RESPONSE_HPP
#define IOX2_HEADER_REQUEST_RESPONSE_HPP

#include "iox2/unique_port_id.hpp"

namespace iox2 {
/// Request header used by [`MessagingPattern::RequestResponse`]
class RequestHeaderRequestResponse {
  public:
    RequestHeaderRequestResponse(const RequestHeaderRequestResponse&) = delete;
    RequestHeaderRequestResponse(RequestHeaderRequestResponse&& rhs) noexcept;
    auto operator=(const RequestHeaderRequestResponse&) -> RequestHeaderRequestResponse& = delete;
    auto operator=(RequestHeaderRequestResponse&& rhs) noexcept -> RequestHeaderRequestResponse&;
    ~RequestHeaderRequestResponse();

    /// Returns the [`UniqueClientId`] of the source [`Client`].
    auto client_port_id() -> UniqueClientId;

  private:
    explicit RequestHeaderRequestResponse(/*iox2_request_header_h handle*/);
    void drop();

    // iox2_request_header_h m_handle = nullptr;
};

/// Response header used by [`MessagingPattern::RequestResponse`]
class ResponseHeaderRequestResponse {
  public:
    ResponseHeaderRequestResponse(const ResponseHeaderRequestResponse&) = delete;
    ResponseHeaderRequestResponse(ResponseHeaderRequestResponse&& rhs) noexcept;
    auto operator=(const ResponseHeaderRequestResponse&) -> ResponseHeaderRequestResponse& = delete;
    auto operator=(ResponseHeaderRequestResponse&& rhs) noexcept -> ResponseHeaderRequestResponse&;
    ~ResponseHeaderRequestResponse();

    /// Returns the [`UniqueServerId`] of the source [`Server`].
    auto server_port_id() -> UniqueServerId;

  private:
    explicit ResponseHeaderRequestResponse(/*iox2_response_header_h handle*/);
    void drop();

    // iox2_response_header_h m_handle = nullptr;
};
} // namespace iox2
#endif
