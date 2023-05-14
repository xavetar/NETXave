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

pub enum ServerError {
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl ServerError {
    pub fn value(&self) -> u16 {
        return match self {
            ServerError::InternalServerError => ServerError::InternalServerError as u16,
            ServerError::NotImplemented => ServerError::NotImplemented as u16,
            ServerError::BadGateway => ServerError::BadGateway as u16,
            ServerError::ServiceUnavailable => ServerError::ServiceUnavailable as u16,
            ServerError::GatewayTimeout => ServerError::GatewayTimeout as u16,
            ServerError::HTTPVersionNotSupported => ServerError::HTTPVersionNotSupported as u16,
            ServerError::VariantAlsoNegotiates => ServerError::VariantAlsoNegotiates as u16,
            ServerError::InsufficientStorage => ServerError::InsufficientStorage as u16,
            ServerError::LoopDetected => ServerError::LoopDetected as u16,
            ServerError::NotExtended => ServerError::NotExtended as u16,
            ServerError::NetworkAuthenticationRequired => ServerError::NetworkAuthenticationRequired as u16
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            ServerError::InternalServerError => "Internal Server Error",
            ServerError::NotImplemented => "Not Implemented",
            ServerError::BadGateway => "Bad Gateway",
            ServerError::ServiceUnavailable => "Service Unavailable",
            ServerError::GatewayTimeout => "Gateway Timeout",
            ServerError::HTTPVersionNotSupported => "HTTP Version Not Supported",
            ServerError::VariantAlsoNegotiates => "Variant Also Negotiates",
            ServerError::InsufficientStorage => "Insufficient Storage",
            ServerError::LoopDetected => "Loop Detected",
            ServerError::NotExtended => "Not Extended",
            ServerError::NetworkAuthenticationRequired => "Network Authentication Required"
        }
    }
}
