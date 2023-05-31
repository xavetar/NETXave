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

use super::{U1};

pub enum TC {
    NO = 0,
    YES = 1
}

impl TC {
    pub fn code(&self) -> U1 {
        return match self {
            TC::NO => U1::new(TC::NO as u8),
            TC::YES => U1::new(TC::YES as u8),
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            TC::NO => "Not-Truncated",
            TC::YES => "Message is Truncated",
        }
    }

    pub fn encode(t: &str) -> TC {
        return match t {
            "NO" | "NT" => TC::NO,
            "YES" | "MT" => TC::YES,
            _ => panic!("Can't encode TC!")
        }
    }

    pub fn decode(t: &U1) -> TC {
        return match t.get() {
            0 => TC::NO,
            1 => TC::YES,
            _ => panic!("Can't decode TC!")
        }
    }
}
