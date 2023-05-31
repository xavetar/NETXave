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

///     CD = 0
///     The server performs the verification of the record.
///     The client does not need to perform signature verification itself.
///     These signatures are also returned to the client so that he can verify them again.
///
///     CD = 1
///     Disable signature verification on the server side.
///     The client must perform the authentication itself.

use super::{U1};

pub enum CD {
    NO = 0,
    YES = 1
}

impl CD {
    pub fn code(&self) -> U1 {
        return match self {
            CD::NO => U1::new(CD::NO as u8),
            CD::YES => U1::new(CD::YES as u8),
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            CD::NO => "Enabled Signature Validation (on Server Side)",
            CD::YES => "Disabled Signature Validation (on Server Side)",
        }
    }

    pub fn encode(t: &str) -> CD {
        return match t {
            "NO" | "ESV" | "NCD" => CD::NO,
            "YES" | "DSV" | "CD" => CD::YES,
            _ => panic!("Can't encode CD!")
        }
    }

    pub fn decode(t: &U1) -> CD {
        return match t.get() {
            0 => CD::NO,
            1 => CD::YES,
            _ => panic!("Can't decode CD!")
        }
    }
}
