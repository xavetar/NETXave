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

use super::{CLASS};
use super::{CLASSInfo};
use super::{RESERVED, UNASSIGNED, RESERVED_PRIVATE_USE};


pub trait CLASSConversion {
    fn encode(qclass: &str) -> Result<CLASSInfo, String>;
    fn decode(dec: &u16) -> Result<CLASSInfo, String>;
}

impl CLASSConversion for CLASS {
    fn encode(qclass: &str) -> Result<CLASSInfo, String> {
        return match qclass {
            "IN" => Ok(
                CLASSInfo::new(CLASS::IN.name(),
                               CLASS::IN.code())
            ),
            "CS" => Ok(
                CLASSInfo::new(CLASS::CS.name(),
                               CLASS::CS.code())
            ),
            "CH" => Ok(
                CLASSInfo::new(CLASS::CH.name(),
                               CLASS::CH.code())
            ),
            "HS" => Ok(
                CLASSInfo::new(CLASS::HS.name(),
                               CLASS::HS.code())
            ),
            "None" => Ok(
                CLASSInfo::new(CLASS::None.name(),
                               CLASS::None.code())
            ),
            "*" => Ok(
                CLASSInfo::new(CLASS::ANY.name(),
                               CLASS::ANY.code())
            ),
            _ => Err(String::from("Can't encode CLASS!"))
        }
    }

    fn decode(decimal: &u16) -> Result<CLASSInfo, String> {
        return match *decimal {
            0 => Ok(
                CLASSInfo::new(RESERVED,
                               *decimal)
            ),
            1 => Ok(
                CLASSInfo::new(CLASS::IN.name(),
                               CLASS::IN.code())
            ),
            2 => Ok(
                CLASSInfo::new(CLASS::CS.name(),
                               CLASS::CS.code())
            ),
            3 => Ok(
                CLASSInfo::new(CLASS::CS.name(),
                               CLASS::CS.code())
            ),
            4 => Ok(
                CLASSInfo::new(CLASS::HS.name(),
                               CLASS::HS.code())
            ),
            5..=253 => Ok(
                CLASSInfo::new(UNASSIGNED,
                               *decimal)
            ),
            254 => Ok(
                CLASSInfo::new(CLASS::None.name(),
                               CLASS::None.code())
            ),
            255 => Ok(
                CLASSInfo::new(CLASS::ANY.name(),
                               CLASS::ANY.code())
            ),
            256..=65279 => Ok(
                CLASSInfo::new(UNASSIGNED,
                               *decimal)
            ),
            65280..=65534 => Ok(
                CLASSInfo::new(RESERVED_PRIVATE_USE,
                               *decimal)
            ),
            65535 => Ok(
                CLASSInfo::new(RESERVED,
                               *decimal)
            ),
            _ => Err(String::from("Can't decode CLASS!"))
        }
    }
}
