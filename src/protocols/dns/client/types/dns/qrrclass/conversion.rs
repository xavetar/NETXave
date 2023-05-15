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

use super::{QRRCLASS};
use super::{QRRCLASSInfo};
use super::{REVERSED, UNASSIGNED, REVERSED_PRIVATE_USE};


pub trait QRRCLASSConversion {
    fn encode(qclass: &str) -> Result<QRRCLASSInfo, String>;
    fn decode(dec: &u16) -> Result<QRRCLASSInfo, String>;
}

impl QRRCLASSConversion for QRRCLASS {
    fn encode(qclass: &str) -> Result<QRRCLASSInfo, String> {
        return match qclass {
            "IN" => Ok(
                QRRCLASSInfo::new(QRRCLASS::IN.name(),
                                  QRRCLASS::IN.code())
            ),
            "CS" => Ok(
                QRRCLASSInfo::new(QRRCLASS::CS.name(),
                                  QRRCLASS::CS.code())
            ),
            "CH" => Ok(
                QRRCLASSInfo::new(QRRCLASS::CH.name(),
                                  QRRCLASS::CH.code())
            ),
            "HS" => Ok(
                QRRCLASSInfo::new(QRRCLASS::HS.name(),
                                  QRRCLASS::HS.code())
            ),
            "None" => Ok(
                QRRCLASSInfo::new(QRRCLASS::None.name(),
                                  QRRCLASS::None.code())
            ),
            "*" => Ok(
                QRRCLASSInfo::new(QRRCLASS::ANY.name(),
                                  QRRCLASS::ANY.code())
            ),
            _ => Err(String::from("Can't encode QCLASS!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QRRCLASSInfo, String> {
        return match *decimal {
            0 => Ok(
                QRRCLASSInfo::new(REVERSED,
                                  *decimal)
            ),
            1 => Ok(
                QRRCLASSInfo::new(QRRCLASS::IN.name(),
                                  QRRCLASS::IN.code())
            ),
            2 => Ok(
                QRRCLASSInfo::new(QRRCLASS::CS.name(),
                                  QRRCLASS::CS.code())
            ),
            3 => Ok(
                QRRCLASSInfo::new(QRRCLASS::CS.name(),
                                  QRRCLASS::CS.code())
            ),
            4 => Ok(
                QRRCLASSInfo::new(QRRCLASS::HS.name(),
                                  QRRCLASS::HS.code())
            ),
            5..=253 => Ok(
                QRRCLASSInfo::new(UNASSIGNED,
                                  *decimal)
            ),
            254 => Ok(
                QRRCLASSInfo::new(QRRCLASS::None.name(),
                                  QRRCLASS::None.code())
            ),
            255 => Ok(
                QRRCLASSInfo::new(QRRCLASS::ANY.name(),
                                  QRRCLASS::ANY.code())
            ),
            256..=65279 => Ok(
                QRRCLASSInfo::new(UNASSIGNED,
                                  *decimal)
            ),
            65280..=65534 => Ok(
                QRRCLASSInfo::new(REVERSED_PRIVATE_USE,
                                  *decimal)
            ),
            65535 => Ok(
                QRRCLASSInfo::new(REVERSED,
                                  *decimal)
            ),
            _ => Err(String::from("Can't decode QCLASS!"))
        }
    }
}
