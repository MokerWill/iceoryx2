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

#include "iox2/unique_port_id.hpp"
#include "iox/assertions_addendum.hpp"

namespace iox2 {
UniquePublisherId::UniquePublisherId(UniquePublisherId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniquePublisherId::operator=(UniquePublisherId&& rhs) noexcept -> UniquePublisherId& {
    if (this != &rhs) {
        drop();
        m_handle = std::move(rhs.m_handle);
        rhs.m_handle = nullptr;
    }

    return *this;
}

UniquePublisherId::~UniquePublisherId() {
    drop();
}

auto operator==(const UniquePublisherId& lhs, const UniquePublisherId& rhs) -> bool {
    return iox2_unique_publisher_id_eq(&lhs.m_handle, &rhs.m_handle);
}

auto operator<(const UniquePublisherId& lhs, const UniquePublisherId& rhs) -> bool {
    return iox2_unique_publisher_id_less(&lhs.m_handle, &rhs.m_handle);
}

UniquePublisherId::UniquePublisherId(iox2_unique_publisher_id_h handle)
    : m_handle { handle } {
}

auto UniquePublisherId::bytes() const -> const iox::optional<RawIdType>& {
    if (!m_raw_id.has_value() && m_handle != nullptr) {
        RawIdType bytes { UNIQUE_PORT_ID_LENGTH, 0 };
        iox2_unique_publisher_id_value(m_handle, bytes.data(), bytes.size());
        m_raw_id.emplace(std::move(bytes));
    }
    return m_raw_id;
};

void UniquePublisherId::drop() {
    if (m_handle != nullptr) {
        iox2_unique_publisher_id_drop(m_handle);
        m_handle = nullptr;
    }
}

UniqueSubscriberId::UniqueSubscriberId(UniqueSubscriberId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniqueSubscriberId::operator=(UniqueSubscriberId&& rhs) noexcept -> UniqueSubscriberId& {
    if (this != &rhs) {
        drop();
        m_handle = std::move(rhs.m_handle);
        rhs.m_handle = nullptr;
    }

    return *this;
}

UniqueSubscriberId::~UniqueSubscriberId() {
    drop();
}

auto operator==(const UniqueSubscriberId& lhs, const UniqueSubscriberId& rhs) -> bool {
    return iox2_unique_subscriber_id_eq(&lhs.m_handle, &rhs.m_handle);
}

auto operator<(const UniqueSubscriberId& lhs, const UniqueSubscriberId& rhs) -> bool {
    return iox2_unique_subscriber_id_less(&lhs.m_handle, &rhs.m_handle);
}

UniqueSubscriberId::UniqueSubscriberId(iox2_unique_subscriber_id_h handle)

    : m_handle { handle } {
}

auto UniqueSubscriberId::bytes() const -> const iox::optional<RawIdType>& {
    if (!m_raw_id.has_value() && m_handle != nullptr) {
        RawIdType bytes { UNIQUE_PORT_ID_LENGTH, 0 };
        iox2_unique_subscriber_id_value(m_handle, bytes.data(), bytes.size());
        m_raw_id.emplace(std::move(bytes));
    }
    return m_raw_id;
};

void UniqueSubscriberId::drop() {
    if (m_handle != nullptr) {
        iox2_unique_subscriber_id_drop(m_handle);
        m_handle = nullptr;
    }
}

UniqueNotifierId::UniqueNotifierId(UniqueNotifierId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniqueNotifierId::operator=(UniqueNotifierId&& rhs) noexcept -> UniqueNotifierId& {
    if (this != &rhs) {
        drop();
        m_handle = std::move(rhs.m_handle);
        rhs.m_handle = nullptr;
    }

    return *this;
}

UniqueNotifierId::~UniqueNotifierId() {
    drop();
}

auto operator==(const UniqueNotifierId& lhs, const UniqueNotifierId& rhs) -> bool {
    return iox2_unique_notifier_id_eq(&lhs.m_handle, &rhs.m_handle);
}

auto operator<(const UniqueNotifierId& lhs, const UniqueNotifierId& rhs) -> bool {
    return iox2_unique_notifier_id_less(&lhs.m_handle, &rhs.m_handle);
}

UniqueNotifierId::UniqueNotifierId(iox2_unique_notifier_id_h handle)
    : m_handle { handle } {
}

auto UniqueNotifierId::bytes() const -> const iox::optional<RawIdType>& {
    if (!m_raw_id.has_value() && m_handle != nullptr) {
        RawIdType bytes { UNIQUE_PORT_ID_LENGTH, 0 };
        iox2_unique_notifier_id_value(m_handle, bytes.data(), bytes.size());
        m_raw_id.emplace(std::move(bytes));
    }
    return m_raw_id;
};

void UniqueNotifierId::drop() {
    if (m_handle != nullptr) {
        iox2_unique_notifier_id_drop(m_handle);
        m_handle = nullptr;
    }
}

UniqueListenerId::UniqueListenerId(UniqueListenerId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniqueListenerId::operator=(UniqueListenerId&& rhs) noexcept -> UniqueListenerId& {
    if (this != &rhs) {
        drop();
        m_handle = std::move(rhs.m_handle);
        rhs.m_handle = nullptr;
    }

    return *this;
}

UniqueListenerId::~UniqueListenerId() {
    drop();
}

auto operator==(const UniqueListenerId& lhs, const UniqueListenerId& rhs) -> bool {
    return iox2_unique_listener_id_eq(&lhs.m_handle, &rhs.m_handle);
}

auto operator<(const UniqueListenerId& lhs, const UniqueListenerId& rhs) -> bool {
    return iox2_unique_listener_id_less(&lhs.m_handle, &rhs.m_handle);
}

UniqueListenerId::UniqueListenerId(iox2_unique_listener_id_h handle)
    : m_handle { handle } {
}

auto UniqueListenerId::bytes() const -> const iox::optional<RawIdType>& {
    if (!m_raw_id.has_value() && m_handle != nullptr) {
        RawIdType bytes { UNIQUE_PORT_ID_LENGTH, 0 };
        iox2_unique_listener_id_value(m_handle, bytes.data(), bytes.size());
        m_raw_id.emplace(std::move(bytes));
    }
    return m_raw_id;
};

void UniqueListenerId::drop() {
    if (m_handle != nullptr) {
        iox2_unique_listener_id_drop(m_handle);
        m_handle = nullptr;
    }
}

UniqueClientId::UniqueClientId(UniqueClientId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniqueClientId::operator=([[maybe_unused]] UniqueClientId&& rhs) noexcept -> UniqueClientId& {
    IOX_TODO();
}

UniqueClientId::~UniqueClientId() {
    drop();
}

auto operator==([[maybe_unused]] const UniqueClientId& lhs, [[maybe_unused]] const UniqueClientId& rhs) -> bool {
    IOX_TODO();
}

auto operator<([[maybe_unused]] const UniqueClientId& lhs, [[maybe_unused]] const UniqueClientId& rhs) -> bool {
    IOX_TODO();
}

UniqueClientId::UniqueClientId(/* iox2_unique_client_id_h handle*/) {
    IOX_TODO();
}

auto UniqueClientId::bytes() const -> const iox::optional<RawIdType>& {
    IOX_TODO();
};

void UniqueClientId::drop() {
    IOX_TODO();
}

UniqueServerId::UniqueServerId(UniqueServerId&& rhs) noexcept {
    *this = std::move(rhs);
}

auto UniqueServerId::operator=([[maybe_unused]] UniqueServerId&& rhs) noexcept -> UniqueServerId& {
    IOX_TODO();
}

UniqueServerId::~UniqueServerId() {
    drop();
}

auto operator==([[maybe_unused]] const UniqueServerId& lhs, [[maybe_unused]] const UniqueServerId& rhs) -> bool {
    IOX_TODO();
}

auto operator<([[maybe_unused]] const UniqueServerId& lhs, [[maybe_unused]] const UniqueServerId& rhs) -> bool {
    IOX_TODO();
}

UniqueServerId::UniqueServerId(/*iox2_unique_server_id_h handle*/) {
    IOX_TODO();
}

auto UniqueServerId::bytes() const -> const iox::optional<RawIdType>& {
    IOX_TODO();
};

void UniqueServerId::drop() {
    IOX_TODO();
}
} // namespace iox2
