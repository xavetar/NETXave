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

pub const REVERSED: &'static str = "Reversed";
pub const UNASSIGNED: &'static str = "Unassigned";
pub const REVERSED_PRIVATE_USE: &'static str = "Reversed for Private Use";

pub struct QCLASSInfo {
    name: &'static str,
    code: u16
}

impl QCLASSInfo {
    pub fn name(&self) -> &'static str {
        return &self.name;
    }

    pub fn code(&self) -> &u16 {
        return &self.code;
    }

    pub fn hex(&self) -> String {
        return format!("{:02x}", &self.code);
    }
}

pub enum QCLASS {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    None = 254,
    ANY = 255
}

impl QCLASS {
    pub fn name(&self) -> &'static str {
        return match self {
            QCLASS::IN => "IN",
            QCLASS::CS => "CS",
            QCLASS::CH => "CH",
            QCLASS::HS => "HS",
            QCLASS::None => "None",
            QCLASS::ANY => "*"
        }
    }

    pub fn code(&self) -> u16 {
        return match self {
            QCLASS::IN => QCLASS::IN as u16,
            QCLASS::CS => QCLASS::CS as u16,
            QCLASS::CH => QCLASS::CH as u16,
            QCLASS::HS => QCLASS::HS as u16,
            QCLASS::None => QCLASS::None as u16,
            QCLASS::ANY => QCLASS::ANY as u16
        }
    }

    pub fn hex(&self) -> String {
        return match self {
            QCLASS::IN => format!("{:02x}", QCLASS::IN.code()),
            QCLASS::CS => format!("{:02x}", QCLASS::CS.code()),
            QCLASS::CH => format!("{:02x}", QCLASS::CH.code()),
            QCLASS::HS => format!("{:02x}", QCLASS::HS.code()),
            QCLASS::None => format!("{:02x}", QCLASS::None.code()),
            QCLASS::ANY => format!("{:02x}", QCLASS::ANY.code())
        }
    }
}

pub trait QCLASSConversion {
    fn encode(qclass: &str) -> Result<QCLASSInfo, String>;
    fn decode(dec: &u16) -> Result<QCLASSInfo, String>;
}

impl QCLASSConversion for QCLASS {
    fn encode(qclass: &str) -> Result<QCLASSInfo, String> {
        return match qclass {
            "IN" => Ok(
                QCLASSInfo {
                    name: QCLASS::IN.name(),
                    code: QCLASS::IN.code()
                }
            ),
            "CS" => Ok(
                QCLASSInfo {
                    name: QCLASS::CS.name(),
                    code: QCLASS::CS.code()
                }
            ),
            "CH" => Ok(
                QCLASSInfo {
                    name: QCLASS::CH.name(),
                    code: QCLASS::CH.code()
                }
            ),
            "HS" => Ok(
                QCLASSInfo {
                    name: QCLASS::HS.name(),
                    code: QCLASS::HS.code()
                }
            ),
            "None" => Ok(
                QCLASSInfo {
                    name: QCLASS::None.name(),
                    code: QCLASS::None.code()
                }
            ),
            "*" => Ok(
                QCLASSInfo {
                    name: QCLASS::ANY.name(),
                    code: QCLASS::ANY.code()
                }
            ),
            _ => Err(String::from("Can't encode QCLASS!"))
        }
    }

    fn decode(decimal: &u16) -> Result<QCLASSInfo, String> {
        return match *decimal {
            0 => Ok(
                QCLASSInfo{
                    name: REVERSED,
                    code: *decimal
                }
            ),
            1 => Ok(
                QCLASSInfo{
                    name: QCLASS::IN.name(),
                    code: QCLASS::IN.code()
                }
            ),
            2 => Ok(
                QCLASSInfo{
                    name: QCLASS::CS.name(),
                    code: QCLASS::CS.code()
                }
            ),
            3 => Ok(
                QCLASSInfo{
                    name: QCLASS::CS.name(),
                    code: QCLASS::CS.code()
                }
            ),
            4 => Ok(
                QCLASSInfo{
                    name: QCLASS::HS.name(),
                    code: QCLASS::HS.code()
                }
            ),
            5..=253 => Ok(
                QCLASSInfo{
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            254 => Ok(
                QCLASSInfo{
                    name: QCLASS::None.name(),
                    code: QCLASS::None.code()
                }
            ),
            255 => Ok(
                QCLASSInfo{
                    name: QCLASS::ANY.name(),
                    code: QCLASS::ANY.code()
                }
            ),
            256..=65279 => Ok(
                QCLASSInfo{
                    name: UNASSIGNED,
                    code: *decimal
                }
            ),
            65280..=65534 => Ok(
                QCLASSInfo{
                    name: REVERSED_PRIVATE_USE,
                    code: *decimal
                }
            ),
            65535 => Ok(
                QCLASSInfo{
                    name: REVERSED,
                    code: *decimal
                }
            ),
            _ => Err(String::from("Can't decode QCLASS!"))
        }
    }
}
