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

use super::{UNUSED, UNASSIGNED, HTTPStatusInfo, HTTPCodeInfo};

pub enum Success {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
}

impl Success {
    pub fn value(&self) -> u16 {
        return match self {
            Success::OK => Success::OK as u16,
            Success::Created => Success::Created as u16,
            Success::Accepted => Success::Accepted as u16,
            Success::NonAuthoritativeInformation => Success::NonAuthoritativeInformation as u16,
            Success::NoContent => Success::NoContent as u16,
            Success::ResetContent => Success::ResetContent as u16,
            Success::PartialContent => Success::PartialContent as u16,
            Success::MultiStatus => Success::MultiStatus as u16,
            Success::AlreadyReported => Success::AlreadyReported as u16,
            Success::IMUsed => Success::IMUsed as u16
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            Success::OK => "OK",
            Success::Created => "Created",
            Success::Accepted => "Accepted",
            Success::NonAuthoritativeInformation => "Non-Authoritative Information",
            Success::NoContent => "No Content",
            Success::ResetContent => "Reset Content",
            Success::PartialContent => "Partial Content",
            Success::MultiStatus => "Multi-Status",
            Success::AlreadyReported => "Already Reported",
            Success::IMUsed => "IM Used",
        }
    }
}

impl HTTPStatusInfo for Success {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            200 => Ok(
                HTTPCodeInfo::new(Success::OK.value(),
                                  Success::OK.description())
            ),
            201 => Ok(
                HTTPCodeInfo::new(Success::Created.value(),
                                  Success::Created.description())
            ),
            202 => Ok(
                HTTPCodeInfo::new(Success::Accepted.value(),
                                  Success::Accepted.description())
            ),
            203 => Ok(
                HTTPCodeInfo::new(Success::NonAuthoritativeInformation.value(),
                                  Success::NonAuthoritativeInformation.description())
            ),
            204 => Ok(
                HTTPCodeInfo::new(Success::NoContent.value(),
                                  Success::NoContent.description())
            ),
            205 => Ok(
                HTTPCodeInfo::new(Success::ResetContent.value(),
                                  Success::ResetContent.description())
            ),
            206 => Ok(
                HTTPCodeInfo::new(Success::PartialContent.value(),
                                  Success::PartialContent.description())
            ),
            207 => Ok(
                HTTPCodeInfo::new(Success::MultiStatus.value(),
                                  Success::MultiStatus.description())
            ),
            208 => Ok(
                HTTPCodeInfo::new(Success::AlreadyReported.value(),
                                  Success::AlreadyReported.description())
            ),
            209..=225 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            226 => Ok(
                HTTPCodeInfo::new(Success::IMUsed.value(),
                                  Success::IMUsed.description())
            ),
            227..=299 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Success code!"))
        }
    }
}
