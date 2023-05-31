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

///     AD = 0
///     When the AD bit is set to 0, this may indicate that although some records have passed
///     signature verification, not all records (RRSet) in the response may be authentic.
///
///     AD = 1
///     When the AD bit is 1, this indicates that the DNS server claims that the records in the
///     response are authentic based on signature verification. However, the presence of the AD bit
///     does not guarantee that all records in the response are authentic, as it depends on the
///     correct configuration of the DNS server and the chain of trust to the signing keys.

use super::{U1};

pub enum AD {
    NO = 0,
    YES = 1
}

impl AD {
    pub fn code_edns(&self) -> U1 {
        return match self {
            AD::NO => U1::new(AD::NO as u8),
            AD::YES => U1::new(AD::YES as u8),
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            AD::NO => "Not-Authenticated Data",
            AD::YES => "Authenticated Data",
        }
    }

    pub fn encode_edns(t: &str) -> AD {
        return match t {
            "NO" | "NAD" => AD::NO,
            "YES" | "AD" => AD::YES,
            _ => panic!("Can't encode AD!")
        }
    }

    pub fn decode_edns(t: &U1) -> AD {
        return match t.get() {
            0 => AD::NO,
            1 => AD::YES,
            _ => panic!("Can't decode AD!")
        }
    }
}
