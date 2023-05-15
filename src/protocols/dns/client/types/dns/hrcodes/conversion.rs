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

use super::{RCODES};
use super::{RCODEInfo};
use super::{REVERSED, UNASSIGNED, REVERSED_PRIVATE_USE};


pub trait RCODEConversion {
    fn encode(name: &str) -> Result<RCODEInfo, String>;
    fn decode(dec: &u16) -> Result<RCODEInfo, String>;
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
            "BADVERS" | "BADSIG" | "BADVERS_BADSIG" => Ok(
                RCODEInfo::new(RCODES::BADVERS_BADSIG.rcode(),
                               RCODES::BADVERS_BADSIG.name(),
                               RCODES::BADVERS_BADSIG.description())
            ),
            "BADKEY" => Ok(
                RCODEInfo::new(RCODES::BADKEY.rcode(),
                               RCODES::BADKEY.name(),
                               RCODES::BADKEY.description())
            ),
            "BADTIME" => Ok(
                RCODEInfo::new(RCODES::BADTIME.rcode(),
                               RCODES::BADTIME.name(),
                               RCODES::BADTIME.description())
            ),
            "BADMODE" => Ok(
                RCODEInfo::new(RCODES::BADMODE.rcode(),
                               RCODES::BADMODE.name(),
                               RCODES::BADMODE.description())
            ),
            "BADNAME" => Ok(
                RCODEInfo::new(RCODES::BADNAME.rcode(),
                               RCODES::BADNAME.name(),
                               RCODES::BADNAME.description())
            ),
            "BADALG" => Ok(
                RCODEInfo::new(RCODES::BADALG.rcode(),
                               RCODES::BADALG.name(),
                               RCODES::BADALG.description())
            ),
            "BADTRUNC" => Ok(
                RCODEInfo::new(RCODES::BADTRUNC.rcode(),
                               RCODES::BADTRUNC.name(),
                               RCODES::BADTRUNC.description())
            ),
            "BADCOOKIE" => Ok(
                RCODEInfo::new(RCODES::BADCOOKIE.rcode(),
                               RCODES::BADCOOKIE.name(),
                               RCODES::BADCOOKIE.description())
            ),
            _ => Err(String::from("Can't encode RCODE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<RCODEInfo, String> {
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
                RCODEInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            16 => Ok(
                RCODEInfo::new(RCODES::BADVERS_BADSIG.rcode(),
                               RCODES::BADVERS_BADSIG.name(),
                               RCODES::BADVERS_BADSIG.description())
            ),
            17 => Ok(
                RCODEInfo::new(RCODES::BADKEY.rcode(),
                               RCODES::BADKEY.name(),
                               RCODES::BADKEY.description())
            ),
            18 => Ok(
                RCODEInfo::new(RCODES::BADTIME.rcode(),
                               RCODES::BADTIME.name(),
                               RCODES::BADTIME.description())
            ),
            19 => Ok(
                RCODEInfo::new(RCODES::BADMODE.rcode(),
                               RCODES::BADMODE.name(),
                               RCODES::BADMODE.description())
            ),
            20 => Ok(
                RCODEInfo::new(RCODES::BADNAME.rcode(),
                               RCODES::BADNAME.name(),
                               RCODES::BADNAME.description())
            ),
            21 => Ok(
                RCODEInfo::new(RCODES::BADALG.rcode(),
                               RCODES::BADALG.name(),
                               RCODES::BADALG.description())
            ),
            22 => Ok(
                RCODEInfo::new(RCODES::BADTRUNC.rcode(),
                               RCODES::BADTRUNC.name(),
                               RCODES::BADTRUNC.description())
            ),
            23 => Ok(
                RCODEInfo::new(RCODES::BADCOOKIE.rcode(),
                               RCODES::BADCOOKIE.name(),
                               RCODES::BADCOOKIE.description())
            ),
            24..=3840 => Ok(
                RCODEInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            3841..=4095 => Ok(
                RCODEInfo::new(*decimal,
                               REVERSED_PRIVATE_USE,
                               REVERSED_PRIVATE_USE)
            ),
            4096..=65534 => Ok(
                RCODEInfo::new(*decimal,
                               UNASSIGNED,
                               UNASSIGNED)
            ),
            65535 => Ok(
                RCODEInfo::new(*decimal,
                               REVERSED,
                               REVERSED)
            ),
            _ => Err(String::from("Can't decode RCODE!"))
        }
    }
}
