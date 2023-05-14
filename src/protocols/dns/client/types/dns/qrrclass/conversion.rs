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

use super::{QCLASS};
use super::{QCLASSInfo};
use super::{REVERSED, UNASSIGNED, REVERSED_PRIVATE_USE};


pub trait QCLASSConversion {
    fn encode(qclass: &str) -> Result<QCLASSInfo, String>;
    fn decode(dec: &u16) -> Result<QCLASSInfo, String>;
}

impl QCLASSConversion for QCLASS {
    fn encode(qclass: &str) -> Result<QCLASSInfo, String> {
        return match qclass {
            "IN" => Ok(
                QCLASSInfo::new(QCLASS::IN.name(),
                                QCLASS::IN.code())
            ),
            "CS" => Ok(
                QCLASSInfo::new(QCLASS::CS.name(),
                                QCLASS::CS.code())
            ),
            "CH" => Ok(
                QCLASSInfo::new(QCLASS::CH.name(),
                                QCLASS::CH.code())
            ),
            "HS" => Ok(
                QCLASSInfo::new(QCLASS::HS.name(),
                                QCLASS::HS.code())
            ),
            "None" => Ok(
                QCLASSInfo::new(QCLASS::None.name(),
                                QCLASS::None.code())
            ),
            "*" => Ok(
                QCLASSInfo::new(QCLASS::ANY.name(),
                                QCLASS::ANY.code())
            ),
            _ => Err(String::from("Can't encode QCLASS!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QCLASSInfo, String> {
        return match *decimal {
            0 => Ok(
                QCLASSInfo::new(REVERSED,
                                *decimal)
            ),
            1 => Ok(
                QCLASSInfo::new(QCLASS::IN.name(),
                                QCLASS::IN.code())
            ),
            2 => Ok(
                QCLASSInfo::new(QCLASS::CS.name(),
                                QCLASS::CS.code())
            ),
            3 => Ok(
                QCLASSInfo::new(QCLASS::CS.name(),
                                QCLASS::CS.code())
            ),
            4 => Ok(
                QCLASSInfo::new(QCLASS::HS.name(),
                                QCLASS::HS.code())
            ),
            5..=253 => Ok(
                QCLASSInfo::new(UNASSIGNED,
                                *decimal)
            ),
            254 => Ok(
                QCLASSInfo::new(QCLASS::None.name(),
                                QCLASS::None.code())
            ),
            255 => Ok(
                QCLASSInfo::new(QCLASS::ANY.name(),
                                QCLASS::ANY.code())
            ),
            256..=65279 => Ok(
                QCLASSInfo::new(UNASSIGNED,
                                *decimal)
            ),
            65280..=65534 => Ok(
                QCLASSInfo::new(REVERSED_PRIVATE_USE,
                                *decimal)
            ),
            65535 => Ok(
                QCLASSInfo::new(REVERSED,
                                *decimal)
            ),
            _ => Err(String::from("Can't decode QCLASS!"))
        }
    }
}
