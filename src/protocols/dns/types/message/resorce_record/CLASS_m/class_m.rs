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

pub enum CLASS {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    None = 254,
    ANY = 255
}

impl CLASS {
    pub fn name(&self) -> &'static str {
        return match self {
            CLASS::IN => "IN",
            CLASS::CS => "CS",
            CLASS::CH => "CH",
            CLASS::HS => "HS",
            CLASS::None => "None",
            CLASS::ANY => "*"
        }
    }

    pub fn code(&self) -> u16 {
        return match self {
            CLASS::IN => CLASS::IN as u16,
            CLASS::CS => CLASS::CS as u16,
            CLASS::CH => CLASS::CH as u16,
            CLASS::HS => CLASS::HS as u16,
            CLASS::None => CLASS::None as u16,
            CLASS::ANY => CLASS::ANY as u16
        }
    }

    pub fn hex(&self) -> String {
        return match self {
            CLASS::IN => format!("{:02x}", CLASS::IN.code()),
            CLASS::CS => format!("{:02x}", CLASS::CS.code()),
            CLASS::CH => format!("{:02x}", CLASS::CH.code()),
            CLASS::HS => format!("{:02x}", CLASS::HS.code()),
            CLASS::None => format!("{:02x}", CLASS::None.code()),
            CLASS::ANY => format!("{:02x}", CLASS::ANY.code())
        }
    }
}
