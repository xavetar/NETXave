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

use std::fmt;

pub struct HTTPCodeInfo {
    code: u16,
    description: &'static str
}

impl HTTPCodeInfo {
    pub fn get_all(&self) -> (u16, &'static str) {
        return (self.code, self.description)
    }

    pub fn get_code(&self) -> u16 {
        return self.code
    }

    pub fn get_description(&self) -> &'static str {
        return self.description
    }
}

pub enum Informational {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
}

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
    IMUser = 226,
}

pub enum Redirection {
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
}

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

pub trait HTTPStatusInfo {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String>;
}

impl HTTPStatusInfo for Informational {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            100 => Ok(
                HTTPCodeInfo {
                    code: Informational::Continue as u16,
                    description: "Continue"
                }
            ),
            101 => Ok(
                HTTPCodeInfo {
                    code: Informational::SwitchingProtocols as u16,
                    description: "Switching Protocols"
                },
            ),
            102 => Ok(
                HTTPCodeInfo {
                    code: Informational::Processing as u16,
                    description: "Processing"
                },
            ),
            103 => Ok(
                HTTPCodeInfo {
                    code: Informational::EarlyHints as u16,
                    description: "Early Hints"
                },
            ),
            104..=199 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                },
            ),
            _ => Err(String::from("Informational. Unknown code!"))
        }
    }
}

impl HTTPStatusInfo for Success {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            200 => Ok(
                HTTPCodeInfo {
                    code: Success::OK as u16,
                    description: "OK"
                }
            ),
            201 => Ok(
                HTTPCodeInfo {
                    code: Success::Created as u16,
                    description: "Created"
                }
            ),
            202 => Ok(
                HTTPCodeInfo {
                    code: Success::Accepted as u16,
                    description: "Accepted"
                }
            ),
            203 => Ok(
                HTTPCodeInfo {
                    code: Success::NonAuthoritativeInformation as u16,
                    description: "Non Authoritative Information"
                }
            ),
            204 => Ok(
                HTTPCodeInfo {
                    code: Success::NoContent as u16,
                    description: "No Content"
                }
            ),
            205 => Ok(
                HTTPCodeInfo {
                    code: Success::ResetContent as u16,
                    description: "Reset Content"
                }
            ),
            206 => Ok(
                HTTPCodeInfo {
                    code: Success::PartialContent as u16,
                    description: "Partial Content"
                }
            ),
            207 => Ok(
                HTTPCodeInfo {
                    code: Success::MultiStatus as u16,
                    description: "Multi-Status"
                }
            ),
            208 => Ok(
                HTTPCodeInfo {
                    code: Success::AlreadyReported as u16,
                    description: "Already Reported"
                }
            ),
            209..=225 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            226 => Ok(
                HTTPCodeInfo {
                    code: Success::IMUser as u16,
                    description: "IM Used/Instance Manipulation Used"
                }
            ),
            227..=299 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            _ => Err(String::from("Success. Unknown code!"))
        }
    }
}

impl HTTPStatusInfo for Redirection {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            300 => Ok(
                HTTPCodeInfo {
                    code: Redirection::MultipleChoices as u16,
                    description: "Multiple Choices"
                }
            ),
            301 => Ok(
                HTTPCodeInfo {
                    code: Redirection::MovedPermanently as u16,
                    description: "Moved Permanently"
                }
            ),
            302 => Ok(
                HTTPCodeInfo {
                    code: Redirection::Found as u16,
                    description: "Found"
                }
            ),
            303 => Ok(
                HTTPCodeInfo {
                    code: Redirection::SeeOther as u16,
                    description: "See Other"
                }
            ),
            304 => Ok(
                HTTPCodeInfo {
                    code: Redirection::NotModified as u16,
                    description: "Not Modified"
                }
            ),
            305 => Ok(
                HTTPCodeInfo {
                    code: Redirection::UseProxy as u16,
                    description: "Use Proxy"
                }
            ),
            306 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unused"
                }
            ),
            307 => Ok(
                HTTPCodeInfo {
                    code: Redirection::TemporaryRedirect as u16,
                    description: "Temporary Redirect"
                }
            ),
            308 => Ok(
                HTTPCodeInfo {
                    code: Redirection::PermanentRedirect as u16,
                    description: "Permanent Redirect"
                }
            ),
            309..=399 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            _ => Err(String::from("Redirection. Unknown code!"))
        }
    }
}

impl HTTPStatusInfo for ClientError {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            400 => Ok(
                HTTPCodeInfo {
                    code: ClientError::BadRequest as u16,
                    description: "Bad Request"
                }
            ),
            401 => Ok(
                HTTPCodeInfo {
                    code: ClientError::Unauthorized as u16,
                    description: "Unauthorized"
                }
            ),
            402 => Ok(
                HTTPCodeInfo {
                    code: ClientError::PaymentRequired as u16,
                    description: "Payment Required"
                }
            ),
            403 => Ok(
                HTTPCodeInfo {
                    code: ClientError::Forbidden as u16,
                    description: "Forbidden"
                }
            ),
            404 => Ok(
                HTTPCodeInfo {
                    code: ClientError::NotFound as u16,
                    description: "Not Found"
                }
            ),
            405 => Ok(
                HTTPCodeInfo {
                    code: ClientError::MethodNotAllowed as u16,
                    description: "Method Not Allowed"
                }
            ),
            406 => Ok(
                HTTPCodeInfo {
                    code: ClientError::NotAcceptable as u16,
                    description: "Not Acceptable"
                }
            ),
            407 => Ok(
                HTTPCodeInfo {
                    code: ClientError::ProxyAuthenticationRequired as u16,
                    description: "Proxy Authentication Required"
                }
            ),
            408 => Ok(
                HTTPCodeInfo {
                    code: ClientError::RequestTimeout as u16,
                    description: "Request Timeout"
                }
            ),
            409 => Ok(
                HTTPCodeInfo {
                    code: ClientError::Conflict as u16,
                    description: "Conflict"
                }
            ),
            410 => Ok(
                HTTPCodeInfo {
                    code: ClientError::Gone as u16,
                    description: "Gone"
                }
            ),
            411 => Ok(
                HTTPCodeInfo {
                    code: ClientError::LengthRequired as u16,
                    description: "Length Required"
                }
            ),
            412 => Ok(
                HTTPCodeInfo {
                    code: ClientError::PreconditionFailed as u16,
                    description: "Precondition Failed"
                }
            ),
            413 => Ok(
                HTTPCodeInfo {
                    code: ClientError::ContentTooLarge as u16,
                    description: "Content Too Large"
                }
            ),
            414 => Ok(
                HTTPCodeInfo {
                    code: ClientError::URITooLong as u16,
                    description: "URI Too Long"
                }
            ),
            415 => Ok(
                HTTPCodeInfo {
                    code: ClientError::UnsupportedMediaType as u16,
                    description: "Unsupported Media Type"
                }
            ),
            416 => Ok(
                HTTPCodeInfo {
                    code: ClientError::RangeNotSatisfiable as u16,
                    description: "Range Not Satisfiable"
                }
            ),
            417 => Ok(
                HTTPCodeInfo {
                    code: ClientError::ExpectationFailed as u16,
                    description: "Expectation Failed"
                }
            ),
            418 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unused"
                }
            ),
            419..=420 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            421 => Ok(
                HTTPCodeInfo {
                    code: ClientError::MisdirectedRequest as u16,
                    description: "Misdirected Request"
                }
            ),
            422 => Ok(
                HTTPCodeInfo {
                    code: ClientError::UnprocessableContent as u16,
                    description: "Unprocessable Content"
                }
            ),
            423 => Ok(
                HTTPCodeInfo {
                    code: ClientError::Locked as u16,
                    description: "Locked"
                }
            ),
            424 => Ok(
                HTTPCodeInfo {
                    code: ClientError::FailedDependency as u16,
                    description: "Failed Dependency"
                }
            ),
            425 => Ok(
                HTTPCodeInfo {
                    code: ClientError::TooEarly as u16,
                    description: "Too Early"
                }
            ),
            426 => Ok(
                HTTPCodeInfo {
                    code: ClientError::UpgradeRequired as u16,
                    description: "Upgrade Required"
                }
            ),
            427 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            428 => Ok(
                HTTPCodeInfo {
                    code: ClientError::PreconditionRequired as u16,
                    description: "Precondition Required"
                }
            ),
            429 => Ok(
                HTTPCodeInfo {
                    code: ClientError::TooManyRequests as u16,
                    description: "Too Many Requests"
                }
            ),
            430 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            431 => Ok(
                HTTPCodeInfo {
                    code: ClientError::RequestHeaderFieldsTooLarge as u16,
                    description: "Request Header Fields Too Large"
                }
            ),
            432..=450 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            451 => Ok(
                HTTPCodeInfo {
                    code: ClientError::UnavailableForLegalReasons as u16,
                    description: "Unavailable For Legal Reasons"
                }
            ),
            452..=499 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            _ => Err(String::from("Client Error. Unknown code!"))
        }
    }
}

impl HTTPStatusInfo for ServerError {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            500 => Ok(
                HTTPCodeInfo {
                    code: ServerError::InternalServerError as u16,
                    description: "Internal Server Error"
                }
            ),
            501 => Ok(
                HTTPCodeInfo {
                    code: ServerError::NotImplemented as u16,
                    description: "Not Implemented"
                }
            ),
            502 => Ok(
                HTTPCodeInfo {
                    code: ServerError::BadGateway as u16,
                    description: "Bad Gateway"
                }
            ),
            503 => Ok(
                HTTPCodeInfo {
                    code: ServerError::ServiceUnavailable as u16,
                    description: "Service Unavailable"
                }
            ),
            504 => Ok(
                HTTPCodeInfo {
                    code: ServerError::GatewayTimeout as u16,
                    description: "Gateway Timeout"
                }
            ),
            505 => Ok(
                HTTPCodeInfo {
                    code: ServerError::HTTPVersionNotSupported as u16,
                    description: "HTTP Version Not Supported"
                }
            ),
            506 => Ok(
                HTTPCodeInfo {
                    code: ServerError::VariantAlsoNegotiates as u16,
                    description: "Variant Also Negotiates"
                }
            ),
            507 => Ok(
                HTTPCodeInfo {
                    code: ServerError::InsufficientStorage as u16,
                    description: "Insufficient Storage"
                }
            ),
            508 => Ok(
                HTTPCodeInfo {
                    code: ServerError::LoopDetected as u16,
                    description: "Loop Detected"
                }
            ),
            509 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            510 => Ok(
                HTTPCodeInfo {
                    code: ServerError::NotExtended as u16,
                    description: "Not Extended"
                }
            ),
            511 => Ok(
                HTTPCodeInfo {
                    code: ServerError::NetworkAuthenticationRequired as u16,
                    description: "Network Authentication Required"
                }
            ),
            512..=599 => Ok(
                HTTPCodeInfo {
                    code: code.clone(),
                    description: "Unassigned"
                }
            ),
            _ => Err(String::from("Server Error. Unknown code!"))
        }
    }
}
