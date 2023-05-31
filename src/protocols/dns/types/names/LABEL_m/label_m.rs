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

use super::{U2};
use super::{LABEL_Details};
use super::{UNALLOCATED};
use super::{TwoResult, TwoResult::First, TwoResult::Second};

pub enum LABEL {
    NORMAL = 0,
    EXTENDED = 1,
    COMPRESSED = 3,
}

impl LABEL {
    pub fn code(&self) -> u8 {
        return match self {
            LABEL::NORMAL => U2::new(LABEL::NORMAL as u8).get() << 6,
            LABEL::EXTENDED => U2::new(LABEL::EXTENDED as u8).get() << 6,
            LABEL::COMPRESSED => U2::new(LABEL::COMPRESSED as u8).get() << 6
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            LABEL::NORMAL => "Normal Label",
            LABEL::EXTENDED => "Extended Label",
            LABEL::COMPRESSED => "Compressed Label"
        }
    }

    pub fn details(&self) -> &'static str {
        return match self {
            LABEL::NORMAL => "Lower 6 bits is the length of the label",
            LABEL::EXTENDED => "The lower 6 bits and the 8 bits from next octet form a pointer to the compression target",
            LABEL::COMPRESSED => "Type the lower 6 bits of this type indicate the type of label in use"
        }
    }

    pub fn encode(t: &str) -> LABEL {
        return match t {
            "NORMAL" => LABEL::NORMAL,
            "EXTENDED" => LABEL::EXTENDED,
            "COMPRESSED" => LABEL::COMPRESSED,
            _ => panic!("Can't encode LABEL!")
        }
    }

    pub fn decode(t: &U2) -> TwoResult<LABEL, LABEL_Details> {
        return match t.get() {
            0 => First(LABEL::NORMAL),
            1 => First(LABEL::EXTENDED),
            2 => Second(LABEL_Details::new(UNALLOCATED, U2::new(t.get()), UNALLOCATED)),
            3 => First(LABEL::COMPRESSED),
            _ => panic!("Can't decode LABEL!")
        }
    }
}
