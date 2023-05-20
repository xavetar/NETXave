/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

pub enum OPTION {
    LLQ = 1,
    UL = 2,
    NSID = 3,
    OWNER = 4,
    DAU = 5,
    DHU = 6,
    N3U = 7,
    ECS = 8,
    EXPIRE = 9,
    COOKIE = 10,
    ETK = 11,
    PADDING = 12,
    CHAIN = 13,
    EKT = 14,
    EXTENDED_ERROR = 15,
    ECT = 16,
    EST = 17,
    UMBRELLA_IDENT = 20292,
    DEVICE_ID = 26946
}

impl OPTION {
    pub fn code(&self) -> u16 {
        return match self {
            OPTION::LLQ => OPTION::LLQ as u16,
            OPTION::UL => OPTION::LLQ as u16,
            OPTION::NSID => OPTION::LLQ as u16,
            OPTION::OWNER => OPTION::OWNER as u16,
            OPTION::DAU => OPTION::LLQ as u16,
            OPTION::DHU => OPTION::LLQ as u16,
            OPTION::N3U => OPTION::LLQ as u16,
            OPTION::ECS => OPTION::LLQ as u16,
            OPTION::EXPIRE => OPTION::LLQ as u16,
            OPTION::COOKIE => OPTION::LLQ as u16,
            OPTION::ETK => OPTION::LLQ as u16,
            OPTION::PADDING => OPTION::LLQ as u16,
            OPTION::CHAIN => OPTION::LLQ as u16,
            OPTION::EKT => OPTION::LLQ as u16,
            OPTION::EXTENDED_ERROR => OPTION::LLQ as u16,
            OPTION::ECT => OPTION::LLQ as u16,
            OPTION::EST => OPTION::LLQ as u16,
            OPTION::UMBRELLA_IDENT => OPTION::LLQ as u16,
            OPTION::DEVICE_ID => OPTION::LLQ as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            OPTION::LLQ => "Long-Lived Queries",
            OPTION::UL => "Update Lease",
            OPTION::NSID => "Name Server Identifier",
            OPTION::OWNER => "Wake-On-LAN (aka OWNER)",
            OPTION::DAU => "DNSSEC Algorithm Understood",
            OPTION::DHU => "DS Hash Understood",
            OPTION::N3U => "NSEC3 Hash Understood",
            OPTION::ECS => "EDNS Client Subnet",
            OPTION::EXPIRE => "Time Zone expiration",
            OPTION::COOKIE => "Enhanced Security with DNS Cookie",
            OPTION::ETK => "EDNS TCP Keepalive",
            OPTION::PADDING => "Enhanced Security of Encryption with Padding",
            OPTION::CHAIN => "CHAIN Query Request",
            OPTION::EKT => "Extended Key Tag",
            OPTION::EXTENDED_ERROR => "Extended DNS Errors",
            OPTION::ECT => "EDNS Client Tag",
            OPTION::EST => "EDNS Server Tag",
            OPTION::UMBRELLA_IDENT => "Umbrella Identifier",
            OPTION::DEVICE_ID => "Device ID"
        }
    }
}
