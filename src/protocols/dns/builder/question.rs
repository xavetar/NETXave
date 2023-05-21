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

#[derive(Debug)]
pub struct QuestionSection {
    QNAME: Vec<u8>,
    QTYPE: u16,
    QCLASS: u16
}

impl Default for QuestionSection {
    fn default() -> QuestionSection {
        QuestionSection {
            QNAME: Vec::<u8>::from(""),
            QTYPE: 0 as u16,
            QCLASS: 0 as u16
        }
    }
}

impl QuestionSection {
    pub fn new(qname: Vec<u8>, qtype: u16, qclass: u16) -> QuestionSection {
        QuestionSection {
            QNAME: qname,
            QTYPE: qtype,
            QCLASS: qclass
        }
    }

    pub fn get_qname(&self) -> &Vec<u8> {
        return &self.QNAME;
    }

    pub fn get_qtype(&self) -> &u16 {
        return &self.QTYPE;
    }

    pub fn get_qclass(&self) -> &u16 {
        return &self.QCLASS;
    }
}
