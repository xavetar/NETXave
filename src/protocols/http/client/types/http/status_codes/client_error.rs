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

pub enum ClientError {
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
}

impl ClientError {
    pub fn value(&self) -> u16 {
        return match self {
            ClientError::BadRequest => ClientError::BadRequest as u16,
            ClientError::Unauthorized => ClientError::Unauthorized as u16,
            ClientError::PaymentRequired => ClientError::PaymentRequired as u16,
            ClientError::Forbidden => ClientError::Forbidden as u16,
            ClientError::NotFound => ClientError::NotFound as u16,
            ClientError::MethodNotAllowed => ClientError::MethodNotAllowed as u16,
            ClientError::NotAcceptable => ClientError::NotAcceptable as u16,
            ClientError::ProxyAuthenticationRequired => ClientError::ProxyAuthenticationRequired as u16,
            ClientError::RequestTimeout => ClientError::RequestTimeout as u16,
            ClientError::Conflict => ClientError::Conflict as u16,
            ClientError::Gone => ClientError::Gone as u16,
            ClientError::LengthRequired => ClientError::LengthRequired as u16,
            ClientError::PreconditionFailed => ClientError::PreconditionFailed as u16,
            ClientError::ContentTooLarge => ClientError::ContentTooLarge as u16,
            ClientError::URITooLong => ClientError::URITooLong as u16,
            ClientError::UnsupportedMediaType => ClientError::UnsupportedMediaType as u16,
            ClientError::RangeNotSatisfiable => ClientError::RangeNotSatisfiable as u16,
            ClientError::ExpectationFailed => ClientError::ExpectationFailed as u16,
            ClientError::MisdirectedRequest => ClientError::MisdirectedRequest as u16,
            ClientError::UnprocessableContent => ClientError::UnprocessableContent as u16,
            ClientError::Locked => ClientError::Locked as u16,
            ClientError::FailedDependency => ClientError::FailedDependency as u16,
            ClientError::TooEarly => ClientError::TooEarly as u16,
            ClientError::UpgradeRequired => ClientError::UpgradeRequired as u16,
            ClientError::PreconditionRequired => ClientError::PreconditionFailed as u16,
            ClientError::TooManyRequests => ClientError::TooManyRequests as u16,
            ClientError::RequestHeaderFieldsTooLarge => ClientError::RequestHeaderFieldsTooLarge as u16,
            ClientError::UnavailableForLegalReasons => ClientError::UnavailableForLegalReasons as u16,
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            ClientError::BadRequest => "Bad Request",
            ClientError::Unauthorized => "Unauthorized",
            ClientError::PaymentRequired => "Payment Required",
            ClientError::Forbidden => "Forbidden",
            ClientError::NotFound => "Not Found",
            ClientError::MethodNotAllowed => "Method Not Allowed",
            ClientError::NotAcceptable => "Not Acceptable",
            ClientError::ProxyAuthenticationRequired => "Proxy Authentication Required",
            ClientError::RequestTimeout => "Request Timeout",
            ClientError::Conflict => "Conflict",
            ClientError::Gone => "Gone",
            ClientError::LengthRequired => "Length Required",
            ClientError::PreconditionFailed => "Precondition Failed",
            ClientError::ContentTooLarge => "Content Too Large",
            ClientError::URITooLong => "URI Too Long",
            ClientError::UnsupportedMediaType => "Unsupported Media Type",
            ClientError::RangeNotSatisfiable => "Range Not Satisfiable",
            ClientError::ExpectationFailed => "Expectation Failed",
            ClientError::MisdirectedRequest => "Misdirected Request",
            ClientError::UnprocessableContent => "Unprocessable Content",
            ClientError::Locked => "Locked",
            ClientError::FailedDependency => "Failed Dependency",
            ClientError::TooEarly => "Too Early",
            ClientError::UpgradeRequired => "Upgrade Required",
            ClientError::PreconditionRequired => "Precondition Required",
            ClientError::TooManyRequests => "Too Many Requests",
            ClientError::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            ClientError::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
        }
    }
}
