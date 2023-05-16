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

use super::{TC};
use super::{TCInfo};


pub trait TCConversion {
    fn encode(name: &str) -> Result<TCInfo, String>;
    fn decode(dec: &u8) -> Result<TCInfo, String>;
}

impl TCConversion for TC {
    fn encode(name: &str) -> Result<TCInfo, String> {
        return match name {
            "Non-Truncated" => Ok(
                TCInfo::new(TC::NonTruncated.name(),
                            TC::NonTruncated.code())
            ),
            "Truncated" => Ok(
                TCInfo::new(TC::Truncated.name(),
                            TC::Truncated.code())
            ),
            _ => Err(String::from("Can't encode TC!"))
        }
    }

    fn decode(decimal: &u8) -> Result<TCInfo, String> {
        return match *decimal {
            0 => Ok(
                TCInfo::new(TC::NonTruncated.name(),
                            TC::NonTruncated.code())
            ),
            1 => Ok(
                TCInfo::new(TC::Truncated.name(),
                            TC::Truncated.code())
            ),
            _ => Err(String::from("Can't decode TC!"))
        }
    }
}
