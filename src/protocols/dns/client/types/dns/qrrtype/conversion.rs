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

use super::{QRRTYPE};
use super::{QRRTYPEInfo};
use super::{REVERSED, UNASSIGNED, PRIVATE_USE};

pub trait QRRTYPEConversion {
    fn encode(qclass: &str) -> Result<QRRTYPEInfo, String>;
    fn decode(dec: &u16) -> Result<QRRTYPEInfo, String>;
}

impl QRRTYPEConversion for QRRTYPE {
    fn encode(qtype: &str) -> Result<QRRTYPEInfo, String> {
        return match qtype {
            "A" => Ok(
                QRRTYPEInfo::new(QRRTYPE::A.name(),
                                 QRRTYPE::A.code())
            ),
            "NS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NS.name(),
                                 QRRTYPE::NS.code())
            ),
            "MD" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MD.name(),
                                 QRRTYPE::MD.code())
            ),
            "MF" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MF.name(),
                                 QRRTYPE::MF.code())
            ),
            "CNAME" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CNAME.name(),
                                 QRRTYPE::CNAME.code())
            ),
            "SOA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SOA.name(),
                                 QRRTYPE::SOA.code())
            ),
            "MB" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MB.name(),
                                 QRRTYPE::MB.code())
            ),
            "MG" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MG.name(),
                                 QRRTYPE::MG.code())
            ),
            "MR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MR.name(),
                                 QRRTYPE::MR.code())
            ),
            "NULL" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NULL.name(),
                                 QRRTYPE::NULL.code())
            ),
            "WKS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::WKS.name(),
                                 QRRTYPE::WKS.code())
            ),
            "PTR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::PTR.name(),
                                 QRRTYPE::PTR.code())
            ),
            "HINFO" => Ok(
                QRRTYPEInfo::new(QRRTYPE::HINFO.name(),
                                 QRRTYPE::HINFO.code())
            ),
            "MINFO" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MINFO.name(),
                                 QRRTYPE::MINFO.code())
            ),
            "MX" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MX.name(),
                                 QRRTYPE::MX.code())
            ),
            "TXT" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TXT.name(),
                                 QRRTYPE::TXT.code())
            ),
            "RP" => Ok(
                QRRTYPEInfo::new(QRRTYPE::RP.name(),
                                 QRRTYPE::RP.code())
            ),
            "AFSDB" => Ok(
                QRRTYPEInfo::new(QRRTYPE::AFSDB.name(),
                                 QRRTYPE::AFSDB.code())
            ),
            "X25" => Ok(
                QRRTYPEInfo::new(QRRTYPE::X25.name(),
                                 QRRTYPE::X25.code())
            ),
            "ISDN" => Ok(
                QRRTYPEInfo::new(QRRTYPE::ISDN.name(),
                                 QRRTYPE::ISDN.code())
            ),
            "RT" => Ok(
                QRRTYPEInfo::new(QRRTYPE::RT.name(),
                                 QRRTYPE::RT.code())
            ),
            "NSAP" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSAP.name(),
                                 QRRTYPE::NSAP.code())
            ),
            "NSAP-PTR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSAP_PTR.name(),
                                 QRRTYPE::NSAP_PTR.code())
            ),
            "SIG" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SIG.name(),
                                 QRRTYPE::SIG.code())
            ),
            "KEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::KEY.name(),
                                 QRRTYPE::KEY.code())
            ),
            "PX" => Ok(
                QRRTYPEInfo::new(QRRTYPE::PX.name(),
                                 QRRTYPE::PX.code())
            ),
            "GPOS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::GPOS.name(),
                                 QRRTYPE::GPOS.code())
            ),
            "AAAA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::AAAA.name(),
                                 QRRTYPE::AAAA.code())
            ),
            "LOC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::LOC.name(),
                                 QRRTYPE::LOC.code())
            ),
            "NXT" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NXT.name(),
                                 QRRTYPE::NXT.code())
            ),
            "EID" => Ok(
                QRRTYPEInfo::new(QRRTYPE::EID.name(),
                                 QRRTYPE::EID.code())
            ),
            "NIMLOC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NIMLOC.name(),
                                 QRRTYPE::NIMLOC.code())
            ),
            "SRV" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SRV.name(),
                                 QRRTYPE::SRV.code())
            ),
            "ATMA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::ATMA.name(),
                                 QRRTYPE::ATMA.code())
            ),
            "NAPTR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NAPTR.name(),
                                 QRRTYPE::NAPTR.code())
            ),
            "KX" => Ok(
                QRRTYPEInfo::new(QRRTYPE::KX.name(),
                                 QRRTYPE::KX.code())
            ),
            "CERT" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CERT.name(),
                                 QRRTYPE::CERT.code())
            ),
            "A6" => Ok(
                QRRTYPEInfo::new(QRRTYPE::A6.name(),
                                 QRRTYPE::A6.code())
            ),
            "DNAME" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DNAME.name(),
                                 QRRTYPE::DNAME.code())
            ),
            "SINK" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SINK.name(),
                                 QRRTYPE::SINK.code())
            ),
            "OPT" => Ok(
                QRRTYPEInfo::new(QRRTYPE::OPT.name(),
                                 QRRTYPE::OPT.code())
            ),
            "APL" => Ok(
                QRRTYPEInfo::new(QRRTYPE::APL.name(),
                                 QRRTYPE::APL.code())
            ),
            "DS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DS.name(),
                                 QRRTYPE::DS.code())
            ),
            "SSHFP" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SSHFP.name(),
                                 QRRTYPE::SSHFP.code())
            ),
            "IPSECKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::IPSECKEY.name(),
                                 QRRTYPE::IPSECKEY.code())
            ),
            "RRSIG" => Ok(
                QRRTYPEInfo::new(QRRTYPE::RRSIG.name(),
                                 QRRTYPE::RRSIG.code())
            ),
            "NSEC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC.name(),
                                 QRRTYPE::NSEC.code())
            ),
            "DNSKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DNSKEY.name(),
                                 QRRTYPE::DNSKEY.code())
            ),
            "DHCID" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DHCID.name(),
                                 QRRTYPE::DHCID.code())
            ),
            "NSEC3" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC3.name(),
                                 QRRTYPE::NSEC3.code())
            ),
            "NSEC3PARAM" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC3PARAM.name(),
                                 QRRTYPE::NSEC3PARAM.code())
            ),
            "TLSA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TLSA.name(),
                                 QRRTYPE::TLSA.code())
            ),
            "SMIMEA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SMIMEA.name(),
                                 QRRTYPE::SMIMEA.code())
            ),
            "HIP" => Ok(
                QRRTYPEInfo::new(QRRTYPE::HIP.name(),
                                 QRRTYPE::HIP.code())
            ),
            "NINFO" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NINFO.name(),
                                 QRRTYPE::NINFO.code())
            ),
            "RKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::RKEY.name(),
                                 QRRTYPE::RKEY.code())
            ),
            "TALINK" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TALINK.name(),
                                 QRRTYPE::TALINK.code())
            ),
            "CDS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CDS.name(),
                                 QRRTYPE::CDS.code())
            ),
            "CDNSKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CDNSKEY.name(),
                                 QRRTYPE::CDNSKEY.code())
            ),
            "OPENPGPKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::OPENPGPKEY.name(),
                                 QRRTYPE::OPENPGPKEY.code())
            ),
            "CSYNC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CSYNC.name(),
                                 QRRTYPE::CSYNC.code())
            ),
            "ZONEMD" => Ok(
                QRRTYPEInfo::new(QRRTYPE::ZONEMD.name(),
                                 QRRTYPE::ZONEMD.code())
            ),
            "SVCB" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SVCB.name(),
                                 QRRTYPE::SVCB.code())
            ),
            "HTTPS" => Ok(
                QRRTYPEInfo::new(QRRTYPE::HTTPS.name(),
                                 QRRTYPE::HTTPS.code())
            ),
            "SPF" => Ok(
                QRRTYPEInfo::new(QRRTYPE::SPF.name(),
                                 QRRTYPE::SPF.code())
            ),
            "UINFO" => Ok(
                QRRTYPEInfo::new(QRRTYPE::UINFO.name(),
                                 QRRTYPE::UINFO.code())
            ),
            "UID" => Ok(
                QRRTYPEInfo::new(QRRTYPE::UID.name(),
                                 QRRTYPE::UID.code())
            ),
            "GID" => Ok(
                QRRTYPEInfo::new(QRRTYPE::GID.name(),
                                 QRRTYPE::GID.code())
            ),
            "UNSPEC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::UNSPEC.name(),
                                 QRRTYPE::UNSPEC.code())
            ),
            "NID" => Ok(
                QRRTYPEInfo::new(QRRTYPE::NID.name(),
                                 QRRTYPE::NID.code())
            ),
            "L32" => Ok(
                QRRTYPEInfo::new(QRRTYPE::L32.name(),
                                 QRRTYPE::L32.code())
            ),
            "L64" => Ok(
                QRRTYPEInfo::new(QRRTYPE::L64.name(),
                                 QRRTYPE::L64.code())
            ),
            "LP" => Ok(
                QRRTYPEInfo::new(QRRTYPE::LP.name(),
                                 QRRTYPE::LP.code())
            ),
            "EUI48" => Ok(
                QRRTYPEInfo::new(QRRTYPE::EUI48.name(),
                                 QRRTYPE::EUI48.code())
            ),
            "EUI64" => Ok(
                QRRTYPEInfo::new(QRRTYPE::EUI64.name(),
                                 QRRTYPE::EUI64.code())
            ),
            "TKEY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TKEY.name(),
                                 QRRTYPE::TKEY.code())
            ),
            "TSIG" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TSIG.name(),
                                 QRRTYPE::TSIG.code())
            ),
            "IXFR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::IXFR.name(),
                                 QRRTYPE::IXFR.code())
            ),
            "AXFR" => Ok(
                QRRTYPEInfo::new(QRRTYPE::AXFR.name(),
                                 QRRTYPE::AXFR.code())
            ),
            "MAILB" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MAILB.name(),
                                 QRRTYPE::MAILB.code())
            ),
            "MAILA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::MAILA.name(),
                                 QRRTYPE::MAILA.code())
            ),
            "*" => Ok(
                QRRTYPEInfo::new(QRRTYPE::ANY.name(),
                                 QRRTYPE::ANY.code())
            ),
            "URI" => Ok(
                QRRTYPEInfo::new(QRRTYPE::URI.name(),
                                 QRRTYPE::URI.code())
            ),
            "CAA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::CAA.name(),
                                 QRRTYPE::CAA.code())
            ),
            "AVC" => Ok(
                QRRTYPEInfo::new(QRRTYPE::AVC.name(),
                                 QRRTYPE::AVC.code())
            ),
            "DOA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DOA.name(),
                                 QRRTYPE::DOA.code())
            ),
            "AMTRELAY" => Ok(
                QRRTYPEInfo::new(QRRTYPE::AMTRELAY.name(),
                                 QRRTYPE::AMTRELAY.code())
            ),
            "TA" => Ok(
                QRRTYPEInfo::new(QRRTYPE::TA.name(),
                                 QRRTYPE::TA.code())
            ),
            "DLV" => Ok(
                QRRTYPEInfo::new(QRRTYPE::DLV.name(),
                                 QRRTYPE::DLV.code())
            ),
            _ => Err(String::from("Can't encode QTYPE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QRRTYPEInfo, String> {
        return match *decimal {
            0 => Ok(
                QRRTYPEInfo::new(REVERSED,
                                 *decimal)
            ),
            1 => Ok(
                QRRTYPEInfo::new(QRRTYPE::A.name(),
                                 QRRTYPE::A.code())
            ),
            2 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NS.name(),
                                 QRRTYPE::NS.code())
            ),
            3 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MD.name(),
                                 QRRTYPE::MD.code())
            ),
            4 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MF.name(),
                                 QRRTYPE::MF.code())
            ),
            5 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CNAME.name(),
                                 QRRTYPE::CNAME.code())
            ),
            6 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SOA.name(),
                                 QRRTYPE::SOA.code())
            ),
            7 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MB.name(),
                                 QRRTYPE::MB.code())
            ),
            8 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MG.name(),
                                 QRRTYPE::MG.code())
            ),
            9 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MR.name(),
                                 QRRTYPE::MR.code())
            ),
            10 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NULL.name(),
                                 QRRTYPE::NULL.code())
            ),
            11 => Ok(
                QRRTYPEInfo::new(QRRTYPE::WKS.name(),
                                 QRRTYPE::WKS.code())
            ),
            12 => Ok(
                QRRTYPEInfo::new(QRRTYPE::PTR.name(),
                                 QRRTYPE::PTR.code())
            ),
            13 => Ok(
                QRRTYPEInfo::new(QRRTYPE::HINFO.name(),
                                 QRRTYPE::HINFO.code())
            ),
            14 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MINFO.name(),
                                 QRRTYPE::MINFO.code())
            ),
            15 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MX.name(),
                                 QRRTYPE::MX.code())
            ),
            16 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TXT.name(),
                                 QRRTYPE::TXT.code())
            ),
            17 => Ok(
                QRRTYPEInfo::new(QRRTYPE::RP.name(),
                                 QRRTYPE::RP.code())
            ),
            18 => Ok(
                QRRTYPEInfo::new(QRRTYPE::AFSDB.name(),
                                 QRRTYPE::AFSDB.code())
            ),
            19 => Ok(
                QRRTYPEInfo::new(QRRTYPE::X25.name(),
                                 QRRTYPE::X25.code())
            ),
            20 => Ok(
                QRRTYPEInfo::new(QRRTYPE::ISDN.name(),
                                 QRRTYPE::ISDN.code())
            ),
            21 => Ok(
                QRRTYPEInfo::new(QRRTYPE::RT.name(),
                                 QRRTYPE::RT.code())
            ),
            22 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSAP.name(),
                                 QRRTYPE::NSAP.code())
            ),
            23 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSAP_PTR.name(),
                                 QRRTYPE::NSAP_PTR.code())
            ),
            24 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SIG.name(),
                                 QRRTYPE::SIG.code())
            ),
            25 => Ok(
                QRRTYPEInfo::new(QRRTYPE::KEY.name(),
                                 QRRTYPE::KEY.code())
            ),
            26 => Ok(
                QRRTYPEInfo::new(QRRTYPE::PX.name(),
                                 QRRTYPE::PX.code())
            ),
            27 => Ok(
                QRRTYPEInfo::new(QRRTYPE::GPOS.name(),
                                 QRRTYPE::GPOS.code())
            ),
            28 => Ok(
                QRRTYPEInfo::new(QRRTYPE::AAAA.name(),
                                 QRRTYPE::AAAA.code())
            ),
            29 => Ok(
                QRRTYPEInfo::new(QRRTYPE::LOC.name(),
                                 QRRTYPE::LOC.code())
            ),
            30 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NXT.name(),
                                 QRRTYPE::NXT.code())
            ),
            31 => Ok(
                QRRTYPEInfo::new(QRRTYPE::EID.name(),
                                 QRRTYPE::EID.code())
            ),
            32 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NIMLOC.name(),
                                 QRRTYPE::NIMLOC.code())
            ),
            33 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SRV.name(),
                                 QRRTYPE::SRV.code())
            ),
            34 => Ok(
                QRRTYPEInfo::new(QRRTYPE::ATMA.name(),
                                 QRRTYPE::ATMA.code())
            ),
            35 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NAPTR.name(),
                                 QRRTYPE::NAPTR.code())
            ),
            36 => Ok(
                QRRTYPEInfo::new(QRRTYPE::KX.name(),
                                 QRRTYPE::KX.code())
            ),
            37 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CERT.name(),
                                 QRRTYPE::CERT.code())
            ),
            38 => Ok(
                QRRTYPEInfo::new(QRRTYPE::A6.name(),
                                 QRRTYPE::A6.code())
            ),
            39 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DNAME.name(),
                                 QRRTYPE::DNAME.code())
            ),
            40 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SINK.name(),
                                 QRRTYPE::SINK.code())
            ),
            41 => Ok(
                QRRTYPEInfo::new(QRRTYPE::OPT.name(),
                                 QRRTYPE::OPT.code())
            ),
            42 => Ok(
                QRRTYPEInfo::new(QRRTYPE::APL.name(),
                                 QRRTYPE::APL.code())
            ),
            43 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DS.name(),
                                 QRRTYPE::DS.code())
            ),
            44 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SSHFP.name(),
                                 QRRTYPE::SSHFP.code())
            ),
            45 => Ok(
                QRRTYPEInfo::new(QRRTYPE::IPSECKEY.name(),
                                 QRRTYPE::IPSECKEY.code())
            ),
            46 => Ok(
                QRRTYPEInfo::new(QRRTYPE::RRSIG.name(),
                                 QRRTYPE::RRSIG.code())
            ),
            47 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC.name(),
                                 QRRTYPE::NSEC.code())
            ),
            48 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DNSKEY.name(),
                                 QRRTYPE::DNSKEY.code())
            ),
            49 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DHCID.name(),
                                 QRRTYPE::DHCID.code())
            ),
            50 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC3.name(),
                                 QRRTYPE::NSEC3.code())
            ),
            51 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NSEC3PARAM.name(),
                                 QRRTYPE::NSEC3PARAM.code())
            ),
            52 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TLSA.name(),
                                 QRRTYPE::TLSA.code())
            ),
            53 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SMIMEA.name(),
                                 QRRTYPE::SMIMEA.code())
            ),
            54 => Ok(
                QRRTYPEInfo::new(UNASSIGNED,
                                 *decimal)
            ),
            55 => Ok(
                QRRTYPEInfo::new(QRRTYPE::HIP.name(),
                                 QRRTYPE::HIP.code())
            ),
            56 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NINFO.name(),
                                 QRRTYPE::NINFO.code())
            ),
            57 => Ok(
                QRRTYPEInfo::new(QRRTYPE::RKEY.name(),
                                 QRRTYPE::RKEY.code())
            ),
            58 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TALINK.name(),
                                 QRRTYPE::TALINK.code())
            ),
            59 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CDS.name(),
                                 QRRTYPE::CDS.code())
            ),
            60 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CDNSKEY.name(),
                                 QRRTYPE::CDNSKEY.code())
            ),
            61 => Ok(
                QRRTYPEInfo::new(QRRTYPE::OPENPGPKEY.name(),
                                 QRRTYPE::OPENPGPKEY.code())
            ),
            62 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CSYNC.name(),
                                 QRRTYPE::CSYNC.code())
            ),
            63 => Ok(
                QRRTYPEInfo::new(QRRTYPE::ZONEMD.name(),
                                 QRRTYPE::ZONEMD.code())
            ),
            64 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SVCB.name(),
                                 QRRTYPE::SVCB.code())
            ),
            65 => Ok(
                QRRTYPEInfo::new(QRRTYPE::HTTPS.name(),
                                 QRRTYPE::HTTPS.code())
            ),
            66..=98 => Ok(
                QRRTYPEInfo::new(UNASSIGNED,
                                 *decimal)
            ),
            99 => Ok(
                QRRTYPEInfo::new(QRRTYPE::SPF.name(),
                                 QRRTYPE::SPF.code())
            ),
            100 => Ok(
                QRRTYPEInfo::new(QRRTYPE::UINFO.name(),
                                 QRRTYPE::UINFO.code())
            ),
            101 => Ok(
                QRRTYPEInfo::new(QRRTYPE::UID.name(),
                                 QRRTYPE::UID.code())
            ),
            102 => Ok(
                QRRTYPEInfo::new(QRRTYPE::GID.name(),
                                 QRRTYPE::GID.code())
            ),
            103 => Ok(
                QRRTYPEInfo::new(QRRTYPE::UNSPEC.name(),
                                 QRRTYPE::UNSPEC.code())
            ),
            104 => Ok(
                QRRTYPEInfo::new(QRRTYPE::NID.name(),
                                 QRRTYPE::NID.code())
            ),
            105 => Ok(
                QRRTYPEInfo::new(QRRTYPE::L32.name(),
                                 QRRTYPE::L32.code())
            ),
            106 => Ok(
                QRRTYPEInfo::new(QRRTYPE::L64.name(),
                                 QRRTYPE::L64.code())
            ),
            107 => Ok(
                QRRTYPEInfo::new(QRRTYPE::LP.name(),
                                 QRRTYPE::LP.code())
            ),
            108 => Ok(
                QRRTYPEInfo::new(QRRTYPE::EUI48.name(),
                                 QRRTYPE::EUI48.code())
            ),
            109 => Ok(
                QRRTYPEInfo::new(QRRTYPE::EUI64.name(),
                                 QRRTYPE::EUI64.code())
            ),
            110..=248 => Ok(
                QRRTYPEInfo::new(UNASSIGNED,
                                 *decimal)
            ),
            249 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TKEY.name(),
                                 QRRTYPE::TKEY.code())
            ),
            250 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TSIG.name(),
                                 QRRTYPE::TSIG.code())
            ),
            251 => Ok(
                QRRTYPEInfo::new(QRRTYPE::IXFR.name(),
                                 QRRTYPE::IXFR.code())
            ),
            252 => Ok(
                QRRTYPEInfo::new(QRRTYPE::AXFR.name(),
                                 QRRTYPE::AXFR.code())
            ),
            253 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MAILB.name(),
                                 QRRTYPE::MAILB.code())
            ),
            254 => Ok(
                QRRTYPEInfo::new(QRRTYPE::MAILA.name(),
                                 QRRTYPE::MAILA.code())
            ),
            255 => Ok(
                QRRTYPEInfo::new(QRRTYPE::ANY.name(),
                                 QRRTYPE::ANY.code())
            ),
            256 => Ok(
                QRRTYPEInfo::new(QRRTYPE::URI.name(),
                                 QRRTYPE::URI.code())
            ),
            257 => Ok(
                QRRTYPEInfo::new(QRRTYPE::CAA.name(),
                                 QRRTYPE::CAA.code())
            ),
            258 => Ok(
                QRRTYPEInfo::new(QRRTYPE::AVC.name(),
                                 QRRTYPE::AVC.code())
            ),
            259 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DOA.name(),
                                 QRRTYPE::DOA.code())
            ),
            260 => Ok(
                QRRTYPEInfo::new(QRRTYPE::AMTRELAY.name(),
                                 QRRTYPE::AMTRELAY.code())
            ),
            261..=32767 => Ok(
                QRRTYPEInfo::new(UNASSIGNED,
                                 *decimal)
            ),
            32768 => Ok(
                QRRTYPEInfo::new(QRRTYPE::TA.name(),
                                 QRRTYPE::TA.code())
            ),
            32769 => Ok(
                QRRTYPEInfo::new(QRRTYPE::DLV.name(),
                                 QRRTYPE::DLV.code())
            ),
            37770..=65279 => Ok(
                QRRTYPEInfo::new(UNASSIGNED,
                                 *decimal)
            ),
            65280..=65534 => Ok(
                QRRTYPEInfo::new(PRIVATE_USE,
                                 *decimal)
            ),
            65535 => Ok(
                QRRTYPEInfo::new(REVERSED,
                                 *decimal)
            ),
            _ => Err(String::from("Can't decode QTYPE!"))
        }
    }
}
