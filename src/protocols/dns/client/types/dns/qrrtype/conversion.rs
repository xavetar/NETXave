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

use super::{QTYPE};
use super::{QTYPEInfo};
use super::{REVERSED, UNASSIGNED, PRIVATE_USE};

pub trait QTYPEConversion {
    fn encode(qclass: &str) -> Result<QTYPEInfo, String>;
    fn decode(dec: &u16) -> Result<QTYPEInfo, String>;
}

impl QTYPEConversion for QTYPE {
    fn encode(qtype: &str) -> Result<QTYPEInfo, String> {
        return match qtype {
            "A" => Ok(
                QTYPEInfo::new(QTYPE::A.name(),
                               QTYPE::A.code())
            ),
            "NS" => Ok(
                QTYPEInfo::new(QTYPE::NS.name(),
                               QTYPE::NS.code())
            ),
            "MD" => Ok(
                QTYPEInfo::new(QTYPE::MD.name(),
                               QTYPE::MD.code())
            ),
            "MF" => Ok(
                QTYPEInfo::new(QTYPE::MF.name(),
                               QTYPE::MF.code())
            ),
            "CNAME" => Ok(
                QTYPEInfo::new(QTYPE::CNAME.name(),
                               QTYPE::CNAME.code())
            ),
            "SOA" => Ok(
                QTYPEInfo::new(QTYPE::SOA.name(),
                               QTYPE::SOA.code())
            ),
            "MB" => Ok(
                QTYPEInfo::new(QTYPE::MB.name(),
                               QTYPE::MB.code())
            ),
            "MG" => Ok(
                QTYPEInfo::new(QTYPE::MG.name(),
                               QTYPE::MG.code())
            ),
            "MR" => Ok(
                QTYPEInfo::new(QTYPE::MR.name(),
                               QTYPE::MR.code())
            ),
            "NULL" => Ok(
                QTYPEInfo::new(QTYPE::NULL.name(),
                               QTYPE::NULL.code())
            ),
            "WKS" => Ok(
                QTYPEInfo::new(QTYPE::WKS.name(),
                               QTYPE::WKS.code())
            ),
            "PTR" => Ok(
                QTYPEInfo::new(QTYPE::PTR.name(),
                               QTYPE::PTR.code())
            ),
            "HINFO" => Ok(
                QTYPEInfo::new(QTYPE::HINFO.name(),
                               QTYPE::HINFO.code())
            ),
            "MINFO" => Ok(
                QTYPEInfo::new(QTYPE::MINFO.name(),
                               QTYPE::MINFO.code())
            ),
            "MX" => Ok(
                QTYPEInfo::new(QTYPE::MX.name(),
                               QTYPE::MX.code())
            ),
            "TXT" => Ok(
                QTYPEInfo::new(QTYPE::TXT.name(),
                               QTYPE::TXT.code())
            ),
            "RP" => Ok(
                QTYPEInfo::new(QTYPE::RP.name(),
                               QTYPE::RP.code())
            ),
            "AFSDB" => Ok(
                QTYPEInfo::new(QTYPE::AFSDB.name(),
                               QTYPE::AFSDB.code())
            ),
            "X25" => Ok(
                QTYPEInfo::new(QTYPE::X25.name(),
                               QTYPE::X25.code())
            ),
            "ISDN" => Ok(
                QTYPEInfo::new(QTYPE::ISDN.name(),
                               QTYPE::ISDN.code())
            ),
            "RT" => Ok(
                QTYPEInfo::new(QTYPE::RT.name(),
                               QTYPE::RT.code())
            ),
            "NSAP" => Ok(
                QTYPEInfo::new(QTYPE::NSAP.name(),
                               QTYPE::NSAP.code())
            ),
            "NSAP-PTR" => Ok(
                QTYPEInfo::new(QTYPE::NSAP_PTR.name(),
                               QTYPE::NSAP_PTR.code())
            ),
            "SIG" => Ok(
                QTYPEInfo::new(QTYPE::SIG.name(),
                               QTYPE::SIG.code())
            ),
            "KEY" => Ok(
                QTYPEInfo::new(QTYPE::KEY.name(),
                               QTYPE::KEY.code())
            ),
            "PX" => Ok(
                QTYPEInfo::new(QTYPE::PX.name(),
                               QTYPE::PX.code())
            ),
            "GPOS" => Ok(
                QTYPEInfo::new(QTYPE::GPOS.name(),
                               QTYPE::GPOS.code())
            ),
            "AAAA" => Ok(
                QTYPEInfo::new(QTYPE::AAAA.name(),
                               QTYPE::AAAA.code())
            ),
            "LOC" => Ok(
                QTYPEInfo::new(QTYPE::LOC.name(),
                               QTYPE::LOC.code())
            ),
            "NXT" => Ok(
                QTYPEInfo::new(QTYPE::NXT.name(),
                               QTYPE::NXT.code())
            ),
            "EID" => Ok(
                QTYPEInfo::new(QTYPE::EID.name(),
                               QTYPE::EID.code())
            ),
            "NIMLOC" => Ok(
                QTYPEInfo::new(QTYPE::NIMLOC.name(),
                               QTYPE::NIMLOC.code())
            ),
            "SRV" => Ok(
                QTYPEInfo::new(QTYPE::SRV.name(),
                               QTYPE::SRV.code())
            ),
            "ATMA" => Ok(
                QTYPEInfo::new(QTYPE::ATMA.name(),
                               QTYPE::ATMA.code())
            ),
            "NAPTR" => Ok(
                QTYPEInfo::new(QTYPE::NAPTR.name(),
                               QTYPE::NAPTR.code())
            ),
            "KX" => Ok(
                QTYPEInfo::new(QTYPE::KX.name(),
                               QTYPE::KX.code())
            ),
            "CERT" => Ok(
                QTYPEInfo::new(QTYPE::CERT.name(),
                               QTYPE::CERT.code())
            ),
            "A6" => Ok(
                QTYPEInfo::new(QTYPE::A6.name(),
                               QTYPE::A6.code())
            ),
            "DNAME" => Ok(
                QTYPEInfo::new(QTYPE::DNAME.name(),
                               QTYPE::DNAME.code())
            ),
            "SINK" => Ok(
                QTYPEInfo::new(QTYPE::SINK.name(),
                               QTYPE::SINK.code())
            ),
            "OPT" => Ok(
                QTYPEInfo::new(QTYPE::OPT.name(),
                               QTYPE::OPT.code())
            ),
            "APL" => Ok(
                QTYPEInfo::new(QTYPE::APL.name(),
                               QTYPE::APL.code())
            ),
            "DS" => Ok(
                QTYPEInfo::new(QTYPE::DS.name(),
                               QTYPE::DS.code())
            ),
            "SSHFP" => Ok(
                QTYPEInfo::new(QTYPE::SSHFP.name(),
                               QTYPE::SSHFP.code())
            ),
            "IPSECKEY" => Ok(
                QTYPEInfo::new(QTYPE::IPSECKEY.name(),
                               QTYPE::IPSECKEY.code())
            ),
            "RRSIG" => Ok(
                QTYPEInfo::new(QTYPE::RRSIG.name(),
                               QTYPE::RRSIG.code())
            ),
            "NSEC" => Ok(
                QTYPEInfo::new(QTYPE::NSEC.name(),
                               QTYPE::NSEC.code())
            ),
            "DNSKEY" => Ok(
                QTYPEInfo::new(QTYPE::DNSKEY.name(),
                               QTYPE::DNSKEY.code())
            ),
            "DHCID" => Ok(
                QTYPEInfo::new(QTYPE::DHCID.name(),
                               QTYPE::DHCID.code())
            ),
            "NSEC3" => Ok(
                QTYPEInfo::new(QTYPE::NSEC3.name(),
                               QTYPE::NSEC3.code())
            ),
            "NSEC3PARAM" => Ok(
                QTYPEInfo::new(QTYPE::NSEC3PARAM.name(),
                               QTYPE::NSEC3PARAM.code())
            ),
            "TLSA" => Ok(
                QTYPEInfo::new(QTYPE::TLSA.name(),
                               QTYPE::TLSA.code())
            ),
            "SMIMEA" => Ok(
                QTYPEInfo::new(QTYPE::SMIMEA.name(),
                               QTYPE::SMIMEA.code())
            ),
            "HIP" => Ok(
                QTYPEInfo::new(QTYPE::HIP.name(),
                               QTYPE::HIP.code())
            ),
            "NINFO" => Ok(
                QTYPEInfo::new(QTYPE::NINFO.name(),
                               QTYPE::NINFO.code())
            ),
            "RKEY" => Ok(
                QTYPEInfo::new(QTYPE::RKEY.name(),
                               QTYPE::RKEY.code())
            ),
            "TALINK" => Ok(
                QTYPEInfo::new(QTYPE::TALINK.name(),
                               QTYPE::TALINK.code())
            ),
            "CDS" => Ok(
                QTYPEInfo::new(QTYPE::CDS.name(),
                               QTYPE::CDS.code())
            ),
            "CDNSKEY" => Ok(
                QTYPEInfo::new(QTYPE::CDNSKEY.name(),
                               QTYPE::CDNSKEY.code())
            ),
            "OPENPGPKEY" => Ok(
                QTYPEInfo::new(QTYPE::OPENPGPKEY.name(),
                               QTYPE::OPENPGPKEY.code())
            ),
            "CSYNC" => Ok(
                QTYPEInfo::new(QTYPE::CSYNC.name(),
                               QTYPE::CSYNC.code())
            ),
            "ZONEMD" => Ok(
                QTYPEInfo::new(QTYPE::ZONEMD.name(),
                               QTYPE::ZONEMD.code())
            ),
            "SVCB" => Ok(
                QTYPEInfo::new(QTYPE::SVCB.name(),
                               QTYPE::SVCB.code())
            ),
            "HTTPS" => Ok(
                QTYPEInfo::new(QTYPE::HTTPS.name(),
                               QTYPE::HTTPS.code())
            ),
            "SPF" => Ok(
                QTYPEInfo::new(QTYPE::SPF.name(),
                               QTYPE::SPF.code())
            ),
            "UINFO" => Ok(
                QTYPEInfo::new(QTYPE::UINFO.name(),
                               QTYPE::UINFO.code())
            ),
            "UID" => Ok(
                QTYPEInfo::new(QTYPE::UID.name(),
                               QTYPE::UID.code())
            ),
            "GID" => Ok(
                QTYPEInfo::new(QTYPE::GID.name(),
                               QTYPE::GID.code())
            ),
            "UNSPEC" => Ok(
                QTYPEInfo::new(QTYPE::UNSPEC.name(),
                               QTYPE::UNSPEC.code())
            ),
            "NID" => Ok(
                QTYPEInfo::new(QTYPE::NID.name(),
                               QTYPE::NID.code())
            ),
            "L32" => Ok(
                QTYPEInfo::new(QTYPE::L32.name(),
                               QTYPE::L32.code())
            ),
            "L64" => Ok(
                QTYPEInfo::new(QTYPE::L64.name(),
                               QTYPE::L64.code())
            ),
            "LP" => Ok(
                QTYPEInfo::new(QTYPE::LP.name(),
                               QTYPE::LP.code())
            ),
            "EUI48" => Ok(
                QTYPEInfo::new(QTYPE::EUI48.name(),
                               QTYPE::EUI48.code())
            ),
            "EUI64" => Ok(
                QTYPEInfo::new(QTYPE::EUI64.name(),
                               QTYPE::EUI64.code())
            ),
            "TKEY" => Ok(
                QTYPEInfo::new(QTYPE::TKEY.name(),
                               QTYPE::TKEY.code())
            ),
            "TSIG" => Ok(
                QTYPEInfo::new(QTYPE::TSIG.name(),
                               QTYPE::TSIG.code())
            ),
            "IXFR" => Ok(
                QTYPEInfo::new(QTYPE::IXFR.name(),
                               QTYPE::IXFR.code())
            ),
            "AXFR" => Ok(
                QTYPEInfo::new(QTYPE::AXFR.name(),
                               QTYPE::AXFR.code())
            ),
            "MAILB" => Ok(
                QTYPEInfo::new(QTYPE::MAILB.name(),
                               QTYPE::MAILB.code())
            ),
            "MAILA" => Ok(
                QTYPEInfo::new(QTYPE::MAILA.name(),
                               QTYPE::MAILA.code())
            ),
            "*" => Ok(
                QTYPEInfo::new(QTYPE::ANY.name(),
                               QTYPE::ANY.code())
            ),
            "URI" => Ok(
                QTYPEInfo::new(QTYPE::URI.name(),
                               QTYPE::URI.code())
            ),
            "CAA" => Ok(
                QTYPEInfo::new(QTYPE::CAA.name(),
                               QTYPE::CAA.code())
            ),
            "AVC" => Ok(
                QTYPEInfo::new(QTYPE::AVC.name(),
                               QTYPE::AVC.code())
            ),
            "DOA" => Ok(
                QTYPEInfo::new(QTYPE::DOA.name(),
                               QTYPE::DOA.code())
            ),
            "AMTRELAY" => Ok(
                QTYPEInfo::new(QTYPE::AMTRELAY.name(),
                               QTYPE::AMTRELAY.code())
            ),
            "TA" => Ok(
                QTYPEInfo::new(QTYPE::TA.name(),
                               QTYPE::TA.code())
            ),
            "DLV" => Ok(
                QTYPEInfo::new(QTYPE::DLV.name(),
                               QTYPE::DLV.code())
            ),
            _ => Err(String::from("Can't encode QTYPE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QTYPEInfo, String> {
        return match *decimal {
            0 => Ok(
                QTYPEInfo::new(REVERSED,
                               *decimal)
            ),
            1 => Ok(
                QTYPEInfo::new(QTYPE::A.name(),
                               QTYPE::A.code())
            ),
            2 => Ok(
                QTYPEInfo::new(QTYPE::NS.name(),
                               QTYPE::NS.code())
            ),
            3 => Ok(
                QTYPEInfo::new(QTYPE::MD.name(),
                               QTYPE::MD.code())
            ),
            4 => Ok(
                QTYPEInfo::new(QTYPE::MF.name(),
                               QTYPE::MF.code())
            ),
            5 => Ok(
                QTYPEInfo::new(QTYPE::CNAME.name(),
                               QTYPE::CNAME.code())
            ),
            6 => Ok(
                QTYPEInfo::new(QTYPE::SOA.name(),
                               QTYPE::SOA.code())
            ),
            7 => Ok(
                QTYPEInfo::new(QTYPE::MB.name(),
                               QTYPE::MB.code())
            ),
            8 => Ok(
                QTYPEInfo::new(QTYPE::MG.name(),
                               QTYPE::MG.code())
            ),
            9 => Ok(
                QTYPEInfo::new(QTYPE::MR.name(),
                               QTYPE::MR.code())
            ),
            10 => Ok(
                QTYPEInfo::new(QTYPE::NULL.name(),
                               QTYPE::NULL.code())
            ),
            11 => Ok(
                QTYPEInfo::new(QTYPE::WKS.name(),
                               QTYPE::WKS.code())
            ),
            12 => Ok(
                QTYPEInfo::new(QTYPE::PTR.name(),
                               QTYPE::PTR.code())
            ),
            13 => Ok(
                QTYPEInfo::new(QTYPE::HINFO.name(),
                               QTYPE::HINFO.code())
            ),
            14 => Ok(
                QTYPEInfo::new(QTYPE::MINFO.name(),
                               QTYPE::MINFO.code())
            ),
            15 => Ok(
                QTYPEInfo::new(QTYPE::MX.name(),
                               QTYPE::MX.code())
            ),
            16 => Ok(
                QTYPEInfo::new(QTYPE::TXT.name(),
                               QTYPE::TXT.code())
            ),
            17 => Ok(
                QTYPEInfo::new(QTYPE::RP.name(),
                               QTYPE::RP.code())
            ),
            18 => Ok(
                QTYPEInfo::new(QTYPE::AFSDB.name(),
                               QTYPE::AFSDB.code())
            ),
            19 => Ok(
                QTYPEInfo::new(QTYPE::X25.name(),
                               QTYPE::X25.code())
            ),
            20 => Ok(
                QTYPEInfo::new(QTYPE::ISDN.name(),
                               QTYPE::ISDN.code())
            ),
            21 => Ok(
                QTYPEInfo::new(QTYPE::RT.name(),
                               QTYPE::RT.code())
            ),
            22 => Ok(
                QTYPEInfo::new(QTYPE::NSAP.name(),
                               QTYPE::NSAP.code())
            ),
            23 => Ok(
                QTYPEInfo::new(QTYPE::NSAP_PTR.name(),
                               QTYPE::NSAP_PTR.code())
            ),
            24 => Ok(
                QTYPEInfo::new(QTYPE::SIG.name(),
                               QTYPE::SIG.code())
            ),
            25 => Ok(
                QTYPEInfo::new(QTYPE::KEY.name(),
                               QTYPE::KEY.code())
            ),
            26 => Ok(
                QTYPEInfo::new(QTYPE::PX.name(),
                               QTYPE::PX.code())
            ),
            27 => Ok(
                QTYPEInfo::new(QTYPE::GPOS.name(),
                               QTYPE::GPOS.code())
            ),
            28 => Ok(
                QTYPEInfo::new(QTYPE::AAAA.name(),
                               QTYPE::AAAA.code())
            ),
            29 => Ok(
                QTYPEInfo::new(QTYPE::LOC.name(),
                               QTYPE::LOC.code())
            ),
            30 => Ok(
                QTYPEInfo::new(QTYPE::NXT.name(),
                               QTYPE::NXT.code())
            ),
            31 => Ok(
                QTYPEInfo::new(QTYPE::EID.name(),
                               QTYPE::EID.code())
            ),
            32 => Ok(
                QTYPEInfo::new(QTYPE::NIMLOC.name(),
                               QTYPE::NIMLOC.code())
            ),
            33 => Ok(
                QTYPEInfo::new(QTYPE::SRV.name(),
                               QTYPE::SRV.code())
            ),
            34 => Ok(
                QTYPEInfo::new(QTYPE::ATMA.name(),
                               QTYPE::ATMA.code())
            ),
            35 => Ok(
                QTYPEInfo::new(QTYPE::NAPTR.name(),
                               QTYPE::NAPTR.code())
            ),
            36 => Ok(
                QTYPEInfo::new(QTYPE::KX.name(),
                               QTYPE::KX.code())
            ),
            37 => Ok(
                QTYPEInfo::new(QTYPE::CERT.name(),
                               QTYPE::CERT.code())
            ),
            38 => Ok(
                QTYPEInfo::new(QTYPE::A6.name(),
                               QTYPE::A6.code())
            ),
            39 => Ok(
                QTYPEInfo::new(QTYPE::DNAME.name(),
                               QTYPE::DNAME.code())
            ),
            40 => Ok(
                QTYPEInfo::new(QTYPE::SINK.name(),
                               QTYPE::SINK.code())
            ),
            41 => Ok(
                QTYPEInfo::new(QTYPE::OPT.name(),
                               QTYPE::OPT.code())
            ),
            42 => Ok(
                QTYPEInfo::new(QTYPE::APL.name(),
                               QTYPE::APL.code())
            ),
            43 => Ok(
                QTYPEInfo::new(QTYPE::DS.name(),
                               QTYPE::DS.code())
            ),
            44 => Ok(
                QTYPEInfo::new(QTYPE::SSHFP.name(),
                               QTYPE::SSHFP.code())
            ),
            45 => Ok(
                QTYPEInfo::new(QTYPE::IPSECKEY.name(),
                               QTYPE::IPSECKEY.code())
            ),
            46 => Ok(
                QTYPEInfo::new(QTYPE::RRSIG.name(),
                               QTYPE::RRSIG.code())
            ),
            47 => Ok(
                QTYPEInfo::new(QTYPE::NSEC.name(),
                               QTYPE::NSEC.code())
            ),
            48 => Ok(
                QTYPEInfo::new(QTYPE::DNSKEY.name(),
                               QTYPE::DNSKEY.code())
            ),
            49 => Ok(
                QTYPEInfo::new(QTYPE::DHCID.name(),
                               QTYPE::DHCID.code())
            ),
            50 => Ok(
                QTYPEInfo::new(QTYPE::NSEC3.name(),
                               QTYPE::NSEC3.code())
            ),
            51 => Ok(
                QTYPEInfo::new(QTYPE::NSEC3PARAM.name(),
                               QTYPE::NSEC3PARAM.code())
            ),
            52 => Ok(
                QTYPEInfo::new(QTYPE::TLSA.name(),
                               QTYPE::TLSA.code())
            ),
            53 => Ok(
                QTYPEInfo::new(QTYPE::SMIMEA.name(),
                               QTYPE::SMIMEA.code())
            ),
            54 => Ok(
                QTYPEInfo::new(UNASSIGNED,
                               *decimal)
            ),
            55 => Ok(
                QTYPEInfo::new(QTYPE::HIP.name(),
                               QTYPE::HIP.code())
            ),
            56 => Ok(
                QTYPEInfo::new(QTYPE::NINFO.name(),
                               QTYPE::NINFO.code())
            ),
            57 => Ok(
                QTYPEInfo::new(QTYPE::RKEY.name(),
                               QTYPE::RKEY.code())
            ),
            58 => Ok(
                QTYPEInfo::new(QTYPE::TALINK.name(),
                               QTYPE::TALINK.code())
            ),
            59 => Ok(
                QTYPEInfo::new(QTYPE::CDS.name(),
                               QTYPE::CDS.code())
            ),
            60 => Ok(
                QTYPEInfo::new(QTYPE::CDNSKEY.name(),
                               QTYPE::CDNSKEY.code())
            ),
            61 => Ok(
                QTYPEInfo::new(QTYPE::OPENPGPKEY.name(),
                               QTYPE::OPENPGPKEY.code())
            ),
            62 => Ok(
                QTYPEInfo::new(QTYPE::CSYNC.name(),
                               QTYPE::CSYNC.code())
            ),
            63 => Ok(
                QTYPEInfo::new(QTYPE::ZONEMD.name(),
                               QTYPE::ZONEMD.code())
            ),
            64 => Ok(
                QTYPEInfo::new(QTYPE::SVCB.name(),
                               QTYPE::SVCB.code())
            ),
            65 => Ok(
                QTYPEInfo::new(QTYPE::HTTPS.name(),
                               QTYPE::HTTPS.code())
            ),
            66..=98 => Ok(
                QTYPEInfo::new(UNASSIGNED,
                               *decimal)
            ),
            99 => Ok(
                QTYPEInfo::new(QTYPE::SPF.name(),
                               QTYPE::SPF.code())
            ),
            100 => Ok(
                QTYPEInfo::new(QTYPE::UINFO.name(),
                               QTYPE::UINFO.code())
            ),
            101 => Ok(
                QTYPEInfo::new(QTYPE::UID.name(),
                               QTYPE::UID.code())
            ),
            102 => Ok(
                QTYPEInfo::new(QTYPE::GID.name(),
                               QTYPE::GID.code())
            ),
            103 => Ok(
                QTYPEInfo::new(QTYPE::UNSPEC.name(),
                               QTYPE::UNSPEC.code())
            ),
            104 => Ok(
                QTYPEInfo::new(QTYPE::NID.name(),
                               QTYPE::NID.code())
            ),
            105 => Ok(
                QTYPEInfo::new(QTYPE::L32.name(),
                               QTYPE::L32.code())
            ),
            106 => Ok(
                QTYPEInfo::new(QTYPE::L64.name(),
                               QTYPE::L64.code())
            ),
            107 => Ok(
                QTYPEInfo::new(QTYPE::LP.name(),
                               QTYPE::LP.code())
            ),
            108 => Ok(
                QTYPEInfo::new(QTYPE::EUI48.name(),
                               QTYPE::EUI48.code())
            ),
            109 => Ok(
                QTYPEInfo::new(QTYPE::EUI64.name(),
                               QTYPE::EUI64.code())
            ),
            110..=248 => Ok(
                QTYPEInfo::new(UNASSIGNED,
                               *decimal)
            ),
            249 => Ok(
                QTYPEInfo::new(QTYPE::TKEY.name(),
                               QTYPE::TKEY.code())
            ),
            250 => Ok(
                QTYPEInfo::new(QTYPE::TSIG.name(),
                               QTYPE::TSIG.code())
            ),
            251 => Ok(
                QTYPEInfo::new(QTYPE::IXFR.name(),
                               QTYPE::IXFR.code())
            ),
            252 => Ok(
                QTYPEInfo::new(QTYPE::AXFR.name(),
                               QTYPE::AXFR.code())
            ),
            253 => Ok(
                QTYPEInfo::new(QTYPE::MAILB.name(),
                               QTYPE::MAILB.code())
            ),
            254 => Ok(
                QTYPEInfo::new(QTYPE::MAILA.name(),
                               QTYPE::MAILA.code())
            ),
            255 => Ok(
                QTYPEInfo::new(QTYPE::ANY.name(),
                               QTYPE::ANY.code())
            ),
            256 => Ok(
                QTYPEInfo::new(QTYPE::URI.name(),
                               QTYPE::URI.code())
            ),
            257 => Ok(
                QTYPEInfo::new(QTYPE::CAA.name(),
                               QTYPE::CAA.code())
            ),
            258 => Ok(
                QTYPEInfo::new(QTYPE::AVC.name(),
                               QTYPE::AVC.code())
            ),
            259 => Ok(
                QTYPEInfo::new(QTYPE::DOA.name(),
                               QTYPE::DOA.code())
            ),
            260 => Ok(
                QTYPEInfo::new(QTYPE::AMTRELAY.name(),
                               QTYPE::AMTRELAY.code())
            ),
            261..=32767 => Ok(
                QTYPEInfo::new(UNASSIGNED,
                               *decimal)
            ),
            32768 => Ok(
                QTYPEInfo::new(QTYPE::TA.name(),
                               QTYPE::TA.code())
            ),
            32769 => Ok(
                QTYPEInfo::new(QTYPE::DLV.name(),
                               QTYPE::DLV.code())
            ),
            37770..=65279 => Ok(
                QTYPEInfo::new(UNASSIGNED,
                               *decimal)
            ),
            65280..=65534 => Ok(
                QTYPEInfo::new(PRIVATE_USE,
                               *decimal)
            ),
            65535 => Ok(
                QTYPEInfo::new(REVERSED,
                               *decimal)
            ),
            _ => Err(String::from("Can't decode QTYPE!"))
        }
    }
}
