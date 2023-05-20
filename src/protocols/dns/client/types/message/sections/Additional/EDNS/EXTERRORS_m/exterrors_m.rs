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

pub enum EXTERRORS {
    OTHER_ERROR = 0,
    UNSUPPORTED_DNSKEY_ALGORITHM = 1,
    UNSUPPORTED_DS_DIGEST_TYPE = 2,
    STALE_ANSWER = 3,
    FORGED_ANSWER = 4,
    DNSSEC_INDETERMINATE = 5,
    DNSSEC_BOGUS = 6,
    SIGNATURE_EXPIRED = 7,
    SIGNATURE_NOT_YET_VALID = 8,
    DNSKEY_MISSING = 9,
    RRSIGS_MISSING = 10,
    NO_ZONE_KEY_BIT_SET = 11,
    NSEC_MISSING = 12,
    CACHED_ERROR = 13,
    NOT_READY = 14,
    BLOCKED = 15,
    CENSORED = 16,
    FILTERED = 17,
    PROHIBITED = 18,
    STALE_NXDOMAIN_ANSWER = 19,
    NOT_AUTHORITATIVE = 20,
    NOT_SUPPORTED = 21,
    NO_REACHABLE_AUTHORITY = 22,
    NETWORK_ERROR = 23,
    INVALID_DATA = 24,
    SIGNATURE_EXPIRED_BEFORE_VALID = 25,
    TOO_EARLY = 26,
    UNSUPPORTED_NSEC3_ITERATIONS_VALUE = 27,
    UNABLE_TO_CONFORM_TO_POLICY = 28,
    SYNTHESIZED = 29
}

impl EXTERRORS {
    pub fn code(&self) -> u16 {
        return match self {
            EXTERRORS::OTHER_ERROR => EXTERRORS::OTHER_ERROR as u16,
            EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM => EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM as u16,
            EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE => EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE as u16,
            EXTERRORS::STALE_ANSWER => EXTERRORS::STALE_ANSWER as u16,
            EXTERRORS::FORGED_ANSWER => EXTERRORS::FORGED_ANSWER as u16,
            EXTERRORS::DNSSEC_INDETERMINATE => EXTERRORS::DNSSEC_INDETERMINATE as u16,
            EXTERRORS::DNSSEC_BOGUS => EXTERRORS::DNSSEC_BOGUS as u16,
            EXTERRORS::SIGNATURE_EXPIRED => EXTERRORS::SIGNATURE_EXPIRED as u16,
            EXTERRORS::SIGNATURE_NOT_YET_VALID => EXTERRORS::SIGNATURE_NOT_YET_VALID as u16,
            EXTERRORS::DNSKEY_MISSING => EXTERRORS::DNSKEY_MISSING as u16,
            EXTERRORS::RRSIGS_MISSING => EXTERRORS::RRSIGS_MISSING as u16,
            EXTERRORS::NO_ZONE_KEY_BIT_SET => EXTERRORS::NO_ZONE_KEY_BIT_SET as u16,
            EXTERRORS::NSEC_MISSING => EXTERRORS::NSEC_MISSING as u16,
            EXTERRORS::CACHED_ERROR => EXTERRORS::CACHED_ERROR as u16,
            EXTERRORS::NOT_READY => EXTERRORS::NOT_READY as u16,
            EXTERRORS::BLOCKED => EXTERRORS::BLOCKED as u16,
            EXTERRORS::CENSORED => EXTERRORS::CENSORED as u16,
            EXTERRORS::FILTERED => EXTERRORS::FILTERED as u16,
            EXTERRORS::PROHIBITED => EXTERRORS::PROHIBITED as u16,
            EXTERRORS::STALE_NXDOMAIN_ANSWER => EXTERRORS::STALE_NXDOMAIN_ANSWER as u16,
            EXTERRORS::NOT_AUTHORITATIVE => EXTERRORS::NOT_AUTHORITATIVE as u16,
            EXTERRORS::NOT_SUPPORTED => EXTERRORS::NOT_SUPPORTED as u16,
            EXTERRORS::NO_REACHABLE_AUTHORITY => EXTERRORS::NO_REACHABLE_AUTHORITY as u16,
            EXTERRORS::NETWORK_ERROR => EXTERRORS::NETWORK_ERROR as u16,
            EXTERRORS::INVALID_DATA => EXTERRORS::INVALID_DATA as u16,
            EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID => EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID as u16,
            EXTERRORS::TOO_EARLY => EXTERRORS::TOO_EARLY as u16,
            EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE => EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE as u16,
            EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY => EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY as u16,
            EXTERRORS::SYNTHESIZED => EXTERRORS::SYNTHESIZED as u16,
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            EXTERRORS::OTHER_ERROR => "Other Error",
            EXTERRORS::UNSUPPORTED_DNSKEY_ALGORITHM => "Unsupported DNSKEY Algorithm",
            EXTERRORS::UNSUPPORTED_DS_DIGEST_TYPE => "Unsupported DS Digest Type",
            EXTERRORS::STALE_ANSWER => "Stale Answer",
            EXTERRORS::FORGED_ANSWER => "Forged Answer",
            EXTERRORS::DNSSEC_INDETERMINATE => "DNSSEC Indeterminate",
            EXTERRORS::DNSSEC_BOGUS => "DNSSEC Bogus",
            EXTERRORS::SIGNATURE_EXPIRED => "Signature Expired",
            EXTERRORS::SIGNATURE_NOT_YET_VALID => "Signature Not Yet Valid",
            EXTERRORS::DNSKEY_MISSING => "DNSKEY Missing",
            EXTERRORS::RRSIGS_MISSING => "RRSIGs Missing",
            EXTERRORS::NO_ZONE_KEY_BIT_SET => "No Zone Key Bit Set",
            EXTERRORS::NSEC_MISSING => "NSEC Missing",
            EXTERRORS::CACHED_ERROR => "Cached Error",
            EXTERRORS::NOT_READY => "Not Ready",
            EXTERRORS::BLOCKED => "Blocked",
            EXTERRORS::CENSORED => "Censored",
            EXTERRORS::FILTERED => "Filtered",
            EXTERRORS::PROHIBITED => "Prohibited",
            EXTERRORS::STALE_NXDOMAIN_ANSWER => "Stale NXDomain Answer",
            EXTERRORS::NOT_AUTHORITATIVE => "Not Authoritative",
            EXTERRORS::NOT_SUPPORTED => "Not Supported",
            EXTERRORS::NO_REACHABLE_AUTHORITY => "No Reachable Authority",
            EXTERRORS::NETWORK_ERROR => "Network Error",
            EXTERRORS::INVALID_DATA => "Invalid Data",
            EXTERRORS::SIGNATURE_EXPIRED_BEFORE_VALID => "Signature Expired before Valid",
            EXTERRORS::TOO_EARLY => "Too Early",
            EXTERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE => "Unsupported NSEC3 Iterations Value",
            EXTERRORS::UNABLE_TO_CONFORM_TO_POLICY => "Unable to conform to policy",
            EXTERRORS::SYNTHESIZED => "Synthesized",
        }
    }
}
