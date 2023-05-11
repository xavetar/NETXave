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

#[cfg(any(feature = "dns"))]
pub mod dns;

#[cfg(any(feature = "ssh"))]
pub mod ssh;

#[cfg(any(feature = "ftp"))]
pub mod ftp;

#[cfg(any(feature = "http"))]
pub mod http;

#[cfg(any(feature = "snmp"))]
pub mod snmp;

#[cfg(any(feature = "pop3"))]
pub mod pop3;

#[cfg(any(feature = "smtp"))]
pub mod smtp;

#[cfg(any(feature = "imap"))]
pub mod imap;

#[cfg(any(feature = "gemini"))]
pub mod gemini;

#[cfg(any(feature = "gopher"))]
pub mod gopher;

#[cfg(any(feature = "telnet"))]
pub mod telnet;
