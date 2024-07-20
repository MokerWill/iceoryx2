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

#ifndef IOX2_SERVICE_NAME_HPP
#define IOX2_SERVICE_NAME_HPP

#include "iox/expected.hpp"
#include "iox/string.hpp"
#include "iox2/iceoryx2_settings.hpp"
#include "iox2/internal/iceoryx2.hpp"
#include "semantic_string.hpp"

namespace iox2 {

/// The name of a [`Service`].
class ServiceName {
  public:
    ServiceName(ServiceName&&) noexcept;
    auto operator=(ServiceName&&) noexcept -> ServiceName&;
    ServiceName(const ServiceName&);
    auto operator=(const ServiceName&) -> ServiceName&;
    ~ServiceName();

    /// Creates a new [`ServiceName`]. The name is not allowed to be empty.
    static auto create(const char* value) -> iox::expected<ServiceName, SemanticStringError>;

    /// Returns a [`iox::string`] containing the [`ServiceName`].
    auto to_string() const -> iox::string<SERVICE_NAME_LENGTH>;

  private:
    explicit ServiceName(iox2_service_name_h handle);
    void drop() noexcept;

    iox2_service_name_h m_handle;
};

} // namespace iox2

#endif