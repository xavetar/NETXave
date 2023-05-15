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

use super::{OPCODES};
use super::{OPCODEInfo};
use super::{UNASSIGNED};


pub trait OPCODEConversion {
    fn encode(name: &str) -> Result<OPCODEInfo, String>;
    fn decode(dec: &u16) -> Result<OPCODEInfo, String>;
}

impl OPCODEConversion for OPCODES {
    fn encode(name: &str) -> Result<OPCODEInfo, String> {
        return match name {
            "Query" => Ok(
                OPCODEInfo::new(OPCODES::Query.name(),
                                OPCODES::Query.opcode())
            ),
            "IQuery" => Ok(
                OPCODEInfo::new(OPCODES::IQuery.name(),
                                OPCODES::IQuery.opcode())
            ),
            "Status" => Ok(
                OPCODEInfo::new(OPCODES::Status.name(),
                                OPCODES::Status.opcode())
            ),
            "Notify" => Ok(
                OPCODEInfo::new(OPCODES::Notify.name(),
                                OPCODES::Notify.opcode())
            ),
            "Update" => Ok(
                OPCODEInfo::new(OPCODES::Update.name(),
                                OPCODES::Update.opcode())
            ),
            "DSO" => Ok(
                OPCODEInfo::new(OPCODES::DSO.name(),
                                OPCODES::DSO.opcode())
            ),
            _ => Err(String::from("Can't encode OPCODE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<OPCODEInfo, String> {
        return match *decimal {
            0 => Ok(
                OPCODEInfo::new(OPCODES::Query.name(),
                                OPCODES::Query.opcode())
            ),
            1 => Ok(
                OPCODEInfo::new(OPCODES::IQuery.name(),
                                OPCODES::IQuery.opcode())
            ),
            2 => Ok(
                OPCODEInfo::new(OPCODES::Status.name(),
                                OPCODES::Status.opcode())
            ),
            3 => Ok(
                OPCODEInfo::new(UNASSIGNED,
                                *decimal)
            ),
            4 => Ok(
                OPCODEInfo::new(OPCODES::Notify.name(),
                                OPCODES::Notify.opcode())
            ),
            5 => Ok(
                OPCODEInfo::new(OPCODES::Update.name(),
                                OPCODES::Update.opcode())
            ),
            6 => Ok(
                OPCODEInfo::new(OPCODES::DSO.name(),
                                OPCODES::DSO.opcode())
            ),
            7..=15 => Ok(
                OPCODEInfo::new(UNASSIGNED,
                                *decimal)
            ),
            _ => Err(String::from("Can't decode OPCODE!"))
        }
    }
}
