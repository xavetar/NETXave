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

use super::{Newable};

#[derive(Debug)]
pub struct U12(u16);

impl U12 {
    pub fn new(value: u16) -> U12 {
        return U12(value & 0b_1111_1111_1111);
    }

    fn set(&mut self, value: u16) {
        if value > 0 && value <= 4095 {
            self.0 |= 0b0000_1111_1111_1111;
        } else if value == 0 {
            self.0 &= 0b0000_0000_0000_0000;
        } else {
            panic!("The value cannot be greater than 12 bits.")
        }
    }

    pub fn get(&self) -> u16 {
        return self.0
    }
}

impl<I> Newable<I, U12> for U12
    where I: std::ops::BitAnd<u16, Output = u16> + PartialOrd<u16> + Into<u16> {
    fn new(value: I) -> U12 {
        if value >= 0 && value <= 4095 {
            return U12(value & 0b0000_1111_1111_1111);
        } else {
            panic!("The value cannot be greater than 12 bits.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::U12;

    #[test]
    fn u12_test() {
        let mut u12 = U12::new(1);

        println!("u12 value: {}", u12.get());

        u12.set(4095);
        println!("u12 value: {}", u12.get());

        u12.set(0);
        println!("u12 value: {}", u12.get());
    }
}
