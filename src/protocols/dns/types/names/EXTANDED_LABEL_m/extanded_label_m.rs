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

use super::{U6};
use super::{EXTENDED};
use super::{EXTANDED_LABEL_Details};
use super::{RESERVED_FUTURE_EXPANSION};
use super::{TwoResult, TwoResult::First, TwoResult::Second};

pub enum EXTANDED_LABEL {
    BINARY = 1
}

impl EXTANDED_LABEL {
    pub fn code(&self) -> u8 {
        return match self {
            EXTANDED_LABEL::BINARY => EXTENDED.code() | U6::new(EXTANDED_LABEL::BINARY as u8).get()
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            EXTANDED_LABEL::BINARY => "Extanded Binary Label"
        }
    }

    pub fn details(&self) -> &'static str {
        return match self {
            EXTANDED_LABEL::BINARY => "Binary Label Representation"
        }
    }

    pub fn encode(t: &str) -> EXTANDED_LABEL {
        return match t {
            "BINARY" => EXTANDED_LABEL::BINARY,
            _ => panic!("Can't encode EXTANDED LABEL!")
        }
    }

    pub fn decode(t: &U6) -> TwoResult<EXTANDED_LABEL, EXTANDED_LABEL_Details> {
        return match t.get() {
            1 => First(EXTANDED_LABEL::BINARY),
            127 => Second(EXTANDED_LABEL_Details::new(RESERVED_FUTURE_EXPANSION, U6::new(t.get()), RESERVED_FUTURE_EXPANSION)),
            _ => panic!("Can't decode EXTANDED LABEL!")
        }
    }
}
