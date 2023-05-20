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

use super::{TYPE};
use super::{TYPEInfo};
use super::{REVERSED, UNASSIGNED, PRIVATE_USE};

pub trait TYPEConversion {
    fn encode(qclass: &str) -> Result<TYPEInfo, String>;
    fn decode(dec: &u16) -> Result<TYPEInfo, String>;
}

impl TYPEConversion for TYPE {
    fn encode(qtype: &str) -> Result<TYPEInfo, String> {
        return match qtype {
            "A" => Ok(
                TYPEInfo::new(TYPE::A.name(),
                              TYPE::A.code())
            ),
            "NS" => Ok(
                TYPEInfo::new(TYPE::NS.name(),
                              TYPE::NS.code())
            ),
            "MD" => Ok(
                TYPEInfo::new(TYPE::MD.name(),
                              TYPE::MD.code())
            ),
            "MF" => Ok(
                TYPEInfo::new(TYPE::MF.name(),
                              TYPE::MF.code())
            ),
            "CNAME" => Ok(
                TYPEInfo::new(TYPE::CNAME.name(),
                              TYPE::CNAME.code())
            ),
            "SOA" => Ok(
                TYPEInfo::new(TYPE::SOA.name(),
                              TYPE::SOA.code())
            ),
            "MB" => Ok(
                TYPEInfo::new(TYPE::MB.name(),
                              TYPE::MB.code())
            ),
            "MG" => Ok(
                TYPEInfo::new(TYPE::MG.name(),
                              TYPE::MG.code())
            ),
            "MR" => Ok(
                TYPEInfo::new(TYPE::MR.name(),
                              TYPE::MR.code())
            ),
            "NULL" => Ok(
                TYPEInfo::new(TYPE::NULL.name(),
                              TYPE::NULL.code())
            ),
            "WKS" => Ok(
                TYPEInfo::new(TYPE::WKS.name(),
                              TYPE::WKS.code())
            ),
            "PTR" => Ok(
                TYPEInfo::new(TYPE::PTR.name(),
                              TYPE::PTR.code())
            ),
            "HINFO" => Ok(
                TYPEInfo::new(TYPE::HINFO.name(),
                              TYPE::HINFO.code())
            ),
            "MINFO" => Ok(
                TYPEInfo::new(TYPE::MINFO.name(),
                              TYPE::MINFO.code())
            ),
            "MX" => Ok(
                TYPEInfo::new(TYPE::MX.name(),
                              TYPE::MX.code())
            ),
            "TXT" => Ok(
                TYPEInfo::new(TYPE::TXT.name(),
                              TYPE::TXT.code())
            ),
            "RP" => Ok(
                TYPEInfo::new(TYPE::RP.name(),
                              TYPE::RP.code())
            ),
            "AFSDB" => Ok(
                TYPEInfo::new(TYPE::AFSDB.name(),
                              TYPE::AFSDB.code())
            ),
            "X25" => Ok(
                TYPEInfo::new(TYPE::X25.name(),
                              TYPE::X25.code())
            ),
            "ISDN" => Ok(
                TYPEInfo::new(TYPE::ISDN.name(),
                              TYPE::ISDN.code())
            ),
            "RT" => Ok(
                TYPEInfo::new(TYPE::RT.name(),
                              TYPE::RT.code())
            ),
            "NSAP" => Ok(
                TYPEInfo::new(TYPE::NSAP.name(),
                              TYPE::NSAP.code())
            ),
            "NSAP-PTR" => Ok(
                TYPEInfo::new(TYPE::NSAP_PTR.name(),
                              TYPE::NSAP_PTR.code())
            ),
            "SIG" => Ok(
                TYPEInfo::new(TYPE::SIG.name(),
                              TYPE::SIG.code())
            ),
            "KEY" => Ok(
                TYPEInfo::new(TYPE::KEY.name(),
                              TYPE::KEY.code())
            ),
            "PX" => Ok(
                TYPEInfo::new(TYPE::PX.name(),
                              TYPE::PX.code())
            ),
            "GPOS" => Ok(
                TYPEInfo::new(TYPE::GPOS.name(),
                              TYPE::GPOS.code())
            ),
            "AAAA" => Ok(
                TYPEInfo::new(TYPE::AAAA.name(),
                              TYPE::AAAA.code())
            ),
            "LOC" => Ok(
                TYPEInfo::new(TYPE::LOC.name(),
                              TYPE::LOC.code())
            ),
            "NXT" => Ok(
                TYPEInfo::new(TYPE::NXT.name(),
                              TYPE::NXT.code())
            ),
            "EID" => Ok(
                TYPEInfo::new(TYPE::EID.name(),
                              TYPE::EID.code())
            ),
            "NIMLOC" => Ok(
                TYPEInfo::new(TYPE::NIMLOC.name(),
                              TYPE::NIMLOC.code())
            ),
            "SRV" => Ok(
                TYPEInfo::new(TYPE::SRV.name(),
                              TYPE::SRV.code())
            ),
            "ATMA" => Ok(
                TYPEInfo::new(TYPE::ATMA.name(),
                              TYPE::ATMA.code())
            ),
            "NAPTR" => Ok(
                TYPEInfo::new(TYPE::NAPTR.name(),
                              TYPE::NAPTR.code())
            ),
            "KX" => Ok(
                TYPEInfo::new(TYPE::KX.name(),
                              TYPE::KX.code())
            ),
            "CERT" => Ok(
                TYPEInfo::new(TYPE::CERT.name(),
                              TYPE::CERT.code())
            ),
            "A6" => Ok(
                TYPEInfo::new(TYPE::A6.name(),
                              TYPE::A6.code())
            ),
            "DNAME" => Ok(
                TYPEInfo::new(TYPE::DNAME.name(),
                              TYPE::DNAME.code())
            ),
            "SINK" => Ok(
                TYPEInfo::new(TYPE::SINK.name(),
                              TYPE::SINK.code())
            ),
            "OPT" => Ok(
                TYPEInfo::new(TYPE::OPT.name(),
                              TYPE::OPT.code())
            ),
            "APL" => Ok(
                TYPEInfo::new(TYPE::APL.name(),
                              TYPE::APL.code())
            ),
            "DS" => Ok(
                TYPEInfo::new(TYPE::DS.name(),
                              TYPE::DS.code())
            ),
            "SSHFP" => Ok(
                TYPEInfo::new(TYPE::SSHFP.name(),
                              TYPE::SSHFP.code())
            ),
            "IPSECKEY" => Ok(
                TYPEInfo::new(TYPE::IPSECKEY.name(),
                              TYPE::IPSECKEY.code())
            ),
            "RRSIG" => Ok(
                TYPEInfo::new(TYPE::RRSIG.name(),
                              TYPE::RRSIG.code())
            ),
            "NSEC" => Ok(
                TYPEInfo::new(TYPE::NSEC.name(),
                              TYPE::NSEC.code())
            ),
            "DNSKEY" => Ok(
                TYPEInfo::new(TYPE::DNSKEY.name(),
                              TYPE::DNSKEY.code())
            ),
            "DHCID" => Ok(
                TYPEInfo::new(TYPE::DHCID.name(),
                              TYPE::DHCID.code())
            ),
            "NSEC3" => Ok(
                TYPEInfo::new(TYPE::NSEC3.name(),
                              TYPE::NSEC3.code())
            ),
            "NSEC3PARAM" => Ok(
                TYPEInfo::new(TYPE::NSEC3PARAM.name(),
                              TYPE::NSEC3PARAM.code())
            ),
            "TLSA" => Ok(
                TYPEInfo::new(TYPE::TLSA.name(),
                              TYPE::TLSA.code())
            ),
            "SMIMEA" => Ok(
                TYPEInfo::new(TYPE::SMIMEA.name(),
                              TYPE::SMIMEA.code())
            ),
            "HIP" => Ok(
                TYPEInfo::new(TYPE::HIP.name(),
                              TYPE::HIP.code())
            ),
            "NINFO" => Ok(
                TYPEInfo::new(TYPE::NINFO.name(),
                              TYPE::NINFO.code())
            ),
            "RKEY" => Ok(
                TYPEInfo::new(TYPE::RKEY.name(),
                              TYPE::RKEY.code())
            ),
            "TALINK" => Ok(
                TYPEInfo::new(TYPE::TALINK.name(),
                              TYPE::TALINK.code())
            ),
            "CDS" => Ok(
                TYPEInfo::new(TYPE::CDS.name(),
                              TYPE::CDS.code())
            ),
            "CDNSKEY" => Ok(
                TYPEInfo::new(TYPE::CDNSKEY.name(),
                              TYPE::CDNSKEY.code())
            ),
            "OPENPGPKEY" => Ok(
                TYPEInfo::new(TYPE::OPENPGPKEY.name(),
                              TYPE::OPENPGPKEY.code())
            ),
            "CSYNC" => Ok(
                TYPEInfo::new(TYPE::CSYNC.name(),
                              TYPE::CSYNC.code())
            ),
            "ZONEMD" => Ok(
                TYPEInfo::new(TYPE::ZONEMD.name(),
                              TYPE::ZONEMD.code())
            ),
            "SVCB" => Ok(
                TYPEInfo::new(TYPE::SVCB.name(),
                              TYPE::SVCB.code())
            ),
            "HTTPS" => Ok(
                TYPEInfo::new(TYPE::HTTPS.name(),
                              TYPE::HTTPS.code())
            ),
            "SPF" => Ok(
                TYPEInfo::new(TYPE::SPF.name(),
                              TYPE::SPF.code())
            ),
            "UINFO" => Ok(
                TYPEInfo::new(TYPE::UINFO.name(),
                              TYPE::UINFO.code())
            ),
            "UID" => Ok(
                TYPEInfo::new(TYPE::UID.name(),
                              TYPE::UID.code())
            ),
            "GID" => Ok(
                TYPEInfo::new(TYPE::GID.name(),
                              TYPE::GID.code())
            ),
            "UNSPEC" => Ok(
                TYPEInfo::new(TYPE::UNSPEC.name(),
                              TYPE::UNSPEC.code())
            ),
            "NID" => Ok(
                TYPEInfo::new(TYPE::NID.name(),
                              TYPE::NID.code())
            ),
            "L32" => Ok(
                TYPEInfo::new(TYPE::L32.name(),
                              TYPE::L32.code())
            ),
            "L64" => Ok(
                TYPEInfo::new(TYPE::L64.name(),
                              TYPE::L64.code())
            ),
            "LP" => Ok(
                TYPEInfo::new(TYPE::LP.name(),
                              TYPE::LP.code())
            ),
            "EUI48" => Ok(
                TYPEInfo::new(TYPE::EUI48.name(),
                              TYPE::EUI48.code())
            ),
            "EUI64" => Ok(
                TYPEInfo::new(TYPE::EUI64.name(),
                              TYPE::EUI64.code())
            ),
            "TKEY" => Ok(
                TYPEInfo::new(TYPE::TKEY.name(),
                              TYPE::TKEY.code())
            ),
            "TSIG" => Ok(
                TYPEInfo::new(TYPE::TSIG.name(),
                              TYPE::TSIG.code())
            ),
            "IXFR" => Ok(
                TYPEInfo::new(TYPE::IXFR.name(),
                              TYPE::IXFR.code())
            ),
            "AXFR" => Ok(
                TYPEInfo::new(TYPE::AXFR.name(),
                              TYPE::AXFR.code())
            ),
            "MAILB" => Ok(
                TYPEInfo::new(TYPE::MAILB.name(),
                              TYPE::MAILB.code())
            ),
            "MAILA" => Ok(
                TYPEInfo::new(TYPE::MAILA.name(),
                              TYPE::MAILA.code())
            ),
            "*" => Ok(
                TYPEInfo::new(TYPE::ANY.name(),
                              TYPE::ANY.code())
            ),
            "URI" => Ok(
                TYPEInfo::new(TYPE::URI.name(),
                              TYPE::URI.code())
            ),
            "CAA" => Ok(
                TYPEInfo::new(TYPE::CAA.name(),
                              TYPE::CAA.code())
            ),
            "AVC" => Ok(
                TYPEInfo::new(TYPE::AVC.name(),
                              TYPE::AVC.code())
            ),
            "DOA" => Ok(
                TYPEInfo::new(TYPE::DOA.name(),
                              TYPE::DOA.code())
            ),
            "AMTRELAY" => Ok(
                TYPEInfo::new(TYPE::AMTRELAY.name(),
                              TYPE::AMTRELAY.code())
            ),
            "TA" => Ok(
                TYPEInfo::new(TYPE::TA.name(),
                              TYPE::TA.code())
            ),
            "DLV" => Ok(
                TYPEInfo::new(TYPE::DLV.name(),
                              TYPE::DLV.code())
            ),
            _ => Err(String::from("Can't encode QTYPE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<TYPEInfo, String> {
        return match *decimal {
            0 => Ok(
                TYPEInfo::new(REVERSED,
                              *decimal)
            ),
            1 => Ok(
                TYPEInfo::new(TYPE::A.name(),
                              TYPE::A.code())
            ),
            2 => Ok(
                TYPEInfo::new(TYPE::NS.name(),
                              TYPE::NS.code())
            ),
            3 => Ok(
                TYPEInfo::new(TYPE::MD.name(),
                              TYPE::MD.code())
            ),
            4 => Ok(
                TYPEInfo::new(TYPE::MF.name(),
                              TYPE::MF.code())
            ),
            5 => Ok(
                TYPEInfo::new(TYPE::CNAME.name(),
                              TYPE::CNAME.code())
            ),
            6 => Ok(
                TYPEInfo::new(TYPE::SOA.name(),
                              TYPE::SOA.code())
            ),
            7 => Ok(
                TYPEInfo::new(TYPE::MB.name(),
                              TYPE::MB.code())
            ),
            8 => Ok(
                TYPEInfo::new(TYPE::MG.name(),
                              TYPE::MG.code())
            ),
            9 => Ok(
                TYPEInfo::new(TYPE::MR.name(),
                              TYPE::MR.code())
            ),
            10 => Ok(
                TYPEInfo::new(TYPE::NULL.name(),
                              TYPE::NULL.code())
            ),
            11 => Ok(
                TYPEInfo::new(TYPE::WKS.name(),
                              TYPE::WKS.code())
            ),
            12 => Ok(
                TYPEInfo::new(TYPE::PTR.name(),
                              TYPE::PTR.code())
            ),
            13 => Ok(
                TYPEInfo::new(TYPE::HINFO.name(),
                              TYPE::HINFO.code())
            ),
            14 => Ok(
                TYPEInfo::new(TYPE::MINFO.name(),
                              TYPE::MINFO.code())
            ),
            15 => Ok(
                TYPEInfo::new(TYPE::MX.name(),
                              TYPE::MX.code())
            ),
            16 => Ok(
                TYPEInfo::new(TYPE::TXT.name(),
                              TYPE::TXT.code())
            ),
            17 => Ok(
                TYPEInfo::new(TYPE::RP.name(),
                              TYPE::RP.code())
            ),
            18 => Ok(
                TYPEInfo::new(TYPE::AFSDB.name(),
                              TYPE::AFSDB.code())
            ),
            19 => Ok(
                TYPEInfo::new(TYPE::X25.name(),
                              TYPE::X25.code())
            ),
            20 => Ok(
                TYPEInfo::new(TYPE::ISDN.name(),
                              TYPE::ISDN.code())
            ),
            21 => Ok(
                TYPEInfo::new(TYPE::RT.name(),
                              TYPE::RT.code())
            ),
            22 => Ok(
                TYPEInfo::new(TYPE::NSAP.name(),
                              TYPE::NSAP.code())
            ),
            23 => Ok(
                TYPEInfo::new(TYPE::NSAP_PTR.name(),
                              TYPE::NSAP_PTR.code())
            ),
            24 => Ok(
                TYPEInfo::new(TYPE::SIG.name(),
                              TYPE::SIG.code())
            ),
            25 => Ok(
                TYPEInfo::new(TYPE::KEY.name(),
                              TYPE::KEY.code())
            ),
            26 => Ok(
                TYPEInfo::new(TYPE::PX.name(),
                              TYPE::PX.code())
            ),
            27 => Ok(
                TYPEInfo::new(TYPE::GPOS.name(),
                              TYPE::GPOS.code())
            ),
            28 => Ok(
                TYPEInfo::new(TYPE::AAAA.name(),
                              TYPE::AAAA.code())
            ),
            29 => Ok(
                TYPEInfo::new(TYPE::LOC.name(),
                              TYPE::LOC.code())
            ),
            30 => Ok(
                TYPEInfo::new(TYPE::NXT.name(),
                              TYPE::NXT.code())
            ),
            31 => Ok(
                TYPEInfo::new(TYPE::EID.name(),
                              TYPE::EID.code())
            ),
            32 => Ok(
                TYPEInfo::new(TYPE::NIMLOC.name(),
                              TYPE::NIMLOC.code())
            ),
            33 => Ok(
                TYPEInfo::new(TYPE::SRV.name(),
                              TYPE::SRV.code())
            ),
            34 => Ok(
                TYPEInfo::new(TYPE::ATMA.name(),
                              TYPE::ATMA.code())
            ),
            35 => Ok(
                TYPEInfo::new(TYPE::NAPTR.name(),
                              TYPE::NAPTR.code())
            ),
            36 => Ok(
                TYPEInfo::new(TYPE::KX.name(),
                              TYPE::KX.code())
            ),
            37 => Ok(
                TYPEInfo::new(TYPE::CERT.name(),
                              TYPE::CERT.code())
            ),
            38 => Ok(
                TYPEInfo::new(TYPE::A6.name(),
                              TYPE::A6.code())
            ),
            39 => Ok(
                TYPEInfo::new(TYPE::DNAME.name(),
                              TYPE::DNAME.code())
            ),
            40 => Ok(
                TYPEInfo::new(TYPE::SINK.name(),
                              TYPE::SINK.code())
            ),
            41 => Ok(
                TYPEInfo::new(TYPE::OPT.name(),
                              TYPE::OPT.code())
            ),
            42 => Ok(
                TYPEInfo::new(TYPE::APL.name(),
                              TYPE::APL.code())
            ),
            43 => Ok(
                TYPEInfo::new(TYPE::DS.name(),
                              TYPE::DS.code())
            ),
            44 => Ok(
                TYPEInfo::new(TYPE::SSHFP.name(),
                              TYPE::SSHFP.code())
            ),
            45 => Ok(
                TYPEInfo::new(TYPE::IPSECKEY.name(),
                              TYPE::IPSECKEY.code())
            ),
            46 => Ok(
                TYPEInfo::new(TYPE::RRSIG.name(),
                              TYPE::RRSIG.code())
            ),
            47 => Ok(
                TYPEInfo::new(TYPE::NSEC.name(),
                              TYPE::NSEC.code())
            ),
            48 => Ok(
                TYPEInfo::new(TYPE::DNSKEY.name(),
                              TYPE::DNSKEY.code())
            ),
            49 => Ok(
                TYPEInfo::new(TYPE::DHCID.name(),
                              TYPE::DHCID.code())
            ),
            50 => Ok(
                TYPEInfo::new(TYPE::NSEC3.name(),
                              TYPE::NSEC3.code())
            ),
            51 => Ok(
                TYPEInfo::new(TYPE::NSEC3PARAM.name(),
                              TYPE::NSEC3PARAM.code())
            ),
            52 => Ok(
                TYPEInfo::new(TYPE::TLSA.name(),
                              TYPE::TLSA.code())
            ),
            53 => Ok(
                TYPEInfo::new(TYPE::SMIMEA.name(),
                              TYPE::SMIMEA.code())
            ),
            54 => Ok(
                TYPEInfo::new(UNASSIGNED,
                              *decimal)
            ),
            55 => Ok(
                TYPEInfo::new(TYPE::HIP.name(),
                              TYPE::HIP.code())
            ),
            56 => Ok(
                TYPEInfo::new(TYPE::NINFO.name(),
                              TYPE::NINFO.code())
            ),
            57 => Ok(
                TYPEInfo::new(TYPE::RKEY.name(),
                              TYPE::RKEY.code())
            ),
            58 => Ok(
                TYPEInfo::new(TYPE::TALINK.name(),
                              TYPE::TALINK.code())
            ),
            59 => Ok(
                TYPEInfo::new(TYPE::CDS.name(),
                              TYPE::CDS.code())
            ),
            60 => Ok(
                TYPEInfo::new(TYPE::CDNSKEY.name(),
                              TYPE::CDNSKEY.code())
            ),
            61 => Ok(
                TYPEInfo::new(TYPE::OPENPGPKEY.name(),
                              TYPE::OPENPGPKEY.code())
            ),
            62 => Ok(
                TYPEInfo::new(TYPE::CSYNC.name(),
                              TYPE::CSYNC.code())
            ),
            63 => Ok(
                TYPEInfo::new(TYPE::ZONEMD.name(),
                              TYPE::ZONEMD.code())
            ),
            64 => Ok(
                TYPEInfo::new(TYPE::SVCB.name(),
                              TYPE::SVCB.code())
            ),
            65 => Ok(
                TYPEInfo::new(TYPE::HTTPS.name(),
                              TYPE::HTTPS.code())
            ),
            66..=98 => Ok(
                TYPEInfo::new(UNASSIGNED,
                              *decimal)
            ),
            99 => Ok(
                TYPEInfo::new(TYPE::SPF.name(),
                              TYPE::SPF.code())
            ),
            100 => Ok(
                TYPEInfo::new(TYPE::UINFO.name(),
                              TYPE::UINFO.code())
            ),
            101 => Ok(
                TYPEInfo::new(TYPE::UID.name(),
                              TYPE::UID.code())
            ),
            102 => Ok(
                TYPEInfo::new(TYPE::GID.name(),
                              TYPE::GID.code())
            ),
            103 => Ok(
                TYPEInfo::new(TYPE::UNSPEC.name(),
                              TYPE::UNSPEC.code())
            ),
            104 => Ok(
                TYPEInfo::new(TYPE::NID.name(),
                              TYPE::NID.code())
            ),
            105 => Ok(
                TYPEInfo::new(TYPE::L32.name(),
                              TYPE::L32.code())
            ),
            106 => Ok(
                TYPEInfo::new(TYPE::L64.name(),
                              TYPE::L64.code())
            ),
            107 => Ok(
                TYPEInfo::new(TYPE::LP.name(),
                              TYPE::LP.code())
            ),
            108 => Ok(
                TYPEInfo::new(TYPE::EUI48.name(),
                              TYPE::EUI48.code())
            ),
            109 => Ok(
                TYPEInfo::new(TYPE::EUI64.name(),
                              TYPE::EUI64.code())
            ),
            110..=248 => Ok(
                TYPEInfo::new(UNASSIGNED,
                              *decimal)
            ),
            249 => Ok(
                TYPEInfo::new(TYPE::TKEY.name(),
                              TYPE::TKEY.code())
            ),
            250 => Ok(
                TYPEInfo::new(TYPE::TSIG.name(),
                              TYPE::TSIG.code())
            ),
            251 => Ok(
                TYPEInfo::new(TYPE::IXFR.name(),
                              TYPE::IXFR.code())
            ),
            252 => Ok(
                TYPEInfo::new(TYPE::AXFR.name(),
                              TYPE::AXFR.code())
            ),
            253 => Ok(
                TYPEInfo::new(TYPE::MAILB.name(),
                              TYPE::MAILB.code())
            ),
            254 => Ok(
                TYPEInfo::new(TYPE::MAILA.name(),
                              TYPE::MAILA.code())
            ),
            255 => Ok(
                TYPEInfo::new(TYPE::ANY.name(),
                              TYPE::ANY.code())
            ),
            256 => Ok(
                TYPEInfo::new(TYPE::URI.name(),
                              TYPE::URI.code())
            ),
            257 => Ok(
                TYPEInfo::new(TYPE::CAA.name(),
                              TYPE::CAA.code())
            ),
            258 => Ok(
                TYPEInfo::new(TYPE::AVC.name(),
                              TYPE::AVC.code())
            ),
            259 => Ok(
                TYPEInfo::new(TYPE::DOA.name(),
                              TYPE::DOA.code())
            ),
            260 => Ok(
                TYPEInfo::new(TYPE::AMTRELAY.name(),
                              TYPE::AMTRELAY.code())
            ),
            261..=32767 => Ok(
                TYPEInfo::new(UNASSIGNED,
                              *decimal)
            ),
            32768 => Ok(
                TYPEInfo::new(TYPE::TA.name(),
                              TYPE::TA.code())
            ),
            32769 => Ok(
                TYPEInfo::new(TYPE::DLV.name(),
                              TYPE::DLV.code())
            ),
            37770..=65279 => Ok(
                TYPEInfo::new(UNASSIGNED,
                              *decimal)
            ),
            65280..=65534 => Ok(
                TYPEInfo::new(PRIVATE_USE,
                              *decimal)
            ),
            65535 => Ok(
                TYPEInfo::new(REVERSED,
                              *decimal)
            ),
            _ => Err(String::from("Can't decode QTYPE!"))
        }
    }
}
