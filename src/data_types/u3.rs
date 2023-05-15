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

pub struct U3(u8);

impl U3 {
    fn new(value: u8) -> Self {
        U3(value & 0b00000111)
    }

    fn set(&mut self, value: u8) {
        if value > 0 && value <= 7 {
            self.0 |= 0b00000111;
        } else if value == 0 {
            self.0 &= 0b11111000;
        } else {
            panic!("The value cannot be greater than 3 bits.")
        }
    }

    fn get(&self) -> u8 {
        return self.0
    }
}

#[cfg(test)]
mod tests {
    use super::U3;

    #[test]
    fn u3_test() {
        let mut u3 = U3::new(1);

        println!("u1 value: {}", u3.get());

        u3.set(7);
        println!("u1 value: {}", u3.get());

        u3.set(0);
        println!("u1 value: {}", u3.get());
    }
}
