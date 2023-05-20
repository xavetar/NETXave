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

use crate::data_types::{U4};
use crate::protocols::dns::client::types::message::Additional::OPT_m::constants::{RESERVED, RESERVED_FUTURE_EXPANSION_USE, RESERVED_LOCAL_EXPERIMENTAL_USE};
use crate::protocols::dns::client::types::message::Answer::TYPE::OPT;

use super::{OPTION};
use super::{OPTIONInfo};
use super::{UNASSIGNED};


pub trait OPTIONConversion {
    fn encode(name: &str) -> Result<OPTIONInfo, String>;
    fn decode(dec: &u16) -> Result<OPTIONInfo, String>;
}

impl OPTIONConversion for OPTION {
    fn encode(name: &str) -> Result<OPTIONInfo, String> {
        return match name {
            "LLQ" => Ok(
                OPTIONInfo::new(OPTION::LLQ.name(),
                                OPTION::LLQ.code())
            ),
            "UL" => Ok(
                OPTIONInfo::new(OPTION::UL.name(),
                                OPTION::UL.code())
            ),
            "NSID" => Ok(
                OPTIONInfo::new(OPTION::NSID.name(),
                                OPTION::NSID.code())
            ),
            "OWNER" => Ok(
                OPTIONInfo::new(OPTION::OWNER.name(),
                                OPTION::OWNER.code())
            ),
            "DAU" => Ok(
                OPTIONInfo::new(OPTION::DAU.name(),
                                OPTION::DAU.code())
            ),
            "DHU" => Ok(
                OPTIONInfo::new(OPTION::DHU.name(),
                                OPTION::DHU.code())
            ),
            "N3U" => Ok(
                OPTIONInfo::new(OPTION::N3U.name(),
                                OPTION::N3U.code())
            ),
            "ECS" => Ok(
                OPTIONInfo::new(OPTION::ECS.name(),
                                OPTION::ECS.code())
            ),
            "EXPIRE" => Ok(
                OPTIONInfo::new(OPTION::EXPIRE.name(),
                                OPTION::EXPIRE.code())
            ),
            "COOKIE" => Ok(
                OPTIONInfo::new(OPTION::COOKIE.name(),
                                OPTION::COOKIE.code())
            ),
            "ETK" => Ok(
                OPTIONInfo::new(OPTION::ETK.name(),
                                OPTION::ETK.code())
            ),
            "PADDING" => Ok(
                OPTIONInfo::new(OPTION::PADDING.name(),
                                OPTION::PADDING.code())
            ),
            "CHAIN" => Ok(
                OPTIONInfo::new(OPTION::CHAIN.name(),
                                OPTION::CHAIN.code())
            ),
            "EKT" => Ok(
                OPTIONInfo::new(OPTION::EKT.name(),
                                OPTION::EKT.code())
            ),
            "EXTENDED_ERROR" => Ok(
                OPTIONInfo::new(OPTION::EXTENDED_ERROR.name(),
                                OPTION::EXTENDED_ERROR.code())
            ),
            "ECT" => Ok(
                OPTIONInfo::new(OPTION::ECT.name(),
                                OPTION::ECT.code())
            ),
            "EST" => Ok(
                OPTIONInfo::new(OPTION::EST.name(),
                                OPTION::EST.code())
            ),
            "UMBRELLA_IDENT" => Ok(
                OPTIONInfo::new(OPTION::UMBRELLA_IDENT.name(),
                                OPTION::UMBRELLA_IDENT.code())
            ),
            "DEVICE_ID" => Ok(
                OPTIONInfo::new(OPTION::DEVICE_ID.name(),
                                OPTION::DEVICE_ID.code())
            ),
            _ => Err(String::from("Can't encode OPTION-CODE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<OPTIONInfo, String> {
        return match *decimal {
            0 => Ok(
                OPTIONInfo::new(RESERVED,
                                *decimal)
            ),
            1 => Ok(
                OPTIONInfo::new(OPTION::LLQ.name(),
                                OPTION::LLQ.code())
            ),
            2 => Ok(
                OPTIONInfo::new(OPTION::UL.name(),
                                OPTION::UL.code())
            ),
            3 => Ok(
                OPTIONInfo::new(OPTION::NSID.name(),
                                OPTION::NSID.code())
            ),
            4 => Ok(
                OPTIONInfo::new(OPTION::OWNER.name(),
                                OPTION::OWNER.code())
            ),
            5 => Ok(
                OPTIONInfo::new(OPTION::DAU.name(),
                                OPTION::DAU.code())
            ),
            6 => Ok(
                OPTIONInfo::new(OPTION::DHU.name(),
                                OPTION::DHU.code())
            ),
            7 => Ok(
                OPTIONInfo::new(OPTION::N3U.name(),
                                OPTION::N3U.code())
            ),
            8 => Ok(
                OPTIONInfo::new(OPTION::ECS.name(),
                                OPTION::ECS.code())
            ),
            9 => Ok(
                OPTIONInfo::new(OPTION::EXPIRE.name(),
                                OPTION::EXPIRE.code())
            ),
            10 => Ok(
                OPTIONInfo::new(OPTION::COOKIE.name(),
                                OPTION::COOKIE.code())
            ),
            11 => Ok(
                OPTIONInfo::new(OPTION::ETK.name(),
                                OPTION::ETK.code())
            ),
            12 => Ok(
                OPTIONInfo::new(OPTION::PADDING.name(),
                                OPTION::PADDING.code())
            ),
            13 => Ok(
                OPTIONInfo::new(OPTION::CHAIN.name(),
                                OPTION::CHAIN.code())
            ),
            14 => Ok(
                OPTIONInfo::new(OPTION::EKT.name(),
                                OPTION::EKT.code())
            ),
            15 => Ok(
                OPTIONInfo::new(OPTION::EXTENDED_ERROR.name(),
                                OPTION::EXTENDED_ERROR.code())
            ),
            16 => Ok(
                OPTIONInfo::new(OPTION::ECT.name(),
                                OPTION::ECT.code())
            ),
            17 => Ok(
                OPTIONInfo::new(OPTION::EST.name(),
                                OPTION::EST.code())
            ),
            18..=20291 => Ok(
                OPTIONInfo::new(UNASSIGNED,
                                *decimal)
            ),
            20292 => Ok(
                OPTIONInfo::new(OPTION::UMBRELLA_IDENT.name(),
                                OPTION::UMBRELLA_IDENT.code())
            ),
            20293..=26945 => Ok(
                OPTIONInfo::new(UNASSIGNED,
                                *decimal)
            ),
            26946 => Ok(
                OPTIONInfo::new(OPTION::DEVICE_ID.name(),
                                OPTION::DEVICE_ID.code())
            ),
            26947..=65000 => Ok(
                OPTIONInfo::new(UNASSIGNED,
                                *decimal)
            ),
           65001..=65534 => Ok(
                OPTIONInfo::new(RESERVED_LOCAL_EXPERIMENTAL_USE,
                                *decimal)
            ),
            65535 => Ok(
                OPTIONInfo::new(RESERVED_FUTURE_EXPANSION_USE,
                                *decimal)
            ),
            _ => Err(String::from("Can't decode OPTION-CODE!"))
        }
    }
}
