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

pub enum OPTION_CODE {
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
    EDE = 15,
    ECT = 16,
    EST = 17,
    UMBRELLA_IDENT = 20292,
    DEVICE_ID = 26946
}

impl OPTION_CODE {
    pub fn code(&self) -> u16 {
        return match self {
            OPTION_CODE::LLQ => OPTION_CODE::LLQ as u16,
            OPTION_CODE::UL => OPTION_CODE::UL as u16,
            OPTION_CODE::NSID => OPTION_CODE::NSID as u16,
            OPTION_CODE::OWNER => OPTION_CODE::OWNER as u16,
            OPTION_CODE::DAU => OPTION_CODE::DAU as u16,
            OPTION_CODE::DHU => OPTION_CODE::DHU as u16,
            OPTION_CODE::N3U => OPTION_CODE::N3U as u16,
            OPTION_CODE::ECS => OPTION_CODE::ECS as u16,
            OPTION_CODE::EXPIRE => OPTION_CODE::EXPIRE as u16,
            OPTION_CODE::COOKIE => OPTION_CODE::COOKIE as u16,
            OPTION_CODE::ETK => OPTION_CODE::ETK as u16,
            OPTION_CODE::PADDING => OPTION_CODE::PADDING as u16,
            OPTION_CODE::CHAIN => OPTION_CODE::CHAIN as u16,
            OPTION_CODE::EKT => OPTION_CODE::EKT as u16,
            OPTION_CODE::EDE => OPTION_CODE::EDE as u16,
            OPTION_CODE::ECT => OPTION_CODE::ECT as u16,
            OPTION_CODE::EST => OPTION_CODE::EST as u16,
            OPTION_CODE::UMBRELLA_IDENT => OPTION_CODE::UMBRELLA_IDENT as u16,
            OPTION_CODE::DEVICE_ID => OPTION_CODE::DEVICE_ID as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            OPTION_CODE::LLQ => "Long-Lived Queries",
            OPTION_CODE::UL => "Update Lease",
            OPTION_CODE::NSID => "Name Server Identifier",
            OPTION_CODE::OWNER => "Wake-On-LAN (aka OWNER)",
            OPTION_CODE::DAU => "DNSSEC Algorithm Understood",
            OPTION_CODE::DHU => "DS Hash Understood",
            OPTION_CODE::N3U => "NSEC3 Hash Understood",
            OPTION_CODE::ECS => "EDNS Client Subnet",
            OPTION_CODE::EXPIRE => "Time Zone expiration",
            OPTION_CODE::COOKIE => "Enhanced Security with DNS Cookie",
            OPTION_CODE::ETK => "EDNS TCP Keepalive",
            OPTION_CODE::PADDING => "Enhanced Security of Encryption with Padding",
            OPTION_CODE::CHAIN => "CHAIN Query Request",
            OPTION_CODE::EKT => "Extended Key Tag",
            OPTION_CODE::EDE => "Extended DNS Error",
            OPTION_CODE::ECT => "EDNS Client Tag",
            OPTION_CODE::EST => "EDNS Server Tag",
            OPTION_CODE::UMBRELLA_IDENT => "Umbrella Identifier",
            OPTION_CODE::DEVICE_ID => "Device ID"
        }
    }
}
