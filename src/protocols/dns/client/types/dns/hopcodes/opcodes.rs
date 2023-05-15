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

pub enum OPCODES {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Notify = 4,
    Update = 5,
    DSO = 6
}

impl OPCODES {
    pub fn opcode(&self) -> u16 {
        return match self {
            OPCODES::Query => OPCODES::Query as u16,
            OPCODES::IQuery => OPCODES::IQuery as u16,
            OPCODES::Status => OPCODES::Status as u16,
            OPCODES::Notify => OPCODES::Notify as u16,
            OPCODES::Update => OPCODES::Update as u16,
            OPCODES::DSO => OPCODES::DSO as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            OPCODES::Query => "Query",
            OPCODES::IQuery => "IQuery",
            OPCODES::Status => "Status",
            OPCODES::Notify => "Notify",
            OPCODES::Update => "Update",
            OPCODES::DSO => "DSO"
        }
    }
}