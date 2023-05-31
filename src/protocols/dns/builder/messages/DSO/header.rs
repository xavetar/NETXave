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

use super::{U1, U3, U4, U7};
use super::{Header};

#[derive(Debug)]
pub struct Flags {
    QR: U1,
    OPCODE: U4,
    Z: Z,
    RCODE: U4
}

#[derive(Debug)]
pub struct Z {
    None: U7
}

impl Default for Header<Flags> {
    fn default() -> Header<Flags> {
        Header::new(
            0 as u16,
            Flags::default(),
            0 as u16,
            0 as u16,
            0 as u16,
            0 as u16
        )
    }
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            QR: U1::new(0),
            OPCODE: U4::new(0),
            Z: Z::default(),
            RCODE: U4::new(0)
        }
    }
}

impl Default for Z {
    fn default() -> Z {
        Z {
            None: U7::new(0)
        }
    }
}

impl Flags {
    pub fn new(qr: U1, opcode: U4, z: Z, rcode: U4) -> Flags {
        Flags {
            QR: qr,
            OPCODE: opcode,
            Z: z,
            RCODE: rcode
        }
    }

    pub fn get_qr(&self) -> &U1 {
        return &self.QR;
    }

    pub fn get_opcode(&self) -> &U4 {
        return &self.OPCODE;
    }

    pub fn get_z(&self) -> &Z {
        return &self.Z;
    }

    pub fn get_rcode(&self) -> &U4 {
        return &self.RCODE;
    }
}

impl Z {
    pub fn new(none: U7) -> Z {
        Z {
            None: none
        }
    }

    pub fn get_none(&self) -> &U7 {
        return &self.None;
    }
}
