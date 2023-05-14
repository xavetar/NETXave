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

use super::{UNUSED, UNASSIGNED, HTTPStatusInfo, HTTPCodeInfo};

pub enum Informational {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
}

impl Informational {
    pub fn value(&self) -> u16 {
        return match self {
            Informational::Continue => Informational::Continue as u16,
            Informational::SwitchingProtocols => Informational::SwitchingProtocols as u16,
            Informational::Processing => Informational::Processing as u16,
            Informational::EarlyHints => Informational::EarlyHints as u16
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            Informational::Continue => "Continue",
            Informational::SwitchingProtocols => "Switching Protocols",
            Informational::Processing => "Processing",
            Informational::EarlyHints => "Early Hints"
        }
    }
}

impl HTTPStatusInfo for Informational {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            100 => Ok(
                HTTPCodeInfo::new(Informational::Continue.value(),
                                  Informational::Continue.description())
            ),
            101 => Ok(
                HTTPCodeInfo::new(Informational::SwitchingProtocols.value(),
                                  Informational::SwitchingProtocols.description())
            ),
            102 => Ok(
                HTTPCodeInfo::new(Informational::Processing.value(),
                                  Informational::Processing.description())
            ),
            103 => Ok(
                HTTPCodeInfo::new(Informational::EarlyHints.value(),
                                  Informational::EarlyHints.description())
            ),
            104..=199 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Informational code!"))
        }
    }
}
