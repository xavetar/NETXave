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

///     ├╌╌╌╌╌┬╌╌╌╌╌╌╌╌┬╌╌╌╌┬╌╌╌╌┬╌╌╌╌┬╌╌╌╌┬╌╌╌┬╌╌╌╌╌╌╌╌┤
///     │  ┄┄ │ ┄┄┄┄┄┄ │ ┄┄ │ ┄┄ │ ┄┄ │ ┄┄ │ ┄ │ RCODE  │ ┌─> LSB: RCODE (Header) <─> 4 bits
///     ├╌╌╌╌╌┴╌╌╌╌╌╌╌╌┴╌╌╌╌┴╌╌╌╌┴╌╌╌╌┴╌╌╌╌┴╌╌╌┴┄┄┄┼┄┄┄┄│ │
///     ┊                                          │    ┊ │
///     │                 EXTENDED-RCODE───────────┘    │ └─> MSB: EXTENDED-RCODE (OPT) <─> 8 bits
///     │                                               │
///     ├───────────────────────────────────────────────┤
///     │                    VERSION                    │ <─> 8 bits
///     ├───────────────────────────────────────────────┤
///     │  DO ┐                                         │ ┌─> MSB: DO     ─> 1 bit
///     │  │  └────────────────────Z                    │ ├─> LSB: Z      ─> 15 bits
///     ├──┴──┬────────────────────┴────────────────────┤ │
///     │  DO │                    Z                    │ └─> FLAGS  ─> 16 bits
///     ├╌╌╌╌╌┴╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤

mod RCODE_EXTENDED_m;
mod VERSION_m;
pub mod Flags;

pub use RCODE_EXTENDED_m::{RCODE_EXTENDED};
pub use VERSION_m::{VERSION};
