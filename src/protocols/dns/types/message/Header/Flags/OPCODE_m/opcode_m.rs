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

use super::{U4};
use super::{UNASSIGNED};
use super::{OPCODE_Details};
use super::{TwoResult, TwoResult::First, TwoResult::Second};

pub enum OPCODE {
    QUERY = 0,
    IQUERY = 1,
    STATUS = 2,
    NOTIFY = 4,
    UPDATE = 5,
    DSO = 6
}

impl OPCODE {
    pub fn code(&self) -> U4 {
        return match self {
            OPCODE::QUERY => U4::new(OPCODE::QUERY as u8),
            OPCODE::IQUERY => U4::new(OPCODE::IQUERY as u8),
            OPCODE::STATUS => U4::new(OPCODE::STATUS as u8),
            OPCODE::NOTIFY => U4::new(OPCODE::NOTIFY as u8),
            OPCODE::UPDATE => U4::new(OPCODE::UPDATE as u8),
            OPCODE::DSO => U4::new(OPCODE::DSO as u8)
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            OPCODE::QUERY => "Query",
            OPCODE::IQUERY => "IQuery",
            OPCODE::STATUS => "Status",
            OPCODE::NOTIFY => "Notify",
            OPCODE::UPDATE => "Update",
            OPCODE::DSO => "DNS Stateful Operations"
        }
    }

    pub fn encode(t: &str) -> OPCODE {
        return match t {
            "QUERY" => OPCODE::QUERY,
            "IQUERY" => OPCODE::IQUERY,
            "STATUS" => OPCODE::STATUS,
            "NOTIFY" => OPCODE::NOTIFY,
            "UPDATE" => OPCODE::UPDATE,
            "DSO" => OPCODE::DSO,
            _ => panic!("Can't encode OPCODE!")
        }
    }

    pub fn decode(t: &U4) -> TwoResult<OPCODE, OPCODE_Details> {
        return match t.get() {
            0 => First(OPCODE::QUERY),
            1 => First(OPCODE::IQUERY),
            2 => First(OPCODE::STATUS),
            3 => Second(OPCODE_Details::new(UNASSIGNED, U4::new(t.get()))),
            4 => First(OPCODE::NOTIFY),
            5 => First(OPCODE::UPDATE),
            6 => First(OPCODE::DSO),
            7..=15 => Second(OPCODE_Details::new(UNASSIGNED, U4::new(t.get()))),
            _ => panic!("Can't decode OPCODE!")
        }
    }
}
