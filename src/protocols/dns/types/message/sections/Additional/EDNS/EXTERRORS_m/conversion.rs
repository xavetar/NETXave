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

use super::{EXTERRORS};
use super::{EXTERRORSInfo};
use super::{UNASSIGNED, RESERVED_PRIVATE_USE};

pub trait EXTERRORSConversion {
    fn encode(name: &str) -> Result<EXTERRORSInfo, String>;
    fn decode(dec: &u16) -> Result<EXTERRORSInfo, String>;
}

impl EXTERRORSConversion for EXTERRORS {
    fn encode(name: &str) -> Result<EXTERRORSInfo, String> {
        return match name {
            "OTHER_ERROR" => Ok(
                EXTERRORSInfo::new(EXTERRORS::OTHER_ERROR.name(),
                                   EXTERRORS::OTHER_ERROR.code())
            ),
            "UNSUPPORTED_DNSKEY_ALGORITHM" => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM.name(),
                                   EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM.code())
            ),
            "UNSUPPORTED_DS_DIGEST_TYPE" => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE.name(),
                                   EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE.code())
            ),
            "STALE_ANSWER" => Ok(
                EXTERRORSInfo::new(EXTERRORS::STALE_ANSWER.name(),
                                   EXTERRORS::STALE_ANSWER.code())
            ),
            "FORGED_ANSWER" => Ok(
                EXTERRORSInfo::new(EXTERRORS::FORGED_ANSWER.name(),
                                   EXTERRORS::FORGED_ANSWER.code())
            ),
            "DNSSEC_INDETERMINATE" => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSSEC_INDETERMINATE.name(),
                                   EXTERRORS::DNSSEC_INDETERMINATE.code())
            ),
            "DNSSEC_BOGUS" => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSSEC_BOGUS.name(),
                                   EXTERRORS::DNSSEC_BOGUS.code())
            ),
            "SIGNATURE_EXPIRED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_EXPIRED.name(),
                                   EXTERRORS::SIGNATURE_EXPIRED.code())
            ),
            "SIGNATURE_NOT_YET_VALID" => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_NOT_YET_VALID.name(),
                                   EXTERRORS::SIGNATURE_NOT_YET_VALID.code())
            ),
            "DNSKEY_MISSING" => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSKEY_MISSING.name(),
                                   EXTERRORS::DNSKEY_MISSING.code())
            ),
            "RRSIGS_MISSING" => Ok(
                EXTERRORSInfo::new(EXTERRORS::RRSIGS_MISSING.name(),
                                   EXTERRORS::RRSIGS_MISSING.code())
            ),
            "NO_ZONE_KEY_BIT_SET" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NO_ZONE_KEY_BIT_SET.name(),
                                   EXTERRORS::NO_ZONE_KEY_BIT_SET.code())
            ),
            "NSEC_MISSING" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NSEC_MISSING.name(),
                                   EXTERRORS::NSEC_MISSING.code())
            ),
            "CACHED_ERROR" => Ok(
                EXTERRORSInfo::new(EXTERRORS::CACHED_ERROR.name(),
                                   EXTERRORS::CACHED_ERROR.code())
            ),
            "NOT_READY" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_READY.name(),
                                   EXTERRORS::NOT_READY.code())
            ),
            "BLOCKED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::BLOCKED.name(),
                                   EXTERRORS::BLOCKED.code())
            ),
            "CENSORED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::CENSORED.name(),
                                   EXTERRORS::CENSORED.code())
            ),
            "FILTERED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::FILTERED.name(),
                                   EXTERRORS::FILTERED.code())
            ),
            "PROHIBITED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::PROHIBITED.name(),
                                   EXTERRORS::PROHIBITED.code())
            ),
            "STALE_NXDOMAIN_ANSWER" => Ok(
                EXTERRORSInfo::new(EXTERRORS::STALE_NXDOMAIN_ANSWER.name(),
                                   EXTERRORS::STALE_NXDOMAIN_ANSWER.code())
            ),
            "NOT_AUTHORITATIVE" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_AUTHORITATIVE.name(),
                                   EXTERRORS::NOT_AUTHORITATIVE.code())
            ),
            "NOT_SUPPORTED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_SUPPORTED.name(),
                                   EXTERRORS::NOT_SUPPORTED.code())
            ),
            "NO_REACHABLE_AUTHORITY" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NO_REACHABLE_AUTHORITY.name(),
                                   EXTERRORS::NO_REACHABLE_AUTHORITY.code())
            ),
            "NETWORK_ERROR" => Ok(
                EXTERRORSInfo::new(EXTERRORS::NETWORK_ERROR.name(),
                                   EXTERRORS::NETWORK_ERROR.code())
            ),
            "INVALID_DATA" => Ok(
                EXTERRORSInfo::new(EXTERRORS::INVALID_DATA.name(),
                                   EXTERRORS::INVALID_DATA.code())
            ),
            "SIGNATURE_EXPIRED_BEFORE_VALID" => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID.name(),
                                   EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID.code())
            ),
            "TOO_EARLY" => Ok(
                EXTERRORSInfo::new(EXTERRORS::TOO_EARLY.name(),
                                   EXTERRORS::TOO_EARLY.code())
            ),
            "UNSUPPORTED_NSEC3_ITERATIONS_VALUE" => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE.name(),
                                   EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE.code())
            ),
            "UNABLE_TO_CONFORM_TO_POLICY" => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY.name(),
                                   EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY.code())
            ),
            "SYNTHESIZED" => Ok(
                EXTERRORSInfo::new(EXTERRORS::SYNTHESIZED.name(),
                                   EXTERRORS::SYNTHESIZED.code())
            ),
            _ => Err(String::from("Can't encode Extended DNS Error Code!"))
        }
    }

    fn decode(decimal: &u16) -> Result<EXTERRORSInfo, String> {
        return match *decimal {
            0 => Ok(
                EXTERRORSInfo::new(EXTERRORS::OTHER_ERROR.name(),
                                   EXTERRORS::OTHER_ERROR.code())
            ),
            1 => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM.name(),
                                   EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM.code())
            ),
            2 => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE.name(),
                                   EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE.code())
            ),
            3 => Ok(
                EXTERRORSInfo::new(EXTERRORS::STALE_ANSWER.name(),
                                   EXTERRORS::STALE_ANSWER.code())
            ),
            4 => Ok(
                EXTERRORSInfo::new(EXTERRORS::FORGED_ANSWER.name(),
                                   EXTERRORS::FORGED_ANSWER.code())
            ),
            5 => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSSEC_INDETERMINATE.name(),
                                   EXTERRORS::DNSSEC_INDETERMINATE.code())
            ),
            6 => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSSEC_BOGUS.name(),
                                   EXTERRORS::DNSSEC_BOGUS.code())
            ),
            7 => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_EXPIRED.name(),
                                   EXTERRORS::SIGNATURE_EXPIRED.code())
            ),
            8 => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_NOT_YET_VALID.name(),
                                   EXTERRORS::SIGNATURE_NOT_YET_VALID.code())
            ),
            9 => Ok(
                EXTERRORSInfo::new(EXTERRORS::DNSKEY_MISSING.name(),
                                   EXTERRORS::DNSKEY_MISSING.code())
            ),
            10 => Ok(
                EXTERRORSInfo::new(EXTERRORS::RRSIGS_MISSING.name(),
                                   EXTERRORS::RRSIGS_MISSING.code())
            ),
            11 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NO_ZONE_KEY_BIT_SET.name(),
                                   EXTERRORS::NO_ZONE_KEY_BIT_SET.code())
            ),
            12 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NSEC_MISSING.name(),
                                   EXTERRORS::NSEC_MISSING.code())
            ),
            13 => Ok(
                EXTERRORSInfo::new(EXTERRORS::CACHED_ERROR.name(),
                                   EXTERRORS::CACHED_ERROR.code())
            ),
            14 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_READY.name(),
                                   EXTERRORS::NOT_READY.code())
            ),
            15 => Ok(
                EXTERRORSInfo::new(EXTERRORS::BLOCKED.name(),
                                   EXTERRORS::BLOCKED.code())
            ),
            16 => Ok(
                EXTERRORSInfo::new(EXTERRORS::CENSORED.name(),
                                   EXTERRORS::CENSORED.code())
            ),
            17 => Ok(
                EXTERRORSInfo::new(EXTERRORS::FILTERED.name(),
                                   EXTERRORS::FILTERED.code())
            ),
            18 => Ok(
                EXTERRORSInfo::new(EXTERRORS::PROHIBITED.name(),
                                   EXTERRORS::PROHIBITED.code())
            ),
            19 => Ok(
                EXTERRORSInfo::new(EXTERRORS::STALE_NXDOMAIN_ANSWER.name(),
                                   EXTERRORS::STALE_NXDOMAIN_ANSWER.code())
            ),
            20 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_AUTHORITATIVE.name(),
                                   EXTERRORS::NOT_AUTHORITATIVE.code())
            ),
            21 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NOT_SUPPORTED.name(),
                                   EXTERRORS::NOT_SUPPORTED.code())
            ),
            22 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NO_REACHABLE_AUTHORITY.name(),
                                   EXTERRORS::NO_REACHABLE_AUTHORITY.code())
            ),
            23 => Ok(
                EXTERRORSInfo::new(EXTERRORS::NETWORK_ERROR.name(),
                                   EXTERRORS::NETWORK_ERROR.code())
            ),
            24 => Ok(
                EXTERRORSInfo::new(EXTERRORS::INVALID_DATA.name(),
                                   EXTERRORS::INVALID_DATA.code())
            ),
            25 => Ok(
                EXTERRORSInfo::new(EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID.name(),
                                   EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID.code())
            ),
            26 => Ok(
                EXTERRORSInfo::new(EXTERRORS::TOO_EARLY.name(),
                                   EXTERRORS::TOO_EARLY.code())
            ),
            27 => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE.name(),
                                   EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE.code())
            ),
            28 => Ok(
                EXTERRORSInfo::new(EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY.name(),
                                   EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY.code())
            ),
            29 => Ok(
                EXTERRORSInfo::new(EXTERRORS::SYNTHESIZED.name(),
                                   EXTERRORS::SYNTHESIZED.code())
            ),
            30..=49151 => Ok(
                EXTERRORSInfo::new(UNASSIGNED,
                                   *decimal)
            ),
            49152..=65535 => Ok(
                EXTERRORSInfo::new(RESERVED_PRIVATE_USE,
                                   *decimal)
            ),
            _ => Err(String::from("Can't decode Extended DNS Error Code!"))
        }
    }
}
