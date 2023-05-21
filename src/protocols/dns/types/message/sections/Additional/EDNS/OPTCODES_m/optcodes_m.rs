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

pub enum OPTCODES {
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
    EXTERROR = 15,
    ECT = 16,
    EST = 17,
    UMBRELLA_IDENT = 20292,
    DEVICE_ID = 26946
}

impl OPTCODES {
    pub fn code(&self) -> u16 {
        return match self {
            OPTCODES::LLQ => OPTCODES::LLQ as u16,
            OPTCODES::UL => OPTCODES::UL as u16,
            OPTCODES::NSID => OPTCODES::NSID as u16,
            OPTCODES::OWNER => OPTCODES::OWNER as u16,
            OPTCODES::DAU => OPTCODES::DAU as u16,
            OPTCODES::DHU => OPTCODES::DHU as u16,
            OPTCODES::N3U => OPTCODES::N3U as u16,
            OPTCODES::ECS => OPTCODES::ECS as u16,
            OPTCODES::EXPIRE => OPTCODES::EXPIRE as u16,
            OPTCODES::COOKIE => OPTCODES::COOKIE as u16,
            OPTCODES::ETK => OPTCODES::ETK as u16,
            OPTCODES::PADDING => OPTCODES::PADDING as u16,
            OPTCODES::CHAIN => OPTCODES::CHAIN as u16,
            OPTCODES::EKT => OPTCODES::EKT as u16,
            OPTCODES::EXTERROR => OPTCODES::EXTERROR as u16,
            OPTCODES::ECT => OPTCODES::ECT as u16,
            OPTCODES::EST => OPTCODES::EST as u16,
            OPTCODES::UMBRELLA_IDENT => OPTCODES::UMBRELLA_IDENT as u16,
            OPTCODES::DEVICE_ID => OPTCODES::DEVICE_ID as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            OPTCODES::LLQ => "Long-Lived Queries",
            OPTCODES::UL => "Update Lease",
            OPTCODES::NSID => "Name Server Identifier",
            OPTCODES::OWNER => "Wake-On-LAN (aka OWNER)",
            OPTCODES::DAU => "DNSSEC Algorithm Understood",
            OPTCODES::DHU => "DS Hash Understood",
            OPTCODES::N3U => "NSEC3 Hash Understood",
            OPTCODES::ECS => "EDNS Client Subnet",
            OPTCODES::EXPIRE => "Time Zone expiration",
            OPTCODES::COOKIE => "Enhanced Security with DNS Cookie",
            OPTCODES::ETK => "EDNS TCP Keepalive",
            OPTCODES::PADDING => "Enhanced Security of Encryption with Padding",
            OPTCODES::CHAIN => "CHAIN Query Request",
            OPTCODES::EKT => "Extended Key Tag",
            OPTCODES::EXTERROR => "Extended DNS Errors",
            OPTCODES::ECT => "EDNS Client Tag",
            OPTCODES::EST => "EDNS Server Tag",
            OPTCODES::UMBRELLA_IDENT => "Umbrella Identifier",
            OPTCODES::DEVICE_ID => "Device ID"
        }
    }
}
