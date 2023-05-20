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

use crate::data_types::{U4, U12};

use super::{RCODES};
use super::{RCODEInfo, RCODERawInfo, RCODEEDNSInfo};
use super::{REVERSED, UNASSIGNED, REVERSED_PRIVATE_USE};


pub trait RCODEConversion {
    fn encode(name: &str) -> Result<RCODEInfo, String>;
    fn decode(decimal: &u8) -> Result<RCODEInfo, String>;
    fn encode_edns(name: &str) -> Result<RCODEEDNSInfo, String>;
    fn decode_edns(dec: &u16) -> Result<RCODEEDNSInfo, String>;
    fn encode_raw(name: &str) -> Result<RCODERawInfo, String>;
    fn decode_raw(dec: &u16) -> Result<RCODERawInfo, String>;
}

impl RCODEConversion for RCODES {
    fn encode(name: &str) -> Result<RCODEInfo, String> {
        return match name {
            "NoError" => Ok(
                RCODEInfo::new(RCODES::NoError.rcode(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            "FormErr" => Ok(
                RCODEInfo::new(RCODES::FormErr.rcode(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            "ServFail" => Ok(
                RCODEInfo::new(RCODES::ServFail.rcode(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            "NXDomain" => Ok(
                RCODEInfo::new(RCODES::NXDomain.rcode(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            "NotImp" => Ok(
                RCODEInfo::new(RCODES::NotImp.rcode(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            "Refused" => Ok(
                RCODEInfo::new(RCODES::Refused.rcode(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            "YXDomain" => Ok(
                RCODEInfo::new(RCODES::YXDomain.rcode(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            "YXRRSet" => Ok(
                RCODEInfo::new(RCODES::YXRRSet.rcode(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            "NXRRSet" => Ok(
                RCODEInfo::new(RCODES::NXRRSet.rcode(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            "NotAuth" => Ok(
                RCODEInfo::new(RCODES::NotAuth.rcode(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            "NotZone" => Ok(
                RCODEInfo::new(RCODES::NotZone.rcode(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            "DSOTYPENI" => Ok(
                RCODEInfo::new(RCODES::DSOTYPENI.rcode(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            _ => Err(String::from("Can't encode RCODE (4 bits)!"))
        }
    }

    fn decode(decimal: &u8) -> Result<RCODEInfo, String> {
        return match *decimal {
            0 => Ok(
                RCODEInfo::new(RCODES::NoError.rcode(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            1 => Ok(
                RCODEInfo::new(RCODES::FormErr.rcode(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            2 => Ok(
                RCODEInfo::new(RCODES::ServFail.rcode(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            3 => Ok(
                RCODEInfo::new(RCODES::NXDomain.rcode(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            4 => Ok(
                RCODEInfo::new(RCODES::NotImp.rcode(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            5 => Ok(
                RCODEInfo::new(RCODES::Refused.rcode(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            6 => Ok(
                RCODEInfo::new(RCODES::YXDomain.rcode(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            7 => Ok(
                RCODEInfo::new(RCODES::YXRRSet.rcode(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            8 => Ok(
                RCODEInfo::new(RCODES::NXRRSet.rcode(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            9 => Ok(
                RCODEInfo::new(RCODES::NotAuth.rcode(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            10 => Ok(
                RCODEInfo::new(RCODES::NotZone.rcode(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            11 => Ok(
                RCODEInfo::new(RCODES::DSOTYPENI.rcode(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            12..=15 => Ok(
                RCODEInfo::new(U4::new(*decimal),
                                  UNASSIGNED,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Can't decode RCODE (4 bits)!"))
        }
    }

    fn encode_edns(name: &str) -> Result<RCODEEDNSInfo, String> {
        return match name {
            "NoError" => Ok(
                RCODEEDNSInfo::new(RCODES::NoError.rcode_edns(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            "FormErr" => Ok(
                RCODEEDNSInfo::new(RCODES::FormErr.rcode_edns(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            "ServFail" => Ok(
                RCODEEDNSInfo::new(RCODES::ServFail.rcode_edns(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            "NXDomain" => Ok(
                RCODEEDNSInfo::new(RCODES::NXDomain.rcode_edns(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            "NotImp" => Ok(
                RCODEEDNSInfo::new(RCODES::NotImp.rcode_edns(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            "Refused" => Ok(
                RCODEEDNSInfo::new(RCODES::Refused.rcode_edns(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            "YXDomain" => Ok(
                RCODEEDNSInfo::new(RCODES::YXDomain.rcode_edns(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            "YXRRSet" => Ok(
                RCODEEDNSInfo::new(RCODES::YXRRSet.rcode_edns(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            "NXRRSet" => Ok(
                RCODEEDNSInfo::new(RCODES::NXRRSet.rcode_edns(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            "NotAuth" => Ok(
                RCODEEDNSInfo::new(RCODES::NotAuth.rcode_edns(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            "NotZone" => Ok(
                RCODEEDNSInfo::new(RCODES::NotZone.rcode_edns(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            "DSOTYPENI" => Ok(
                RCODEEDNSInfo::new(RCODES::DSOTYPENI.rcode_edns(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            "BADVERS" | "BADSIG" | "BADVERS_BADSIG" => Ok(
                RCODEEDNSInfo::new(RCODES::BADVERS_BADSIG.rcode_edns(),
                                  RCODES::BADVERS_BADSIG.name(),
                                  RCODES::BADVERS_BADSIG.description())
            ),
            "BADKEY" => Ok(
                RCODEEDNSInfo::new(RCODES::BADKEY.rcode_edns(),
                                  RCODES::BADKEY.name(),
                                  RCODES::BADKEY.description())
            ),
            "BADTIME" => Ok(
                RCODEEDNSInfo::new(RCODES::BADTIME.rcode_edns(),
                                  RCODES::BADTIME.name(),
                                  RCODES::BADTIME.description())
            ),
            "BADMODE" => Ok(
                RCODEEDNSInfo::new(RCODES::BADMODE.rcode_edns(),
                                  RCODES::BADMODE.name(),
                                  RCODES::BADMODE.description())
            ),
            "BADNAME" => Ok(
                RCODEEDNSInfo::new(RCODES::BADNAME.rcode_edns(),
                                  RCODES::BADNAME.name(),
                                  RCODES::BADNAME.description())
            ),
            "BADALG" => Ok(
                RCODEEDNSInfo::new(RCODES::BADALG.rcode_edns(),
                                  RCODES::BADALG.name(),
                                  RCODES::BADALG.description())
            ),
            "BADTRUNC" => Ok(
                RCODEEDNSInfo::new(RCODES::BADTRUNC.rcode_edns(),
                                  RCODES::BADTRUNC.name(),
                                  RCODES::BADTRUNC.description())
            ),
            "BADCOOKIE" => Ok(
                RCODEEDNSInfo::new(RCODES::BADCOOKIE.rcode_edns(),
                                  RCODES::BADCOOKIE.name(),
                                  RCODES::BADCOOKIE.description())
            ),
            _ => Err(String::from("Can't encode EDNS RCODE (12 bits)!"))
        }
    }

    fn decode_edns(decimal: &u16) -> Result<RCODEEDNSInfo, String> {
        return match *decimal {
            0 => Ok(
                RCODEEDNSInfo::new(RCODES::NoError.rcode_edns(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            1 => Ok(
                RCODEEDNSInfo::new(RCODES::FormErr.rcode_edns(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            2 => Ok(
                RCODEEDNSInfo::new(RCODES::ServFail.rcode_edns(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            3 => Ok(
                RCODEEDNSInfo::new(RCODES::NXDomain.rcode_edns(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            4 => Ok(
                RCODEEDNSInfo::new(RCODES::NotImp.rcode_edns(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            5 => Ok(
                RCODEEDNSInfo::new(RCODES::Refused.rcode_edns(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            6 => Ok(
                RCODEEDNSInfo::new(RCODES::YXDomain.rcode_edns(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            7 => Ok(
                RCODEEDNSInfo::new(RCODES::YXRRSet.rcode_edns(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            8 => Ok(
                RCODEEDNSInfo::new(RCODES::NXRRSet.rcode_edns(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            9 => Ok(
                RCODEEDNSInfo::new(RCODES::NotAuth.rcode_edns(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            10 => Ok(
                RCODEEDNSInfo::new(RCODES::NotZone.rcode_edns(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            11 => Ok(
                RCODEEDNSInfo::new(RCODES::DSOTYPENI.rcode_edns(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            12..=15 => Ok(
                RCODEEDNSInfo::new(U12::new(*decimal),
                                  UNASSIGNED,
                                  UNASSIGNED)
            ),
            16 => Ok(
                RCODEEDNSInfo::new(RCODES::BADVERS_BADSIG.rcode_edns(),
                                  RCODES::BADVERS_BADSIG.name(),
                                  RCODES::BADVERS_BADSIG.description())
            ),
            17 => Ok(
                RCODEEDNSInfo::new(RCODES::BADKEY.rcode_edns(),
                                  RCODES::BADKEY.name(),
                                  RCODES::BADKEY.description())
            ),
            18 => Ok(
                RCODEEDNSInfo::new(RCODES::BADTIME.rcode_edns(),
                                  RCODES::BADTIME.name(),
                                  RCODES::BADTIME.description())
            ),
            19 => Ok(
                RCODEEDNSInfo::new(RCODES::BADMODE.rcode_edns(),
                                  RCODES::BADMODE.name(),
                                  RCODES::BADMODE.description())
            ),
            20 => Ok(
                RCODEEDNSInfo::new(RCODES::BADNAME.rcode_edns(),
                                  RCODES::BADNAME.name(),
                                  RCODES::BADNAME.description())
            ),
            21 => Ok(
                RCODEEDNSInfo::new(RCODES::BADALG.rcode_edns(),
                                  RCODES::BADALG.name(),
                                  RCODES::BADALG.description())
            ),
            22 => Ok(
                RCODEEDNSInfo::new(RCODES::BADTRUNC.rcode_edns(),
                                  RCODES::BADTRUNC.name(),
                                  RCODES::BADTRUNC.description())
            ),
            23 => Ok(
                RCODEEDNSInfo::new(RCODES::BADCOOKIE.rcode_edns(),
                                  RCODES::BADCOOKIE.name(),
                                  RCODES::BADCOOKIE.description())
            ),
            24..=3840 => Ok(
                RCODEEDNSInfo::new(U12::new(*decimal),
                                  UNASSIGNED,
                                  UNASSIGNED)
            ),
            3841..=4095 => Ok(
                RCODEEDNSInfo::new(U12::new(*decimal),
                                  REVERSED_PRIVATE_USE,
                                  REVERSED_PRIVATE_USE)
            ),
            _ => Err(String::from("Can't decode EDNS RCODE (12 bits)!"))
        }
    }

    fn encode_raw(name: &str) -> Result<RCODERawInfo, String> {
        return match name {
            "NoError" => Ok(
                RCODERawInfo::new(RCODES::NoError.rcode_raw(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            "FormErr" => Ok(
                RCODERawInfo::new(RCODES::FormErr.rcode_raw(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            "ServFail" => Ok(
                RCODERawInfo::new(RCODES::ServFail.rcode_raw(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            "NXDomain" => Ok(
                RCODERawInfo::new(RCODES::NXDomain.rcode_raw(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            "NotImp" => Ok(
                RCODERawInfo::new(RCODES::NotImp.rcode_raw(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            "Refused" => Ok(
                RCODERawInfo::new(RCODES::Refused.rcode_raw(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            "YXDomain" => Ok(
                RCODERawInfo::new(RCODES::YXDomain.rcode_raw(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            "YXRRSet" => Ok(
                RCODERawInfo::new(RCODES::YXRRSet.rcode_raw(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            "NXRRSet" => Ok(
                RCODERawInfo::new(RCODES::NXRRSet.rcode_raw(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            "NotAuth" => Ok(
                RCODERawInfo::new(RCODES::NotAuth.rcode_raw(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            "NotZone" => Ok(
                RCODERawInfo::new(RCODES::NotZone.rcode_raw(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            "DSOTYPENI" => Ok(
                RCODERawInfo::new(RCODES::DSOTYPENI.rcode_raw(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            "BADVERS" | "BADSIG" | "BADVERS_BADSIG" => Ok(
                RCODERawInfo::new(RCODES::BADVERS_BADSIG.rcode_raw(),
                                  RCODES::BADVERS_BADSIG.name(),
                                  RCODES::BADVERS_BADSIG.description())
            ),
            "BADKEY" => Ok(
                RCODERawInfo::new(RCODES::BADKEY.rcode_raw(),
                                  RCODES::BADKEY.name(),
                                  RCODES::BADKEY.description())
            ),
            "BADTIME" => Ok(
                RCODERawInfo::new(RCODES::BADTIME.rcode_raw(),
                                  RCODES::BADTIME.name(),
                                  RCODES::BADTIME.description())
            ),
            "BADMODE" => Ok(
                RCODERawInfo::new(RCODES::BADMODE.rcode_raw(),
                                  RCODES::BADMODE.name(),
                                  RCODES::BADMODE.description())
            ),
            "BADNAME" => Ok(
                RCODERawInfo::new(RCODES::BADNAME.rcode_raw(),
                                  RCODES::BADNAME.name(),
                                  RCODES::BADNAME.description())
            ),
            "BADALG" => Ok(
                RCODERawInfo::new(RCODES::BADALG.rcode_raw(),
                                  RCODES::BADALG.name(),
                                  RCODES::BADALG.description())
            ),
            "BADTRUNC" => Ok(
                RCODERawInfo::new(RCODES::BADTRUNC.rcode_raw(),
                                  RCODES::BADTRUNC.name(),
                                  RCODES::BADTRUNC.description())
            ),
            "BADCOOKIE" => Ok(
                RCODERawInfo::new(RCODES::BADCOOKIE.rcode_raw(),
                                  RCODES::BADCOOKIE.name(),
                                  RCODES::BADCOOKIE.description())
            ),
            _ => Err(String::from("Can't encode RAW RCODE!"))
        }
    }

    fn decode_raw(decimal: &u16) -> Result<RCODERawInfo, String> {
        return match *decimal {
            0 => Ok(
                RCODERawInfo::new(RCODES::NoError.rcode_raw(),
                                  RCODES::NoError.name(),
                                  RCODES::NoError.description())
            ),
            1 => Ok(
                RCODERawInfo::new(RCODES::FormErr.rcode_raw(),
                                  RCODES::FormErr.name(),
                                  RCODES::FormErr.description())
            ),
            2 => Ok(
                RCODERawInfo::new(RCODES::ServFail.rcode_raw(),
                                  RCODES::ServFail.name(),
                                  RCODES::ServFail.description())
            ),
            3 => Ok(
                RCODERawInfo::new(RCODES::NXDomain.rcode_raw(),
                                  RCODES::NXDomain.name(),
                                  RCODES::NXDomain.description())
            ),
            4 => Ok(
                RCODERawInfo::new(RCODES::NotImp.rcode_raw(),
                                  RCODES::NotImp.name(),
                                  RCODES::NotImp.description())
            ),
            5 => Ok(
                RCODERawInfo::new(RCODES::Refused.rcode_raw(),
                                  RCODES::Refused.name(),
                                  RCODES::Refused.description())
            ),
            6 => Ok(
                RCODERawInfo::new(RCODES::YXDomain.rcode_raw(),
                                  RCODES::YXDomain.name(),
                                  RCODES::YXDomain.description())
            ),
            7 => Ok(
                RCODERawInfo::new(RCODES::YXRRSet.rcode_raw(),
                                  RCODES::YXRRSet.name(),
                                  RCODES::YXRRSet.description())
            ),
            8 => Ok(
                RCODERawInfo::new(RCODES::NXRRSet.rcode_raw(),
                                  RCODES::NXRRSet.name(),
                                  RCODES::NXRRSet.description())
            ),
            9 => Ok(
                RCODERawInfo::new(RCODES::NotAuth.rcode_raw(),
                                  RCODES::NotAuth.name(),
                                  RCODES::NotAuth.description())
            ),
            10 => Ok(
                RCODERawInfo::new(RCODES::NotZone.rcode_raw(),
                                  RCODES::NotZone.name(),
                                  RCODES::NotZone.description())
            ),
            11 => Ok(
                RCODERawInfo::new(RCODES::DSOTYPENI.rcode_raw(),
                                  RCODES::DSOTYPENI.name(),
                                  RCODES::DSOTYPENI.description())
            ),
            12..=15 => Ok(
                RCODERawInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            16 => Ok(
                RCODERawInfo::new(RCODES::BADVERS_BADSIG.rcode_raw(),
                                  RCODES::BADVERS_BADSIG.name(),
                                  RCODES::BADVERS_BADSIG.description())
            ),
            17 => Ok(
                RCODERawInfo::new(RCODES::BADKEY.rcode_raw(),
                                  RCODES::BADKEY.name(),
                                  RCODES::BADKEY.description())
            ),
            18 => Ok(
                RCODERawInfo::new(RCODES::BADTIME.rcode_raw(),
                                  RCODES::BADTIME.name(),
                                  RCODES::BADTIME.description())
            ),
            19 => Ok(
                RCODERawInfo::new(RCODES::BADMODE.rcode_raw(),
                                  RCODES::BADMODE.name(),
                                  RCODES::BADMODE.description())
            ),
            20 => Ok(
                RCODERawInfo::new(RCODES::BADNAME.rcode_raw(),
                                  RCODES::BADNAME.name(),
                                  RCODES::BADNAME.description())
            ),
            21 => Ok(
                RCODERawInfo::new(RCODES::BADALG.rcode_raw(),
                                  RCODES::BADALG.name(),
                                  RCODES::BADALG.description())
            ),
            22 => Ok(
                RCODERawInfo::new(RCODES::BADTRUNC.rcode_raw(),
                                  RCODES::BADTRUNC.name(),
                                  RCODES::BADTRUNC.description())
            ),
            23 => Ok(
                RCODERawInfo::new(RCODES::BADCOOKIE.rcode_raw(),
                                  RCODES::BADCOOKIE.name(),
                                  RCODES::BADCOOKIE.description())
            ),
            24..=3840 => Ok(
                RCODERawInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            3841..=4095 => Ok(
                RCODERawInfo::new(*decimal,
                               REVERSED_PRIVATE_USE,
                               REVERSED_PRIVATE_USE)
            ),
            4096..=65534 => Ok(
                RCODERawInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            65535 => Ok(
                RCODERawInfo::new(*decimal,
                               REVERSED,
                               REVERSED)
            ),
            _ => Err(String::from("Can't decode RAW RCODE!"))
        }
    }
}
