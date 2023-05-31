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

use crate::protocols::dns::types::data::result::{TwoResult};

use crate::protocols::dns::types::data::integer::{U1, U2, U3, U4, U6, U12};
use crate::protocols::dns::builder::BASE::{Header, Flags, Z};
use crate::protocols::dns::builder::BASE::{Question};

use crate::protocols::dns::types::message::Header::Flags::{
    QR, AA, TC, RD, RA, OPCODE, RCODE
};

use crate::protocols::dns::types::message::Header::Flags::Z::BASE::{NONE};
use crate::protocols::dns::types::message::Question::{NAME::{LABEL}, TYPE, CLASS};
use crate::protocols::dns::types::message::Additional::OPT::RDATA::{OPTION_CODE};

use crate::protocols::dns::types::message::DSO::{DSO_TYPE};

#[derive(Debug)]
pub struct DNSMessage {
    header: Header<Flags>,
    question: Question
}

impl DNSMessage {
    pub fn new(header: Header<Flags>, question: Question) -> DNSMessage {
        DNSMessage {
            header: header,
            question: question
        }
    }
}

fn domain_to_qname(domain: &str) -> Result<Vec<u8>, String> {
    if domain.len() <= 255 {
        let mut qname: Vec<u8> = Vec::<u8>::new();
        let addr_parts: Vec<&str> = domain.split(".").collect();
        for part in addr_parts {
            if part.len() <= 63 {
                // Push Normal Label and use bitwise "OR" for addition with a result of 6 bit length.
                qname.push(LABEL::NORMAL.code()| U6::new(part.len() as u8).get());
                qname.extend_from_slice(part.as_bytes());
            } else {
                return Err(String::from("Label Length is big then 63 chars!"));
            }
        }
        qname.push(0);
        return Ok(qname);
    } else {
        return Err(String::from("Domain Length is big then 255 chars!"));
    }
}

pub fn get_query(domain: &str) -> String {
    let mut header = Header::new(
        43690, Flags::new(
            QR::QUERY.code(),
            OPCODE::QUERY.code(),
            AA::NO.code(),
            TC::NO.code(),
            RD::YES.code(),
            RA::YES.code(),
            Z::new(U3::new(0)),
            RCODE::NoError.code::<u16, U4>()
        ), 1, 0, 0, 0
    );

    /// Question

    let mut question = Question::new(
        domain_to_qname(domain).unwrap(),TYPE::A.code(),CLASS::IN.code()
    );

    /// Message (Query)

    let mut dns_message = DNSMessage::new(header, question);

    let mut bytes_dns_message: Vec<u8> = Vec::<u8>::new();

    // Header:

    // ID
    bytes_dns_message.extend_from_slice(&dns_message.header.get_id().to_be_bytes());

    println!("ID: {:?}", &dns_message.header.get_id().to_be_bytes().iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<String>>().join(""));

    // Flags
    let mut dns_flags: u16 = 0;
    dns_flags |= (dns_message.header.get_flags().get_qr().get() as u16) << 15;
    dns_flags |= (dns_message.header.get_flags().get_opcode().get() as u16) << 11;
    dns_flags |= (dns_message.header.get_flags().get_aa().get() as u16) << 10;
    dns_flags |= (dns_message.header.get_flags().get_tc().get() as u16) << 9;
    dns_flags |= (dns_message.header.get_flags().get_rd().get() as u16) << 8;
    dns_flags |= (dns_message.header.get_flags().get_ra().get() as u16) << 7;
    // SE
    // dns_flags |= (dns_message.header.get_flags().get_z().get_none().get() as u16) << 6;
    // dns_flags |= (dns_message.header.get_flags().get_z().get_ad().get() as u16) << 5;
    // dns_flags |= (dns_message.header.get_flags().get_z().get_cd().get() as u16) << 4;
    // BASE
    dns_flags |= (dns_message.header.get_flags().get_z().get_none().get() as u16) << 4;
    dns_flags |= (dns_message.header.get_flags().get_rcode().get() as u16) << 0;

    bytes_dns_message.extend_from_slice(&dns_flags.to_be_bytes());

    // QDCOUNT
    bytes_dns_message.extend_from_slice(&dns_message.header.get_qdcount().to_be_bytes());
    // ANCOUNT
    bytes_dns_message.extend_from_slice(&dns_message.header.get_ancount().to_be_bytes());
    // NSCOUNT
    bytes_dns_message.extend_from_slice(&dns_message.header.get_nscount().to_be_bytes());
    // ARCOUNT
    bytes_dns_message.extend_from_slice(&dns_message.header.get_arcount().to_be_bytes());

    // Question:

    // QNAME
    bytes_dns_message.extend_from_slice(&dns_message.question.get_qname());
    // QTYPE
    bytes_dns_message.extend_from_slice(&dns_message.question.get_qtype().to_be_bytes());
    // QCLASS
    bytes_dns_message.extend_from_slice(&dns_message.question.get_qclass().to_be_bytes());

    /// TO HEX Repr

    return String::from(bytes_dns_message.iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<String>>().join(""));
}
