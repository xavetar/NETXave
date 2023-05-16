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

use super::warnings::{NOT_AUTH_W, BAD_VERS_BAD_SIG_W};

pub enum RCODES {
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
    BADCOOKIE = 23,
}

impl RCODES {
    pub fn rcode(&self) -> u16 {
        return match self {
            RCODES::NoError => RCODES::NoError as u16,
            RCODES::FormErr => RCODES::FormErr as u16,
            RCODES::ServFail => RCODES::ServFail as u16,
            RCODES::NXDomain => RCODES::NXDomain as u16,
            RCODES::NotImp => RCODES::NotImp as u16,
            RCODES::Refused => RCODES::Refused as u16,
            RCODES::YXDomain => RCODES::YXDomain as u16,
            RCODES::YXRRSet => RCODES::YXRRSet as u16,
            RCODES::NXRRSet => RCODES::NXRRSet as u16,
            RCODES::NotAuth => {
                eprintln!("{NOT_AUTH_W}");
                return RCODES::NotAuth as u16;
            },
            RCODES::NotZone => RCODES::NotZone as u16,
            RCODES::DSOTYPENI => RCODES::DSOTYPENI as u16,
            RCODES::BADVERS_BADSIG => {
                eprintln!("{BAD_VERS_BAD_SIG_W}");
                return RCODES::BADVERS_BADSIG as u16;
            },
            RCODES::BADKEY => RCODES::BADKEY as u16,
            RCODES::BADTIME => RCODES::BADTIME as u16,
            RCODES::BADMODE => RCODES::BADMODE as u16,
            RCODES::BADNAME => RCODES::BADNAME as u16,
            RCODES::BADALG => RCODES::BADALG as u16,
            RCODES::BADTRUNC => RCODES::BADTRUNC as u16,
            RCODES::BADCOOKIE => RCODES::BADCOOKIE as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            RCODES::NoError => "NoError",
            RCODES::FormErr => "FormErr",
            RCODES::ServFail => "ServFail",
            RCODES::NXDomain => "NXDomain",
            RCODES::NotImp => "NotImp",
            RCODES::Refused => "Refused",
            RCODES::YXDomain => "YXDomain",
            RCODES::YXRRSet => "YXRRSet",
            RCODES::NXRRSet => "NXRRSet",
            RCODES::NotAuth => "NotAuth",
            RCODES::NotZone => "NotZone",
            RCODES::DSOTYPENI => "DSOTYPENI",
            RCODES::BADVERS_BADSIG => "BADVERS_BADSIG",
            RCODES::BADKEY => "BADKEY",
            RCODES::BADTIME => "BADTIME",
            RCODES::BADMODE => "BADMODE",
            RCODES::BADNAME => "BADNAME",
            RCODES::BADALG => "BADALG",
            RCODES::BADTRUNC => "BADTRUNC",
            RCODES::BADCOOKIE => "BADCOOKIE",
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            RCODES::NoError => "No Error",
            RCODES::FormErr => "Format Error",
            RCODES::ServFail => "Server Failure",
            RCODES::NXDomain => "Non-Existent Domain",
            RCODES::NotImp => "Not Implemented",
            RCODES::Refused => "Query Refused",
            RCODES::YXDomain => "Name Exists when it should not",
            RCODES::YXRRSet => "RR Set Exists when it should not",
            RCODES::NXRRSet => "RR Set that should exist does not",
            RCODES::NotAuth => "Server Not Authoritative for zone/Not Authorized",
            RCODES::NotZone => "Name not contained in zone",
            RCODES::DSOTYPENI => "DSO-TYPE Not Implemented",
            RCODES::BADVERS_BADSIG => "Bad OPT Version/TSIG Signature Failure",
            RCODES::BADKEY => "Key not recognized",
            RCODES::BADTIME => "Signature out of time window",
            RCODES::BADMODE => "Bad TKEY Mode",
            RCODES::BADNAME => "Duplicate key name",
            RCODES::BADALG => "Algorithm not supported",
            RCODES::BADTRUNC => "Bad Truncation",
            RCODES::BADCOOKIE => "Bad/missing Server Cookie"
        }
    }
}
