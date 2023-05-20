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

use super::{OPTCODES};
use super::{OPTCODESInfo};
use super::{RESERVED, UNASSIGNED, RESERVED_FUTURE_EXPANSION_USE, RESERVED_LOCAL_EXPERIMENTAL_USE};


pub trait OPTCODESConversion {
    fn encode(name: &str) -> Result<OPTCODESInfo, String>;
    fn decode(dec: &u16) -> Result<OPTCODESInfo, String>;
}

impl OPTCODESConversion for OPTCODES {
    fn encode(name: &str) -> Result<OPTCODESInfo, String> {
        return match name {
            "LLQ" => Ok(
                OPTCODESInfo::new(OPTCODES::LLQ.name(),
                                  OPTCODES::LLQ.code())
            ),
            "UL" => Ok(
                OPTCODESInfo::new(OPTCODES::UL.name(),
                                  OPTCODES::UL.code())
            ),
            "NSID" => Ok(
                OPTCODESInfo::new(OPTCODES::NSID.name(),
                                  OPTCODES::NSID.code())
            ),
            "OWNER" => Ok(
                OPTCODESInfo::new(OPTCODES::OWNER.name(),
                                  OPTCODES::OWNER.code())
            ),
            "DAU" => Ok(
                OPTCODESInfo::new(OPTCODES::DAU.name(),
                                  OPTCODES::DAU.code())
            ),
            "DHU" => Ok(
                OPTCODESInfo::new(OPTCODES::DHU.name(),
                                  OPTCODES::DHU.code())
            ),
            "N3U" => Ok(
                OPTCODESInfo::new(OPTCODES::N3U.name(),
                                  OPTCODES::N3U.code())
            ),
            "ECS" => Ok(
                OPTCODESInfo::new(OPTCODES::ECS.name(),
                                  OPTCODES::ECS.code())
            ),
            "EXPIRE" => Ok(
                OPTCODESInfo::new(OPTCODES::EXPIRE.name(),
                                  OPTCODES::EXPIRE.code())
            ),
            "COOKIE" => Ok(
                OPTCODESInfo::new(OPTCODES::COOKIE.name(),
                                  OPTCODES::COOKIE.code())
            ),
            "ETK" => Ok(
                OPTCODESInfo::new(OPTCODES::ETK.name(),
                                  OPTCODES::ETK.code())
            ),
            "PADDING" => Ok(
                OPTCODESInfo::new(OPTCODES::PADDING.name(),
                                  OPTCODES::PADDING.code())
            ),
            "CHAIN" => Ok(
                OPTCODESInfo::new(OPTCODES::CHAIN.name(),
                                  OPTCODES::CHAIN.code())
            ),
            "EKT" => Ok(
                OPTCODESInfo::new(OPTCODES::EKT.name(),
                                  OPTCODES::EKT.code())
            ),
            "EXTENDED_ERROR" => Ok(
                OPTCODESInfo::new(OPTCODES::EXTERROR.name(),
                                  OPTCODES::EXTERROR.code())
            ),
            "ECT" => Ok(
                OPTCODESInfo::new(OPTCODES::ECT.name(),
                                  OPTCODES::ECT.code())
            ),
            "EST" => Ok(
                OPTCODESInfo::new(OPTCODES::EST.name(),
                                  OPTCODES::EST.code())
            ),
            "UMBRELLA_IDENT" => Ok(
                OPTCODESInfo::new(OPTCODES::UMBRELLA_IDENT.name(),
                                  OPTCODES::UMBRELLA_IDENT.code())
            ),
            "DEVICE_ID" => Ok(
                OPTCODESInfo::new(OPTCODES::DEVICE_ID.name(),
                                  OPTCODES::DEVICE_ID.code())
            ),
            _ => Err(String::from("Can't encode OPTION-CODE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<OPTCODESInfo, String> {
        return match *decimal {
            0 => Ok(
                OPTCODESInfo::new(RESERVED,
                                  *decimal)
            ),
            1 => Ok(
                OPTCODESInfo::new(OPTCODES::LLQ.name(),
                                  OPTCODES::LLQ.code())
            ),
            2 => Ok(
                OPTCODESInfo::new(OPTCODES::UL.name(),
                                  OPTCODES::UL.code())
            ),
            3 => Ok(
                OPTCODESInfo::new(OPTCODES::NSID.name(),
                                  OPTCODES::NSID.code())
            ),
            4 => Ok(
                OPTCODESInfo::new(OPTCODES::OWNER.name(),
                                  OPTCODES::OWNER.code())
            ),
            5 => Ok(
                OPTCODESInfo::new(OPTCODES::DAU.name(),
                                  OPTCODES::DAU.code())
            ),
            6 => Ok(
                OPTCODESInfo::new(OPTCODES::DHU.name(),
                                  OPTCODES::DHU.code())
            ),
            7 => Ok(
                OPTCODESInfo::new(OPTCODES::N3U.name(),
                                  OPTCODES::N3U.code())
            ),
            8 => Ok(
                OPTCODESInfo::new(OPTCODES::ECS.name(),
                                  OPTCODES::ECS.code())
            ),
            9 => Ok(
                OPTCODESInfo::new(OPTCODES::EXPIRE.name(),
                                  OPTCODES::EXPIRE.code())
            ),
            10 => Ok(
                OPTCODESInfo::new(OPTCODES::COOKIE.name(),
                                  OPTCODES::COOKIE.code())
            ),
            11 => Ok(
                OPTCODESInfo::new(OPTCODES::ETK.name(),
                                  OPTCODES::ETK.code())
            ),
            12 => Ok(
                OPTCODESInfo::new(OPTCODES::PADDING.name(),
                                  OPTCODES::PADDING.code())
            ),
            13 => Ok(
                OPTCODESInfo::new(OPTCODES::CHAIN.name(),
                                  OPTCODES::CHAIN.code())
            ),
            14 => Ok(
                OPTCODESInfo::new(OPTCODES::EKT.name(),
                                  OPTCODES::EKT.code())
            ),
            15 => Ok(
                OPTCODESInfo::new(OPTCODES::EXTERROR.name(),
                                  OPTCODES::EXTERROR.code())
            ),
            16 => Ok(
                OPTCODESInfo::new(OPTCODES::ECT.name(),
                                  OPTCODES::ECT.code())
            ),
            17 => Ok(
                OPTCODESInfo::new(OPTCODES::EST.name(),
                                  OPTCODES::EST.code())
            ),
            18..=20291 => Ok(
                OPTCODESInfo::new(UNASSIGNED,
                                  *decimal)
            ),
            20292 => Ok(
                OPTCODESInfo::new(OPTCODES::UMBRELLA_IDENT.name(),
                                  OPTCODES::UMBRELLA_IDENT.code())
            ),
            20293..=26945 => Ok(
                OPTCODESInfo::new(UNASSIGNED,
                                  *decimal)
            ),
            26946 => Ok(
                OPTCODESInfo::new(OPTCODES::DEVICE_ID.name(),
                                  OPTCODES::DEVICE_ID.code())
            ),
            26947..=65000 => Ok(
                OPTCODESInfo::new(UNASSIGNED,
                                  *decimal)
            ),
           65001..=65534 => Ok(
                OPTCODESInfo::new(RESERVED_LOCAL_EXPERIMENTAL_USE,
                                  *decimal)
            ),
            65535 => Ok(
                OPTCODESInfo::new(RESERVED_FUTURE_EXPANSION_USE,
                                  *decimal)
            ),
            _ => Err(String::from("Can't decode OPTION-CODE!"))
        }
    }
}
