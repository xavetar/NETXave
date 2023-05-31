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

use super::{U4, U12};
use super::{Newable};
use super::{RESERVED, UNASSIGNED, RESERVED_PRIVATE_USE};
use super::{NOT_AUTH_W, BAD_VERS_BAD_SIG_W};
use super::{RCODE_Details};
use super::{TwoResult, TwoResult::First, TwoResult::Second};

pub enum RCODE {
    NoError = 0,
    FormErr = 1,
    ServFail = 2,
    NXDomain = 3,
    NotImp = 4,
    Refused = 5,
    YXDomain = 6,
    YXRRSet = 7,
    NXRRSet = 8,
    NotAuth = 9,
    NotZone = 10,
    DSOTYPENI = 11,
    BADVERS_BADSIG = 16,
    BADKEY = 17,
    BADTIME = 18,
    BADMODE = 19,
    BADNAME = 20,
    BADALG = 21,
    BADTRUNC = 22,
    BADCOOKIE = 23
}

impl From<RCODE> for u16 {
    fn from(value: RCODE) -> Self {
        value as u16
    }
}

impl RCODE {
    pub fn code<I, R>(&self) -> R
        where I: From<RCODE>, R: Newable<I, R> {
        return match self {
            RCODE::NoError => R::new(I::from(RCODE::NoError)),
            RCODE::FormErr => R::new(I::from(RCODE::FormErr)),
            RCODE::ServFail => R::new(I::from(RCODE::ServFail)),
            RCODE::NXDomain => R::new(I::from(RCODE::NXDomain)),
            RCODE::NotImp => R::new(I::from(RCODE::NotImp)),
            RCODE::Refused => R::new(I::from(RCODE::Refused)),
            RCODE::YXDomain => R::new(I::from(RCODE::YXDomain)),
            RCODE::YXRRSet => R::new(I::from(RCODE::YXRRSet)),
            RCODE::NXRRSet => R::new(I::from(RCODE::NXRRSet)),
            RCODE::NotAuth => {
                eprintln!("{NOT_AUTH_W}");
                return R::new(I::from(RCODE::NotAuth));
            },
            RCODE::NotZone => R::new(I::from(RCODE::NotZone)),
            RCODE::DSOTYPENI => R::new(I::from(RCODE::DSOTYPENI)),
            RCODE::BADVERS_BADSIG => {
                eprintln!("{BAD_VERS_BAD_SIG_W}");
                return R::new(I::from(RCODE::BADVERS_BADSIG));
            },
            RCODE::BADKEY => R::new(I::from(RCODE::BADKEY)),
            RCODE::BADTIME => R::new(I::from(RCODE::BADTIME)),
            RCODE::BADMODE => R::new(I::from(RCODE::BADMODE)),
            RCODE::BADNAME => R::new(I::from(RCODE::BADNAME)),
            RCODE::BADALG => R::new(I::from(RCODE::BADALG)),
            RCODE::BADTRUNC => R::new(I::from(RCODE::BADTRUNC)),
            RCODE::BADCOOKIE => R::new(I::from(RCODE::BADCOOKIE))
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            RCODE::NoError => "NoError",
            RCODE::FormErr => "FormErr",
            RCODE::ServFail => "ServFail",
            RCODE::NXDomain => "NXDomain",
            RCODE::NotImp => "NotImp",
            RCODE::Refused => "Refused",
            RCODE::YXDomain => "YXDomain",
            RCODE::YXRRSet => "YXRRSet",
            RCODE::NXRRSet => "NXRRSet",
            RCODE::NotAuth => "NotAuth",
            RCODE::NotZone => "NotZone",
            RCODE::DSOTYPENI => "DSOTYPENI",
            RCODE::BADVERS_BADSIG => "BADVERS_BADSIG",
            RCODE::BADKEY => "BADKEY",
            RCODE::BADTIME => "BADTIME",
            RCODE::BADMODE => "BADMODE",
            RCODE::BADNAME => "BADNAME",
            RCODE::BADALG => "BADALG",
            RCODE::BADTRUNC => "BADTRUNC",
            RCODE::BADCOOKIE => "BADCOOKIE"
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            RCODE::NoError => "No Error",
            RCODE::FormErr => "Format Error",
            RCODE::ServFail => "Server Failure",
            RCODE::NXDomain => "Non-Existent Domain",
            RCODE::NotImp => "Not Implemented",
            RCODE::Refused => "Query Refused",
            RCODE::YXDomain => "Name Exists when it should not",
            RCODE::YXRRSet => "RR Set Exists when it should not",
            RCODE::NXRRSet => "RR Set that should exist does not",
            RCODE::NotAuth => "Server Not Authoritative for zone/Not Authorized",
            RCODE::NotZone => "Name not contained in zone",
            RCODE::DSOTYPENI => "DSO-TYPE Not Implemented",
            RCODE::BADVERS_BADSIG => "Bad OPT Version/TSIG Signature Failure",
            RCODE::BADKEY => "Key not recognized",
            RCODE::BADTIME => "Signature out of time window",
            RCODE::BADMODE => "Bad TKEY Mode",
            RCODE::BADNAME => "Duplicate key name",
            RCODE::BADALG => "Algorithm not supported",
            RCODE::BADTRUNC => "Bad Truncation",
            RCODE::BADCOOKIE => "Bad/missing Server Cookie"
        }
    }

    pub fn encode(t: &str) -> RCODE {
        return match t {
            "NoError" => RCODE::NoError,
            "FormErr" => RCODE::FormErr,
            "ServFail" => RCODE::ServFail,
            "NXDomain" => RCODE::NXDomain,
            "NotImp" => RCODE::NotImp,
            "Refused" => RCODE::Refused,
            "YXDomain" => RCODE::YXDomain,
            "YXRRSet" => RCODE::YXRRSet,
            "NXRRSet" => RCODE::NXRRSet,
            "NotAuth" => RCODE::NotAuth,
            "NotZone" => RCODE::NotZone,
            "DSOTYPENI" => RCODE::DSOTYPENI,
            "BADVERS" | "BADSIG" | "BADVERS_BADSIG" => RCODE::BADVERS_BADSIG,
            "BADKEY" => RCODE::BADKEY,
            "BADTIME" => RCODE::BADTIME,
            "BADMODE" => RCODE::BADMODE,
            "BADNAME" => RCODE::BADNAME,
            "BADALG" => RCODE::BADALG,
            "BADTRUNC" => RCODE::BADTRUNC,
            "BADCOOKIE" => RCODE::BADCOOKIE,
            _ => panic!("Can't encode RCODE!")
        }
    }

    pub fn decode<I, R>(t: &I) -> TwoResult<RCODE, RCODE_Details<R>>
        where I: Clone + Into<u16>, R: Newable<I, R> {
        return match (*t).clone().into() {
            0 => First(RCODE::NoError),
            1 => First(RCODE::FormErr),
            2 => First(RCODE::ServFail),
            3 => First(RCODE::NXDomain),
            4 => First(RCODE::NotImp),
            5 => First(RCODE::Refused),
            6 => First(RCODE::YXDomain),
            7 => First(RCODE::YXRRSet),
            8 => First(RCODE::NXRRSet),
            9 => First(RCODE::NotAuth),
            10 => First(RCODE::NotZone),
            11 => First(RCODE::DSOTYPENI),
            12..=15 => Second(RCODE_Details::<R>::new(UNASSIGNED,R::new((*t).clone()),UNASSIGNED)),
            16 => First(RCODE::BADVERS_BADSIG),
            17 => First(RCODE::BADKEY),
            18 => First(RCODE::BADTIME),
            19 => First(RCODE::BADMODE),
            20 => First(RCODE::BADNAME),
            21 => First(RCODE::BADALG),
            22 => First(RCODE::BADTRUNC),
            23 => First(RCODE::BADCOOKIE),
            24..=3840 => Second(RCODE_Details::<R>::new(UNASSIGNED,R::new((*t).clone()),UNASSIGNED)),
            3841..=4095 => Second(RCODE_Details::<R>::new(RESERVED_PRIVATE_USE,R::new((*t).clone()),RESERVED_PRIVATE_USE)),
            4096..=65534 => Second(RCODE_Details::<R>::new(UNASSIGNED,R::new((*t).clone()),UNASSIGNED)),
            65535 => Second(RCODE_Details::<R>::new(RESERVED,R::new((*t).clone()),RESERVED)),
            _ => panic!("Can't decode RCODE!")
        }
    }
}
