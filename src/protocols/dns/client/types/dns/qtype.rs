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

pub const REVERSED: &'static str = "Reversed";
pub const UNASSIGNED: &'static str = "Unassigned";
pub const PRIVATE_USE: &'static str = "Private Use";

pub struct QTYPEInfo {
    name: &'static str,
    code: u16
}

impl QTYPEInfo {
    pub fn name(&self) -> &'static str {
        return &self.name;
    }

    pub fn code(&self) -> &u16 {
        return &self.code;
    }

    pub fn hex(&self) -> String {
        return format!("{:02x}", &self.code);
    }
}

pub enum QTYPE {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    RP = 17,
    AFSDB = 18,
    X25 = 19,
    ISDN = 20,
    RT = 21,
    NSAP = 22,
    NSAP_PTR = 23,
    SIG = 24,
    KEY = 25,
    PX = 26,
    GPOS = 27,
    AAAA = 28,
    LOC = 29,
    NXT = 30,
    EID = 31,
    NIMLOC = 32,
    SRV = 33,
    ATMA = 34,
    NAPTR = 35,
    KX = 36,
    CERT = 37,
    A6 = 38,
    DNAME = 39,
    SINK = 40,
    OPT = 41,
    APL = 42,
    DS = 43,
    SSHFP = 44,
    IPSECKEY = 45,
    RRSIG = 46,
    NSEC = 47,
    DNSKEY = 48,
    DHCID = 49,
    NSEC3 = 50,
    NSEC3PARAM = 51,
    TLSA = 52,
    SMIMEA = 53,
    HIP = 55,
    NINFO = 56,
    RKEY = 57,
    TALINK = 58,
    CDS = 59,
    CDNSKEY = 60,
    OPENPGPKEY = 61,
    CSYNC = 62,
    ZONEMD = 63,
    SVCB = 64,
    HTTPS = 65,
    SPF = 99,
    UINFO = 100,
    UID = 101,
    GID = 102,
    UNSPEC = 103,
    NID = 104,
    L32 = 105,
    L64 = 106,
    LP = 107,
    EUI48 = 108,
    EUI64 = 109,
    TKEY = 249,
    TSIG = 250,
    IXFR = 251,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,
    URI = 256,
    CAA = 257,
    AVC = 258,
    DOA = 259,
    AMTRELAY = 260,
    TA = 32768,
    DLV = 32769,
}

impl QTYPE {
    pub fn name(&self) -> &'static str {
        return match self {
            QTYPE::A => "A",
            QTYPE::NS => "NS",
            QTYPE::MD => "MD",
            QTYPE::MF => "MF",
            QTYPE::CNAME => "CNAME",
            QTYPE::SOA => "SOA",
            QTYPE::MB => "MB",
            QTYPE::MG => "MG",
            QTYPE::MR => "MR",
            QTYPE::NULL => "NULL",
            QTYPE::WKS => "WKS",
            QTYPE::PTR => "PTR",
            QTYPE::HINFO => "HINFO",
            QTYPE::MINFO => "MINFO",
            QTYPE::MX => "MX",
            QTYPE::TXT => "TXT",
            QTYPE::RP => "RP",
            QTYPE::AFSDB => "AFSDB",
            QTYPE::X25 => "X25",
            QTYPE::ISDN => "ISDN",
            QTYPE::RT => "RT",
            QTYPE::NSAP => "NSAP",
            QTYPE::NSAP_PTR => "NSAP-PTR",
            QTYPE::SIG => "SIG",
            QTYPE::KEY => "KEY",
            QTYPE::PX => "PX",
            QTYPE::GPOS => "GPOS",
            QTYPE::AAAA => "AAAA",
            QTYPE::LOC => "LOC",
            QTYPE::NXT => "NXT",
            QTYPE::EID => "EID",
            QTYPE::NIMLOC => "NIMLOC",
            QTYPE::SRV => "SRV",
            QTYPE::ATMA => "ATMA",
            QTYPE::NAPTR => "NAPTR",
            QTYPE::KX => "KX",
            QTYPE::CERT => "CERT",
            QTYPE::A6 => "A6",
            QTYPE::DNAME => "DNAME",
            QTYPE::SINK => "SINK",
            QTYPE::OPT => "OPT",
            QTYPE::APL => "APL",
            QTYPE::DS => "DS",
            QTYPE::SSHFP => "SSHFP",
            QTYPE::IPSECKEY => "IPSECKEY",
            QTYPE::RRSIG => "RRSIG",
            QTYPE::NSEC => "NSEC",
            QTYPE::DNSKEY => "DNSKEY",
            QTYPE::DHCID => "DHCID",
            QTYPE::NSEC3 => "NSEC3",
            QTYPE::NSEC3PARAM => "NSEC3PARAM",
            QTYPE::TLSA => "TLSA",
            QTYPE::SMIMEA => "SMIMEA",
            QTYPE::HIP => "HIP",
            QTYPE::NINFO => "NINFO",
            QTYPE::RKEY => "RKEY",
            QTYPE::TALINK => "TALINK",
            QTYPE::CDS => "CDS",
            QTYPE::CDNSKEY => "CDNSKEY",
            QTYPE::OPENPGPKEY => "OPENPGPKEY",
            QTYPE::CSYNC => "CSYNC",
            QTYPE::ZONEMD => "ZONEMD",
            QTYPE::SVCB => "SVCB",
            QTYPE::HTTPS => "HTTPS",
            QTYPE::SPF => "SPF",
            QTYPE::UINFO => "UINFO",
            QTYPE::UID => "UID",
            QTYPE::GID => "GID",
            QTYPE::UNSPEC => "UNSPEC",
            QTYPE::NID => "NID",
            QTYPE::L32 => "L32",
            QTYPE::L64 => "L64",
            QTYPE::LP => "LP",
            QTYPE::EUI48 => "EUI48",
            QTYPE::EUI64 => "EUI64",
            QTYPE::TKEY => "TKEY",
            QTYPE::TSIG => "TSIG",
            QTYPE::IXFR => "IXFR",
            QTYPE::AXFR => "AXFR",
            QTYPE::MAILB => "MAILB",
            QTYPE::MAILA => "MAILA",
            QTYPE::ANY => "*",
            QTYPE::URI => "URI",
            QTYPE::CAA => "CAA",
            QTYPE::AVC => "AVC",
            QTYPE::DOA => "DOA",
            QTYPE::AMTRELAY => "AMTRELAY",
            QTYPE::TA => "TA",
            QTYPE::DLV => "DLV"
        }
    }

    pub fn code(&self) -> u16 {
        return match self {
            QTYPE::A => QTYPE::A as u16,
            QTYPE::NS => QTYPE::NS as u16,
            QTYPE::MD => QTYPE::MD as u16,
            QTYPE::MF => QTYPE::MF as u16,
            QTYPE::CNAME => QTYPE::CNAME as u16,
            QTYPE::SOA => QTYPE::SOA as u16,
            QTYPE::MB => QTYPE::MB as u16,
            QTYPE::MG => QTYPE::MG as u16,
            QTYPE::MR => QTYPE::MR as u16,
            QTYPE::NULL => QTYPE::NULL as u16,
            QTYPE::WKS => QTYPE::WKS as u16,
            QTYPE::PTR => QTYPE::PTR as u16,
            QTYPE::HINFO => QTYPE::HINFO as u16,
            QTYPE::MINFO => QTYPE::MINFO as u16,
            QTYPE::MX => QTYPE::MX as u16,
            QTYPE::TXT => QTYPE::TXT as u16,
            QTYPE::RP => QTYPE::RP as u16,
            QTYPE::AFSDB => QTYPE::AFSDB as u16,
            QTYPE::X25 => QTYPE::X25 as u16,
            QTYPE::ISDN => QTYPE::ISDN as u16,
            QTYPE::RT => QTYPE::RT as u16,
            QTYPE::NSAP => QTYPE::NSAP as u16,
            QTYPE::NSAP_PTR => QTYPE::NSAP_PTR as u16,
            QTYPE::SIG => QTYPE::SIG as u16,
            QTYPE::KEY => QTYPE::KEY as u16,
            QTYPE::PX => QTYPE::PX as u16,
            QTYPE::GPOS => QTYPE::GPOS as u16,
            QTYPE::AAAA => QTYPE::AAAA as u16,
            QTYPE::LOC => QTYPE::LOC as u16,
            QTYPE::NXT => QTYPE::NXT as u16,
            QTYPE::EID => QTYPE::EID as u16,
            QTYPE::NIMLOC => QTYPE::NIMLOC as u16,
            QTYPE::SRV => QTYPE::SRV as u16,
            QTYPE::ATMA => QTYPE::ATMA as u16,
            QTYPE::NAPTR => QTYPE::NAPTR as u16,
            QTYPE::KX => QTYPE::KX as u16,
            QTYPE::CERT => QTYPE::CERT as u16,
            QTYPE::A6 => QTYPE::A6 as u16,
            QTYPE::DNAME => QTYPE::DNAME as u16,
            QTYPE::SINK => QTYPE::SINK as u16,
            QTYPE::OPT => QTYPE::OPT as u16,
            QTYPE::APL => QTYPE::APL as u16,
            QTYPE::DS => QTYPE::DS as u16,
            QTYPE::SSHFP => QTYPE::SSHFP as u16,
            QTYPE::IPSECKEY => QTYPE::IPSECKEY as u16,
            QTYPE::RRSIG => QTYPE::RRSIG as u16,
            QTYPE::NSEC => QTYPE::NSEC as u16,
            QTYPE::DNSKEY => QTYPE::DNSKEY as u16,
            QTYPE::DHCID => QTYPE::DHCID as u16,
            QTYPE::NSEC3 => QTYPE::NSEC3 as u16,
            QTYPE::NSEC3PARAM => QTYPE::NSEC3PARAM as u16,
            QTYPE::TLSA => QTYPE::TLSA as u16,
            QTYPE::SMIMEA => QTYPE::SMIMEA as u16,
            QTYPE::HIP => QTYPE::HIP as u16,
            QTYPE::NINFO => QTYPE::NINFO as u16,
            QTYPE::RKEY => QTYPE::RKEY as u16,
            QTYPE::TALINK => QTYPE::TALINK as u16,
            QTYPE::CDS => QTYPE::CDS as u16,
            QTYPE::CDNSKEY => QTYPE::CDNSKEY as u16,
            QTYPE::OPENPGPKEY => QTYPE::OPENPGPKEY as u16,
            QTYPE::CSYNC => QTYPE::CSYNC as u16,
            QTYPE::ZONEMD => QTYPE::ZONEMD as u16,
            QTYPE::SVCB => QTYPE::SVCB as u16,
            QTYPE::HTTPS => QTYPE::HTTPS as u16,
            QTYPE::SPF => QTYPE::SPF as u16,
            QTYPE::UINFO => QTYPE::UINFO as u16,
            QTYPE::UID => QTYPE::UID as u16,
            QTYPE::GID => QTYPE::GID as u16,
            QTYPE::UNSPEC => QTYPE::UNSPEC as u16,
            QTYPE::NID => QTYPE::NID as u16,
            QTYPE::L32 => QTYPE::L32 as u16,
            QTYPE::L64 => QTYPE::L64 as u16,
            QTYPE::LP => QTYPE::LP as u16,
            QTYPE::EUI48 => QTYPE::EUI48 as u16,
            QTYPE::EUI64 => QTYPE::EUI64 as u16,
            QTYPE::TKEY => QTYPE::TKEY as u16,
            QTYPE::TSIG => QTYPE::TSIG as u16,
            QTYPE::IXFR => QTYPE::IXFR as u16,
            QTYPE::AXFR => QTYPE::AXFR as u16,
            QTYPE::MAILB => QTYPE::MAILB as u16,
            QTYPE::MAILA => QTYPE::MAILA as u16,
            QTYPE::ANY => QTYPE::ANY as u16,
            QTYPE::URI => QTYPE::URI as u16,
            QTYPE::CAA => QTYPE::CAA as u16,
            QTYPE::AVC => QTYPE::AVC as u16,
            QTYPE::DOA => QTYPE::DOA as u16,
            QTYPE::AMTRELAY => QTYPE::AMTRELAY as u16,
            QTYPE::TA => QTYPE::TA as u16,
            QTYPE::DLV => QTYPE::DLV as u16,
        }
    }

    pub fn hex(&self) -> String {
        return match self {
            QTYPE::A => format!("{:02x}", QTYPE::A.code()),
            QTYPE::NS => format!("{:02x}", QTYPE::NS.code()),
            QTYPE::MD => format!("{:02x}", QTYPE::MD.code()),
            QTYPE::MF => format!("{:02x}", QTYPE::MF.code()),
            QTYPE::CNAME => format!("{:02x}", QTYPE::CNAME.code()),
            QTYPE::SOA => format!("{:02x}", QTYPE::SOA.code()),
            QTYPE::MB => format!("{:02x}", QTYPE::MB.code()),
            QTYPE::MG => format!("{:02x}", QTYPE::MG.code()),
            QTYPE::MR => format!("{:02x}", QTYPE::MR.code()),
            QTYPE::NULL => format!("{:02x}", QTYPE::NULL.code()),
            QTYPE::WKS => format!("{:02x}", QTYPE::WKS.code()),
            QTYPE::PTR => format!("{:02x}", QTYPE::PTR.code()),
            QTYPE::HINFO => format!("{:02x}", QTYPE::HINFO.code()),
            QTYPE::MINFO => format!("{:02x}", QTYPE::MINFO.code()),
            QTYPE::MX => format!("{:02x}", QTYPE::MX.code()),
            QTYPE::TXT => format!("{:02x}", QTYPE::TXT.code()),
            QTYPE::RP => format!("{:02x}", QTYPE::RP.code()),
            QTYPE::AFSDB => format!("{:02x}", QTYPE::AFSDB.code()),
            QTYPE::X25 => format!("{:02x}", QTYPE::X25.code()),
            QTYPE::ISDN => format!("{:02x}", QTYPE::ISDN.code()),
            QTYPE::RT => format!("{:02x}", QTYPE::RT.code()),
            QTYPE::NSAP => format!("{:02x}", QTYPE::NSAP.code()),
            QTYPE::NSAP_PTR => format!("{:02x}", QTYPE::NSAP_PTR.code()),
            QTYPE::SIG => format!("{:02x}", QTYPE::SIG.code()),
            QTYPE::KEY => format!("{:02x}", QTYPE::KEY.code()),
            QTYPE::PX => format!("{:02x}", QTYPE::PX.code()),
            QTYPE::GPOS => format!("{:02x}", QTYPE::GPOS.code()),
            QTYPE::AAAA => format!("{:02x}", QTYPE::AAAA.code()),
            QTYPE::LOC => format!("{:02x}", QTYPE::LOC.code()),
            QTYPE::NXT => format!("{:02x}", QTYPE::NXT.code()),
            QTYPE::EID => format!("{:02x}", QTYPE::EID.code()),
            QTYPE::NIMLOC => format!("{:02x}", QTYPE::NIMLOC.code()),
            QTYPE::SRV => format!("{:02x}", QTYPE::SRV.code()),
            QTYPE::ATMA => format!("{:02x}", QTYPE::ATMA.code()),
            QTYPE::NAPTR => format!("{:02x}", QTYPE::NAPTR.code()),
            QTYPE::KX => format!("{:02x}", QTYPE::KX.code()),
            QTYPE::CERT => format!("{:02x}", QTYPE::CERT.code()),
            QTYPE::A6 => format!("{:02x}", QTYPE::A6.code()),
            QTYPE::DNAME => format!("{:02x}", QTYPE::DNAME.code()),
            QTYPE::SINK => format!("{:02x}", QTYPE::SINK.code()),
            QTYPE::OPT => format!("{:02x}", QTYPE::OPT.code()),
            QTYPE::APL => format!("{:02x}", QTYPE::APL.code()),
            QTYPE::DS => format!("{:02x}", QTYPE::DS.code()),
            QTYPE::SSHFP => format!("{:02x}", QTYPE::SSHFP.code()),
            QTYPE::IPSECKEY => format!("{:02x}", QTYPE::IPSECKEY.code()),
            QTYPE::RRSIG => format!("{:02x}", QTYPE::RRSIG.code()),
            QTYPE::NSEC => format!("{:02x}", QTYPE::NSEC.code()),
            QTYPE::DNSKEY => format!("{:02x}", QTYPE::DNSKEY.code()),
            QTYPE::DHCID => format!("{:02x}", QTYPE::DHCID.code()),
            QTYPE::NSEC3 => format!("{:02x}", QTYPE::NSEC3.code()),
            QTYPE::NSEC3PARAM => format!("{:02x}", QTYPE::NSEC3PARAM.code()),
            QTYPE::TLSA => format!("{:02x}", QTYPE::TLSA.code()),
            QTYPE::SMIMEA => format!("{:02x}", QTYPE::SMIMEA.code()),
            QTYPE::HIP => format!("{:02x}", QTYPE::HIP.code()),
            QTYPE::NINFO => format!("{:02x}", QTYPE::MINFO.code()),
            QTYPE::RKEY => format!("{:02x}", QTYPE::RKEY.code()),
            QTYPE::TALINK => format!("{:02x}", QTYPE::TALINK.code()),
            QTYPE::CDS => format!("{:02x}", QTYPE::CDS.code()),
            QTYPE::CDNSKEY => format!("{:02x}", QTYPE::CDNSKEY.code()),
            QTYPE::OPENPGPKEY => format!("{:02x}", QTYPE::OPENPGPKEY.code()),
            QTYPE::CSYNC => format!("{:02x}", QTYPE::CSYNC.code()),
            QTYPE::ZONEMD => format!("{:02x}", QTYPE::ZONEMD.code()),
            QTYPE::SVCB => format!("{:02x}", QTYPE::SVCB.code()),
            QTYPE::HTTPS => format!("{:02x}", QTYPE::HTTPS.code()),
            QTYPE::SPF => format!("{:02x}", QTYPE::SPF.code()),
            QTYPE::UINFO => format!("{:02x}", QTYPE::UINFO.code()),
            QTYPE::UID => format!("{:02x}", QTYPE::UID.code()),
            QTYPE::GID => format!("{:02x}", QTYPE::GID.code()),
            QTYPE::UNSPEC => format!("{:02x}", QTYPE::UNSPEC.code()),
            QTYPE::NID => format!("{:02x}", QTYPE::NID.code()),
            QTYPE::L32 => format!("{:02x}", QTYPE::L32.code()),
            QTYPE::L64 => format!("{:02x}", QTYPE::L64.code()),
            QTYPE::LP => format!("{:02x}", QTYPE::LP.code()),
            QTYPE::EUI48 => format!("{:02x}", QTYPE::EUI48.code()),
            QTYPE::EUI64 => format!("{:02x}", QTYPE::EUI64.code()),
            QTYPE::TKEY => format!("{:02x}", QTYPE::TKEY.code()),
            QTYPE::TSIG => format!("{:02x}", QTYPE::TSIG.code()),
            QTYPE::IXFR => format!("{:02x}", QTYPE::IXFR.code()),
            QTYPE::AXFR => format!("{:02x}", QTYPE::AXFR.code()),
            QTYPE::MAILB => format!("{:02x}", QTYPE::MAILB.code()),
            QTYPE::MAILA => format!("{:02x}", QTYPE::MAILA.code()),
            QTYPE::ANY => format!("{:02x}", QTYPE::ANY.code()),
            QTYPE::URI => format!("{:02x}", QTYPE::URI.code()),
            QTYPE::CAA => format!("{:02x}", QTYPE::CAA.code()),
            QTYPE::AVC => format!("{:02x}", QTYPE::AVC.code()),
            QTYPE::DOA => format!("{:02x}", QTYPE::DOA.code()),
            QTYPE::AMTRELAY => format!("{:02x}", QTYPE::AMTRELAY.code()),
            QTYPE::TA => format!("{:02x}", QTYPE::TA.code()),
            QTYPE::DLV => format!("{:02x}", QTYPE::DLV.code()),
        }
    }
}

pub trait QTYPEConversion {
    fn encode(qclass: &str) -> Result<QTYPEInfo, String>;
    fn decode(dec: &u16) -> Result<QTYPEInfo, String>;
}

impl QTYPEConversion for QTYPE {
    fn encode(qtype: &str) -> Result<QTYPEInfo, String> {
        return match qtype {
            "A" => Ok(
                QTYPEInfo {
                    name: QTYPE::A.name(),
                    code: QTYPE::A.code()
                }
            ),
            "NS" => Ok(
                QTYPEInfo {
                    name: QTYPE::NS.name(),
                    code: QTYPE::NS.code()
                }
            ),
            "MD" => Ok(
                QTYPEInfo {
                    name: QTYPE::MD.name(),
                    code: QTYPE::MD.code()
                }
            ),
            "MF" => Ok(
                QTYPEInfo {
                    name: QTYPE::MF.name(),
                    code: QTYPE::MF.code()
                }
            ),
            "CNAME" => Ok(
                QTYPEInfo {
                    name: QTYPE::CNAME.name(),
                    code: QTYPE::CNAME.code()
                }
            ),
            "SOA" => Ok(
                QTYPEInfo {
                    name: QTYPE::SOA.name(),
                    code: QTYPE::SOA.code()
                }
            ),
            "MB" => Ok(
                QTYPEInfo {
                    name: QTYPE::MB.name(),
                    code: QTYPE::MB.code()
                }
            ),
            "MG" => Ok(
                QTYPEInfo {
                    name: QTYPE::MG.name(),
                    code: QTYPE::MG.code()
                }
            ),
            "MR" => Ok(
                QTYPEInfo {
                    name: QTYPE::MR.name(),
                    code: QTYPE::MR.code()
                }
            ),
            "NULL" => Ok(
                QTYPEInfo {
                    name: QTYPE::NULL.name(),
                    code: QTYPE::NULL.code()
                }
            ),
            "WKS" => Ok(
                QTYPEInfo {
                    name: QTYPE::WKS.name(),
                    code: QTYPE::WKS.code()
                }
            ),
            "PTR" => Ok(
                QTYPEInfo {
                    name: QTYPE::PTR.name(),
                    code: QTYPE::PTR.code()
                }
            ),
            "HINFO" => Ok(
                QTYPEInfo {
                    name: QTYPE::HINFO.name(),
                    code: QTYPE::HINFO.code()
                }
            ),
            "MINFO" => Ok(
                QTYPEInfo {
                    name: QTYPE::MINFO.name(),
                    code: QTYPE::MINFO.code()
                }
            ),
            "MX" => Ok(
                QTYPEInfo {
                    name: QTYPE::MX.name(),
                    code: QTYPE::MX.code()
                }
            ),
            "TXT" => Ok(
                QTYPEInfo {
                    name: QTYPE::TXT.name(),
                    code: QTYPE::TXT.code()
                }
            ),
            "RP" => Ok(
                QTYPEInfo {
                    name: QTYPE::RP.name(),
                    code: QTYPE::RP.code()
                }
            ),
            "AFSDB" => Ok(
                QTYPEInfo {
                    name: QTYPE::AFSDB.name(),
                    code: QTYPE::AFSDB.code()
                }
            ),
            "X25" => Ok(
                QTYPEInfo {
                    name: QTYPE::X25.name(),
                    code: QTYPE::X25.code()
                }
            ),
            "ISDN" => Ok(
                QTYPEInfo {
                    name: QTYPE::ISDN.name(),
                    code: QTYPE::ISDN.code()
                }
            ),
            "RT" => Ok(
                QTYPEInfo {
                    name: QTYPE::RT.name(),
                    code: QTYPE::RT.code()
                }
            ),
            "NSAP" => Ok(
                QTYPEInfo {
                    name: QTYPE::NSAP.name(),
                    code: QTYPE::NSAP.code()
                }
            ),
            "NSAP-PTR" => Ok(
                QTYPEInfo {
                    name: QTYPE::NSAP_PTR.name(),
                    code: QTYPE::NSAP_PTR.code()
                }
            ),
            "SIG" => Ok(
                QTYPEInfo {
                    name: QTYPE::SIG.name(),
                    code: QTYPE::SIG.code()
                }
            ),
            "KEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::KEY.name(),
                    code: QTYPE::KEY.code()
                }
            ),
            "PX" => Ok(
                QTYPEInfo {
                    name: QTYPE::PX.name(),
                    code: QTYPE::PX.code()
                }
            ),
            "GPOS" => Ok(
                QTYPEInfo {
                    name: QTYPE::GPOS.name(),
                    code: QTYPE::GPOS.code()
                }
            ),
            "AAAA" => Ok(
                QTYPEInfo {
                    name: QTYPE::AAAA.name(),
                    code: QTYPE::AAAA.code()
                }
            ),
            "LOC" => Ok(
                QTYPEInfo {
                    name: QTYPE::LOC.name(),
                    code: QTYPE::LOC.code()
                }
            ),
            "NXT" => Ok(
                QTYPEInfo {
                    name: QTYPE::NXT.name(),
                    code: QTYPE::NXT.code()
                }
            ),
            "EID" => Ok(
                QTYPEInfo {
                    name: QTYPE::EID.name(),
                    code: QTYPE::EID.code()
                }
            ),
            "NIMLOC" => Ok(
                QTYPEInfo {
                    name: QTYPE::NIMLOC.name(),
                    code: QTYPE::NIMLOC.code()
                }
            ),
            "SRV" => Ok(
                QTYPEInfo {
                    name: QTYPE::SRV.name(),
                    code: QTYPE::SRV.code()
                }
            ),
            "ATMA" => Ok(
                QTYPEInfo {
                    name: QTYPE::ATMA.name(),
                    code: QTYPE::ATMA.code()
                }
            ),
            "NAPTR" => Ok(
                QTYPEInfo {
                    name: QTYPE::NAPTR.name(),
                    code: QTYPE::NAPTR.code()
                }
            ),
            "KX" => Ok(
                QTYPEInfo {
                    name: QTYPE::KX.name(),
                    code: QTYPE::KX.code()
                }
            ),
            "CERT" => Ok(
                QTYPEInfo {
                    name: QTYPE::CERT.name(),
                    code: QTYPE::CERT.code()
                }
            ),
            "A6" => Ok(
                QTYPEInfo {
                    name: QTYPE::A6.name(),
                    code: QTYPE::A6.code()
                }
            ),
            "DNAME" => Ok(
                QTYPEInfo {
                    name: QTYPE::DNAME.name(),
                    code: QTYPE::DNAME.code()
                }
            ),
            "SINK" => Ok(
                QTYPEInfo {
                    name: QTYPE::SINK.name(),
                    code: QTYPE::SINK.code()
                }
            ),
            "OPT" => Ok(
                QTYPEInfo {
                    name: QTYPE::OPT.name(),
                    code: QTYPE::OPT.code()
                }
            ),
            "APL" => Ok(
                QTYPEInfo {
                    name: QTYPE::APL.name(),
                    code: QTYPE::APL.code()
                }
            ),
            "DS" => Ok(
                QTYPEInfo {
                    name: QTYPE::DS.name(),
                    code: QTYPE::DS.code()
                }
            ),
            "SSHFP" => Ok(
                QTYPEInfo {
                    name: QTYPE::SSHFP.name(),
                    code: QTYPE::SSHFP.code()
                }
            ),
            "IPSECKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::IPSECKEY.name(),
                    code: QTYPE::IPSECKEY.code()
                }
            ),
            "RRSIG" => Ok(
                QTYPEInfo {
                    name: QTYPE::RRSIG.name(),
                    code: QTYPE::RRSIG.code()
                }
            ),
            "NSEC" => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC.name(),
                    code: QTYPE::NSEC.code()
                }
            ),
            "DNSKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::DNSKEY.name(),
                    code: QTYPE::DNSKEY.code()
                }
            ),
            "DHCID" => Ok(
                QTYPEInfo {
                    name: QTYPE::DHCID.name(),
                    code: QTYPE::DHCID.code()
                }
            ),
            "NSEC3" => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC3.name(),
                    code: QTYPE::NSEC3.code()
                }
            ),
            "NSEC3PARAM" => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC3PARAM.name(),
                    code: QTYPE::NSEC3PARAM.code()
                }
            ),
            "TLSA" => Ok(
                QTYPEInfo {
                    name: QTYPE::TLSA.name(),
                    code: QTYPE::TLSA.code()
                }
            ),
            "SMIMEA" => Ok(
                QTYPEInfo {
                    name: QTYPE::SMIMEA.name(),
                    code: QTYPE::SMIMEA.code()
                }
            ),
            "HIP" => Ok(
                QTYPEInfo {
                    name: QTYPE::HIP.name(),
                    code: QTYPE::HIP.code()
                }
            ),
            "NINFO" => Ok(
                QTYPEInfo {
                    name: QTYPE::NINFO.name(),
                    code: QTYPE::NINFO.code()
                }
            ),
            "RKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::RKEY.name(),
                    code: QTYPE::RKEY.code()
                }
            ),
            "TALINK" => Ok(
                QTYPEInfo {
                    name: QTYPE::TALINK.name(),
                    code: QTYPE::TALINK.code()
                }
            ),
            "CDS" => Ok(
                QTYPEInfo {
                    name: QTYPE::CDS.name(),
                    code: QTYPE::CDS.code()
                }
            ),
            "CDNSKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::CDNSKEY.name(),
                    code: QTYPE::CDNSKEY.code()
                }
            ),
            "OPENPGPKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::OPENPGPKEY.name(),
                    code: QTYPE::OPENPGPKEY.code()
                }
            ),
            "CSYNC" => Ok(
                QTYPEInfo {
                    name: QTYPE::CSYNC.name(),
                    code: QTYPE::CSYNC.code()
                }
            ),
            "ZONEMD" => Ok(
                QTYPEInfo {
                    name: QTYPE::ZONEMD.name(),
                    code: QTYPE::ZONEMD.code()
                }
            ),
            "SVCB" => Ok(
                QTYPEInfo {
                    name: QTYPE::SVCB.name(),
                    code: QTYPE::SVCB.code()
                }
            ),
            "HTTPS" => Ok(
                QTYPEInfo {
                    name: QTYPE::HTTPS.name(),
                    code: QTYPE::HTTPS.code()
                }
            ),
            "SPF" => Ok(
                QTYPEInfo {
                    name: QTYPE::SPF.name(),
                    code: QTYPE::SPF.code()
                }
            ),
            "UINFO" => Ok(
                QTYPEInfo {
                    name: QTYPE::UINFO.name(),
                    code: QTYPE::UINFO.code()
                }
            ),
            "UID" => Ok(
                QTYPEInfo {
                    name: QTYPE::UID.name(),
                    code: QTYPE::UID.code()
                }
            ),
            "GID" => Ok(
                QTYPEInfo {
                    name: QTYPE::GID.name(),
                    code: QTYPE::GID.code()
                }
            ),
            "UNSPEC" => Ok(
                QTYPEInfo {
                    name: QTYPE::UNSPEC.name(),
                    code: QTYPE::UNSPEC.code()
                }
            ),
            "NID" => Ok(
                QTYPEInfo {
                    name: QTYPE::NID.name(),
                    code: QTYPE::NID.code()
                }
            ),
            "L32" => Ok(
                QTYPEInfo {
                    name: QTYPE::L32.name(),
                    code: QTYPE::L32.code()
                }
            ),
            "L64" => Ok(
                QTYPEInfo {
                    name: QTYPE::L64.name(),
                    code: QTYPE::L64.code()
                }
            ),
            "LP" => Ok(
                QTYPEInfo {
                    name: QTYPE::LP.name(),
                    code: QTYPE::LP.code()
                }
            ),
            "EUI48" => Ok(
                QTYPEInfo {
                    name: QTYPE::EUI48.name(),
                    code: QTYPE::EUI48.code()
                }
            ),
            "EUI64" => Ok(
                QTYPEInfo {
                    name: QTYPE::EUI64.name(),
                    code: QTYPE::EUI64.code()
                }
            ),
            "TKEY" => Ok(
                QTYPEInfo {
                    name: QTYPE::TKEY.name(),
                    code: QTYPE::TKEY.code()
                }
            ),
            "TSIG" => Ok(
                QTYPEInfo {
                    name: QTYPE::TSIG.name(),
                    code: QTYPE::TSIG.code()
                }
            ),
            "IXFR" => Ok(
                QTYPEInfo {
                    name: QTYPE::IXFR.name(),
                    code: QTYPE::IXFR.code()
                }
            ),
            "AXFR" => Ok(
                QTYPEInfo {
                    name: QTYPE::AXFR.name(),
                    code: QTYPE::AXFR.code()
                }
            ),
            "MAILB" => Ok(
                QTYPEInfo {
                    name: QTYPE::MAILB.name(),
                    code: QTYPE::MAILB.code()
                }
            ),
            "MAILA" => Ok(
                QTYPEInfo {
                    name: QTYPE::MAILA.name(),
                    code: QTYPE::MAILA.code()
                }
            ),
            "*" => Ok(
                QTYPEInfo {
                    name: QTYPE::ANY.name(),
                    code: QTYPE::ANY.code()
                }
            ),
            "URI" => Ok(
                QTYPEInfo {
                    name: QTYPE::URI.name(),
                    code: QTYPE::URI.code()
                }
            ),
            "CAA" => Ok(
                QTYPEInfo {
                    name: QTYPE::CAA.name(),
                    code: QTYPE::CAA.code()
                }
            ),
            "AVC" => Ok(
                QTYPEInfo {
                    name: QTYPE::AVC.name(),
                    code: QTYPE::AVC.code()
                }
            ),
            "DOA" => Ok(
                QTYPEInfo {
                    name: QTYPE::DOA.name(),
                    code: QTYPE::DOA.code()
                }
            ),
            "AMTRELAY" => Ok(
                QTYPEInfo {
                    name: QTYPE::AMTRELAY.name(),
                    code: QTYPE::AMTRELAY.code()
                }
            ),
            "TA" => Ok(
                QTYPEInfo {
                    name: QTYPE::TA.name(),
                    code: QTYPE::TA.code()
                }
            ),
            "DLV" => Ok(
                QTYPEInfo {
                    name: QTYPE::DLV.name(),
                    code: QTYPE::DLV.code()
                }
            ),
            _ => Err(String::from("Can't encode QTYPE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QTYPEInfo, String> {
        return match *decimal {
            0 => Ok(
                QTYPEInfo {
                    name: REVERSED,
                    code: *decimal
                }
            ),
            1 => Ok(
                QTYPEInfo {
                    name: QTYPE::A.name(),
                    code: QTYPE::A.code()
                }
            ),
            2 => Ok(
                QTYPEInfo {
                    name: QTYPE::NS.name(),
                    code: QTYPE::NS.code()
                }
            ),
            3 => Ok(
                QTYPEInfo {
                    name: QTYPE::MD.name(),
                    code: QTYPE::MD.code()
                }
            ),
            4 => Ok(
                QTYPEInfo {
                    name: QTYPE::MF.name(),
                    code: QTYPE::MF.code()
                }
            ),
            5 => Ok(
                QTYPEInfo {
                    name: QTYPE::CNAME.name(),
                    code: QTYPE::CNAME.code()
                }
            ),
            6 => Ok(
                QTYPEInfo {
                    name: QTYPE::SOA.name(),
                    code: QTYPE::SOA.code()
                }
            ),
            7 => Ok(
                QTYPEInfo {
                    name: QTYPE::MB.name(),
                    code: QTYPE::MB.code()
                }
            ),
            8 => Ok(
                QTYPEInfo {
                    name: QTYPE::MG.name(),
                    code: QTYPE::MG.code()
                }
            ),
            9 => Ok(
                QTYPEInfo {
                    name: QTYPE::MR.name(),
                    code: QTYPE::MR.code()
                }
            ),
            10 => Ok(
                QTYPEInfo {
                    name: QTYPE::NULL.name(),
                    code: QTYPE::NULL.code()
                }
            ),
            11 => Ok(
                QTYPEInfo {
                    name: QTYPE::WKS.name(),
                    code: QTYPE::WKS.code()
                }
            ),
            12 => Ok(
                QTYPEInfo {
                    name: QTYPE::PTR.name(),
                    code: QTYPE::PTR.code()
                }
            ),
            13 => Ok(
                QTYPEInfo {
                    name: QTYPE::HINFO.name(),
                    code: QTYPE::HINFO.code()
                }
            ),
            14 => Ok(
                QTYPEInfo {
                    name: QTYPE::MINFO.name(),
                    code: QTYPE::MINFO.code()
                }
            ),
            15 => Ok(
                QTYPEInfo {
                    name: QTYPE::MX.name(),
                    code: QTYPE::MX.code()
                }
            ),
            16 => Ok(
                QTYPEInfo {
                    name: QTYPE::TXT.name(),
                    code: QTYPE::TXT.code()
                }
            ),
            17 => Ok(
                QTYPEInfo {
                    name: QTYPE::RP.name(),
                    code: QTYPE::RP.code()
                }
            ),
            18 => Ok(
                QTYPEInfo {
                    name: QTYPE::AFSDB.name(),
                    code: QTYPE::AFSDB.code()
                }
            ),
            19 => Ok(
                QTYPEInfo {
                    name: QTYPE::X25.name(),
                    code: QTYPE::X25.code()
                }
            ),
            20 => Ok(
                QTYPEInfo {
                    name: QTYPE::ISDN.name(),
                    code: QTYPE::ISDN.code()
                }
            ),
            21 => Ok(
                QTYPEInfo {
                    name: QTYPE::RT.name(),
                    code: QTYPE::RT.code()
                }
            ),
            22 => Ok(
                QTYPEInfo {
                    name: QTYPE::NSAP.name(),
                    code: QTYPE::NSAP.code()
                }
            ),
            23 => Ok(
                QTYPEInfo {
                    name: QTYPE::NSAP_PTR.name(),
                    code: QTYPE::NSAP_PTR.code()
                }
            ),
            24 => Ok(
                QTYPEInfo {
                    name: QTYPE::SIG.name(),
                    code: QTYPE::SIG.code()
                }
            ),
            25 => Ok(
                QTYPEInfo {
                    name: QTYPE::KEY.name(),
                    code: QTYPE::KEY.code()
                }
            ),
            26 => Ok(
                QTYPEInfo {
                    name: QTYPE::PX.name(),
                    code: QTYPE::PX.code()
                }
            ),
            27 => Ok(
                QTYPEInfo {
                    name: QTYPE::GPOS.name(),
                    code: QTYPE::GPOS.code()
                }
            ),
            28 => Ok(
                QTYPEInfo {
                    name: QTYPE::AAAA.name(),
                    code: QTYPE::AAAA.code()
                }
            ),
            29 => Ok(
                QTYPEInfo {
                    name: QTYPE::LOC.name(),
                    code: QTYPE::LOC.code()
                }
            ),
            30 => Ok(
                QTYPEInfo {
                    name: QTYPE::NXT.name(),
                    code: QTYPE::NXT.code()
                }
            ),
            31 => Ok(
                QTYPEInfo {
                    name: QTYPE::EID.name(),
                    code: QTYPE::EID.code()
                }
            ),
            32 => Ok(
                QTYPEInfo {
                    name: QTYPE::NIMLOC.name(),
                    code: QTYPE::NIMLOC.code()
                }
            ),
            33 => Ok(
                QTYPEInfo {
                    name: QTYPE::SRV.name(),
                    code: QTYPE::SRV.code()
                }
            ),
            34 => Ok(
                QTYPEInfo {
                    name: QTYPE::ATMA.name(),
                    code: QTYPE::ATMA.code()
                }
            ),
            35 => Ok(
                QTYPEInfo {
                    name: QTYPE::NAPTR.name(),
                    code: QTYPE::NAPTR.code()
                }
            ),
            36 => Ok(
                QTYPEInfo {
                    name: QTYPE::KX.name(),
                    code: QTYPE::KX.code()
                }
            ),
            37 => Ok(
                QTYPEInfo {
                    name: QTYPE::CERT.name(),
                    code: QTYPE::CERT.code()
                }
            ),
            38 => Ok(
                QTYPEInfo {
                    name: QTYPE::A6.name(),
                    code: QTYPE::A6.code()
                }
            ),
            39 => Ok(
                QTYPEInfo {
                    name: QTYPE::DNAME.name(),
                    code: QTYPE::DNAME.code()
                }
            ),
            40 => Ok(
                QTYPEInfo {
                    name: QTYPE::SINK.name(),
                    code: QTYPE::SINK.code()
                }
            ),
            41 => Ok(
                QTYPEInfo {
                    name: QTYPE::OPT.name(),
                    code: QTYPE::OPT.code()
                }
            ),
            42 => Ok(
                QTYPEInfo {
                    name: QTYPE::APL.name(),
                    code: QTYPE::APL.code()
                }
            ),
            43 => Ok(
                QTYPEInfo {
                    name: QTYPE::DS.name(),
                    code: QTYPE::DS.code()
                }
            ),
            44 => Ok(
                QTYPEInfo {
                    name: QTYPE::SSHFP.name(),
                    code: QTYPE::SSHFP.code()
                }
            ),
            45 => Ok(
                QTYPEInfo {
                    name: QTYPE::IPSECKEY.name(),
                    code: QTYPE::IPSECKEY.code()
                }
            ),
            46 => Ok(
                QTYPEInfo {
                    name: QTYPE::RRSIG.name(),
                    code: QTYPE::RRSIG.code()
                }
            ),
            47 => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC.name(),
                    code: QTYPE::NSEC.code()
                }
            ),
            48 => Ok(
                QTYPEInfo {
                    name: QTYPE::DNSKEY.name(),
                    code: QTYPE::DNSKEY.code()
                }
            ),
            49 => Ok(
                QTYPEInfo {
                    name: QTYPE::DHCID.name(),
                    code: QTYPE::DHCID.code()
                }
            ),
            50 => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC3.name(),
                    code: QTYPE::NSEC3.code()
                }
            ),
            51 => Ok(
                QTYPEInfo {
                    name: QTYPE::NSEC3PARAM.name(),
                    code: QTYPE::NSEC3PARAM.code()
                }
            ),
            52 => Ok(
                QTYPEInfo {
                    name: QTYPE::TLSA.name(),
                    code: QTYPE::TLSA.code()
                }
            ),
            53 => Ok(
                QTYPEInfo {
                    name: QTYPE::SMIMEA.name(),
                    code: QTYPE::SMIMEA.code()
                }
            ),
            54 => Ok(
                QTYPEInfo {
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            55 => Ok(
                QTYPEInfo {
                    name: QTYPE::HIP.name(),
                    code: QTYPE::HIP.code()
                }
            ),
            56 => Ok(
                QTYPEInfo {
                    name: QTYPE::NINFO.name(),
                    code: QTYPE::NINFO.code()
                }
            ),
            57 => Ok(
                QTYPEInfo {
                    name: QTYPE::RKEY.name(),
                    code: QTYPE::RKEY.code()
                }
            ),
            58 => Ok(
                QTYPEInfo {
                    name: QTYPE::TALINK.name(),
                    code: QTYPE::TALINK.code()
                }
            ),
            59 => Ok(
                QTYPEInfo {
                    name: QTYPE::CDS.name(),
                    code: QTYPE::CDS.code()
                }
            ),
            60 => Ok(
                QTYPEInfo {
                    name: QTYPE::CDNSKEY.name(),
                    code: QTYPE::CDNSKEY.code()
                }
            ),
            61 => Ok(
                QTYPEInfo {
                    name: QTYPE::OPENPGPKEY.name(),
                    code: QTYPE::OPENPGPKEY.code()
                }
            ),
            62 => Ok(
                QTYPEInfo {
                    name: QTYPE::CSYNC.name(),
                    code: QTYPE::CSYNC.code()
                }
            ),
            63 => Ok(
                QTYPEInfo {
                    name: QTYPE::ZONEMD.name(),
                    code: QTYPE::ZONEMD.code()
                }
            ),
            64 => Ok(
                QTYPEInfo {
                    name: QTYPE::SVCB.name(),
                    code: QTYPE::SVCB.code()
                }
            ),
            65 => Ok(
                QTYPEInfo {
                    name: QTYPE::HTTPS.name(),
                    code: QTYPE::HTTPS.code()
                }
            ),
            66..=98 => Ok(
                QTYPEInfo {
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            99 => Ok(
                QTYPEInfo {
                    name: QTYPE::SPF.name(),
                    code: QTYPE::SPF.code()
                }
            ),
            100 => Ok(
                QTYPEInfo {
                    name: QTYPE::UINFO.name(),
                    code: QTYPE::UINFO.code()
                }
            ),
            101 => Ok(
                QTYPEInfo {
                    name: QTYPE::UID.name(),
                    code: QTYPE::UID.code()
                }
            ),
            102 => Ok(
                QTYPEInfo {
                    name: QTYPE::GID.name(),
                    code: QTYPE::GID.code()
                }
            ),
            103 => Ok(
                QTYPEInfo {
                    name: QTYPE::UNSPEC.name(),
                    code: QTYPE::UNSPEC.code()
                }
            ),
            104 => Ok(
                QTYPEInfo {
                    name: QTYPE::NID.name(),
                    code: QTYPE::NID.code()
                }
            ),
            105 => Ok(
                QTYPEInfo {
                    name: QTYPE::L32.name(),
                    code: QTYPE::L32.code()
                }
            ),
            106 => Ok(
                QTYPEInfo {
                    name: QTYPE::L64.name(),
                    code: QTYPE::L64.code()
                }
            ),
            107 => Ok(
                QTYPEInfo {
                    name: QTYPE::LP.name(),
                    code: QTYPE::LP.code()
                }
            ),
            108 => Ok(
                QTYPEInfo {
                    name: QTYPE::EUI48.name(),
                    code: QTYPE::EUI48.code()
                }
            ),
            109 => Ok(
                QTYPEInfo {
                    name: QTYPE::EUI64.name(),
                    code: QTYPE::EUI64.code()
                }
            ),
            110..=248 => Ok(
                QTYPEInfo {
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            249 => Ok(
                QTYPEInfo {
                    name: QTYPE::TKEY.name(),
                    code: QTYPE::TKEY.code()
                }
            ),
            250 => Ok(
                QTYPEInfo {
                    name: QTYPE::TSIG.name(),
                    code: QTYPE::TSIG.code()
                }
            ),
            251 => Ok(
                QTYPEInfo {
                    name: QTYPE::IXFR.name(),
                    code: QTYPE::IXFR.code()
                }
            ),
            252 => Ok(
                QTYPEInfo {
                    name: QTYPE::AXFR.name(),
                    code: QTYPE::AXFR.code()
                }
            ),
            253 => Ok(
                QTYPEInfo {
                    name: QTYPE::MAILB.name(),
                    code: QTYPE::MAILB.code()
                }
            ),
            254 => Ok(
                QTYPEInfo {
                    name: QTYPE::MAILA.name(),
                    code: QTYPE::MAILA.code()
                }
            ),
            255 => Ok(
                QTYPEInfo {
                    name: QTYPE::ANY.name(),
                    code: QTYPE::ANY.code()
                }
            ),
            256 => Ok(
                QTYPEInfo {
                    name: QTYPE::URI.name(),
                    code: QTYPE::URI.code()
                }
            ),
            257 => Ok(
                QTYPEInfo {
                    name: QTYPE::CAA.name(),
                    code: QTYPE::CAA.code()
                }
            ),
            258 => Ok(
                QTYPEInfo {
                    name: QTYPE::AVC.name(),
                    code: QTYPE::AVC.code()
                }
            ),
            259 => Ok(
                QTYPEInfo {
                    name: QTYPE::DOA.name(),
                    code: QTYPE::DOA.code()
                }
            ),
            260 => Ok(
                QTYPEInfo {
                    name: QTYPE::AMTRELAY.name(),
                    code: QTYPE::AMTRELAY.code()
                }
            ),
            261..=32767 => Ok(
                QTYPEInfo {
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            32768 => Ok(
                QTYPEInfo {
                    name: QTYPE::TA.name(),
                    code: QTYPE::TA.code()
                }
            ),
            32769 => Ok(
                QTYPEInfo {
                    name: QTYPE::DLV.name(),
                    code: QTYPE::DLV.code()
                }
            ),
            37770..=65279 => Ok(
                QTYPEInfo {
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            65280..=65534 => Ok(
                QTYPEInfo {
                    name: PRIVATE_USE,
                    code: *decimal
                }
            ),
            65535 => Ok(
                QTYPEInfo {
                    name: REVERSED,
                    code: *decimal
                }
            ),
            _ => Err(String::from("Can't decode QTYPE!"))
        }
    }
}
