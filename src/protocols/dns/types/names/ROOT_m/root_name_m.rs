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

use super::{ROOT_NAME_Details};
use super::{TwoResult, TwoResult::First, TwoResult::Second};

pub enum ROOT_NAME {
    BASE = 0
}

impl ROOT_NAME {
    pub fn code(&self) -> u8 {
        return match self {
            ROOT_NAME::BASE => ROOT_NAME::BASE as u8
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            ROOT_NAME::BASE => "Base Root Domain"
        }
    }

    pub fn encode(t: &str) -> ROOT_NAME {
        return match t {
            "BASE" | "ROOT" => ROOT_NAME::BASE,
            _ => panic!("Can't encode ROOT NAME!")
        }
    }

    pub fn decode(t: &u8) -> TwoResult<ROOT_NAME, ROOT_NAME_Details> {
        return match *t {
            0 => First(ROOT_NAME::BASE),
            _ => panic!("Can't decode ROOT NAME!")
        }
    }
}
