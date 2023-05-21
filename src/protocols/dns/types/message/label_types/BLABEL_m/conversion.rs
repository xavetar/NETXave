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

use crate::data_types::{U2};

use super::{UNALLOCATED};
use super::{BLABEL};
use super::{BLABELInfo};

pub trait BLABELConversion {
    fn encode(name: &str) -> Result<BLABELInfo, String>;
    fn decode(dec: &u8) -> Result<BLABELInfo, String>;
}

impl BLABELConversion for BLABEL {
    fn encode(name: &str) -> Result<BLABELInfo, String> {
        return match name {
            "NORMAL" => Ok(
                BLABELInfo::new(BLABEL::NORMAL.name(),
                                BLABEL::NORMAL.code(),
                                BLABEL::NORMAL.details())
            ),
            "EXTENDED" => Ok(
                BLABELInfo::new(BLABEL::EXTENDED.name(),
                                BLABEL::EXTENDED.code(),
                                BLABEL::EXTENDED.details())
            ),
            "COMPRESSED" => Ok(
                BLABELInfo::new(BLABEL::COMPRESSED.name(),
                                BLABEL::COMPRESSED.code(),
                                BLABEL::COMPRESSED.details())
            ),
            _ => Err(String::from("Can't encode BASIC LABEL!"))
        }
    }

    fn decode(decimal: &u8) -> Result<BLABELInfo, String> {
        return match *decimal {
            0 => Ok(
                BLABELInfo::new(BLABEL::NORMAL.name(),
                                BLABEL::NORMAL.code(),
                                BLABEL::NORMAL.details())
            ),
            1 => Ok(
                BLABELInfo::new(BLABEL::EXTENDED.name(),
                                BLABEL::EXTENDED.code(),
                                BLABEL::EXTENDED.details())
            ),
            2 => Ok(
                BLABELInfo::new(UNALLOCATED,
                                U2::new(*decimal),
                                UNALLOCATED)
            ),
            3 => Ok(
                BLABELInfo::new(BLABEL::COMPRESSED.name(),
                                BLABEL::COMPRESSED.code(),
                                BLABEL::COMPRESSED.details())
            ),
            _ => Err(String::from("Can't decode BASIC LABEL!"))
        }
    }
}
