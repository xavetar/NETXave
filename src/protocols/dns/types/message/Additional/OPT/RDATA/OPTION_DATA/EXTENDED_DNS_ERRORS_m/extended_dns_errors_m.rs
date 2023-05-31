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

pub enum EXTENDED_DNS_ERRORS {
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

impl EXTENDED_DNS_ERRORS {
    pub fn code(&self) -> u16 {
        return match self {
            EXTENDED_DNS_ERRORS::OTHER_ERROR => EXTENDED_DNS_ERRORS::OTHER_ERROR as u16,
            EXTENDED_DNS_ERRORS::UNSUPPORTED_DNSKEY_ALGORITHM => EXTENDED_DNS_ERRORS::UNSUPPORTED_DNSKEY_ALGORITHM as u16,
            EXTENDED_DNS_ERRORS::UNSUPPORTED_DS_DIGEST_TYPE => EXTENDED_DNS_ERRORS::UNSUPPORTED_DS_DIGEST_TYPE as u16,
            EXTENDED_DNS_ERRORS::STALE_ANSWER => EXTENDED_DNS_ERRORS::STALE_ANSWER as u16,
            EXTENDED_DNS_ERRORS::FORGED_ANSWER => EXTENDED_DNS_ERRORS::FORGED_ANSWER as u16,
            EXTENDED_DNS_ERRORS::DNSSEC_INDETERMINATE => EXTENDED_DNS_ERRORS::DNSSEC_INDETERMINATE as u16,
            EXTENDED_DNS_ERRORS::DNSSEC_BOGUS => EXTENDED_DNS_ERRORS::DNSSEC_BOGUS as u16,
            EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED => EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED as u16,
            EXTENDED_DNS_ERRORS::SIGNATURE_NOT_YET_VALID => EXTENDED_DNS_ERRORS::SIGNATURE_NOT_YET_VALID as u16,
            EXTENDED_DNS_ERRORS::DNSKEY_MISSING => EXTENDED_DNS_ERRORS::DNSKEY_MISSING as u16,
            EXTENDED_DNS_ERRORS::RRSIGS_MISSING => EXTENDED_DNS_ERRORS::RRSIGS_MISSING as u16,
            EXTENDED_DNS_ERRORS::NO_ZONE_KEY_BIT_SET => EXTENDED_DNS_ERRORS::NO_ZONE_KEY_BIT_SET as u16,
            EXTENDED_DNS_ERRORS::NSEC_MISSING => EXTENDED_DNS_ERRORS::NSEC_MISSING as u16,
            EXTENDED_DNS_ERRORS::CACHED_ERROR => EXTENDED_DNS_ERRORS::CACHED_ERROR as u16,
            EXTENDED_DNS_ERRORS::NOT_READY => EXTENDED_DNS_ERRORS::NOT_READY as u16,
            EXTENDED_DNS_ERRORS::BLOCKED => EXTENDED_DNS_ERRORS::BLOCKED as u16,
            EXTENDED_DNS_ERRORS::CENSORED => EXTENDED_DNS_ERRORS::CENSORED as u16,
            EXTENDED_DNS_ERRORS::FILTERED => EXTENDED_DNS_ERRORS::FILTERED as u16,
            EXTENDED_DNS_ERRORS::PROHIBITED => EXTENDED_DNS_ERRORS::PROHIBITED as u16,
            EXTENDED_DNS_ERRORS::STALE_NXDOMAIN_ANSWER => EXTENDED_DNS_ERRORS::STALE_NXDOMAIN_ANSWER as u16,
            EXTENDED_DNS_ERRORS::NOT_AUTHORITATIVE => EXTENDED_DNS_ERRORS::NOT_AUTHORITATIVE as u16,
            EXTENDED_DNS_ERRORS::NOT_SUPPORTED => EXTENDED_DNS_ERRORS::NOT_SUPPORTED as u16,
            EXTENDED_DNS_ERRORS::NO_REACHABLE_AUTHORITY => EXTENDED_DNS_ERRORS::NO_REACHABLE_AUTHORITY as u16,
            EXTENDED_DNS_ERRORS::NETWORK_ERROR => EXTENDED_DNS_ERRORS::NETWORK_ERROR as u16,
            EXTENDED_DNS_ERRORS::INVALID_DATA => EXTENDED_DNS_ERRORS::INVALID_DATA as u16,
            EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED_BEFORE_VALID => EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED_BEFORE_VALID as u16,
            EXTENDED_DNS_ERRORS::TOO_EARLY => EXTENDED_DNS_ERRORS::TOO_EARLY as u16,
            EXTENDED_DNS_ERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE => EXTENDED_DNS_ERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE as u16,
            EXTENDED_DNS_ERRORS::UNABLE_TO_CONFORM_TO_POLICY => EXTENDED_DNS_ERRORS::UNABLE_TO_CONFORM_TO_POLICY as u16,
            EXTENDED_DNS_ERRORS::SYNTHESIZED => EXTENDED_DNS_ERRORS::SYNTHESIZED as u16,
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            EXTENDED_DNS_ERRORS::OTHER_ERROR => "Other Error",
            EXTENDED_DNS_ERRORS::UNSUPPORTED_DNSKEY_ALGORITHM => "Unsupported DNSKEY Algorithm",
            EXTENDED_DNS_ERRORS::UNSUPPORTED_DS_DIGEST_TYPE => "Unsupported DS Digest Type",
            EXTENDED_DNS_ERRORS::STALE_ANSWER => "Stale Answer",
            EXTENDED_DNS_ERRORS::FORGED_ANSWER => "Forged Answer",
            EXTENDED_DNS_ERRORS::DNSSEC_INDETERMINATE => "DNSSEC Indeterminate",
            EXTENDED_DNS_ERRORS::DNSSEC_BOGUS => "DNSSEC Bogus",
            EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED => "Signature Expired",
            EXTENDED_DNS_ERRORS::SIGNATURE_NOT_YET_VALID => "Signature Not Yet Valid",
            EXTENDED_DNS_ERRORS::DNSKEY_MISSING => "DNSKEY Missing",
            EXTENDED_DNS_ERRORS::RRSIGS_MISSING => "RRSIGs Missing",
            EXTENDED_DNS_ERRORS::NO_ZONE_KEY_BIT_SET => "No Zone Key Bit Set",
            EXTENDED_DNS_ERRORS::NSEC_MISSING => "NSEC Missing",
            EXTENDED_DNS_ERRORS::CACHED_ERROR => "Cached Error",
            EXTENDED_DNS_ERRORS::NOT_READY => "Not Ready",
            EXTENDED_DNS_ERRORS::BLOCKED => "Blocked",
            EXTENDED_DNS_ERRORS::CENSORED => "Censored",
            EXTENDED_DNS_ERRORS::FILTERED => "Filtered",
            EXTENDED_DNS_ERRORS::PROHIBITED => "Prohibited",
            EXTENDED_DNS_ERRORS::STALE_NXDOMAIN_ANSWER => "Stale NXDomain Answer",
            EXTENDED_DNS_ERRORS::NOT_AUTHORITATIVE => "Not Authoritative",
            EXTENDED_DNS_ERRORS::NOT_SUPPORTED => "Not Supported",
            EXTENDED_DNS_ERRORS::NO_REACHABLE_AUTHORITY => "No Reachable Authority",
            EXTENDED_DNS_ERRORS::NETWORK_ERROR => "Network Error",
            EXTENDED_DNS_ERRORS::INVALID_DATA => "Invalid Data",
            EXTENDED_DNS_ERRORS::SIGNATURE_EXPIRED_BEFORE_VALID => "Signature Expired before Valid",
            EXTENDED_DNS_ERRORS::TOO_EARLY => "Too Early",
            EXTENDED_DNS_ERRORS::UNSUPPORTED_NSEC3_ITERATIONS_VALUE => "Unsupported NSEC3 Iterations Value",
            EXTENDED_DNS_ERRORS::UNABLE_TO_CONFORM_TO_POLICY => "Unable to conform to policy",
            EXTENDED_DNS_ERRORS::SYNTHESIZED => "Synthesized",
        }
    }
}
