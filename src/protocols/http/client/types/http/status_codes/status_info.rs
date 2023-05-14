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

use super::{UNUSED, UNASSIGNED};
use super::{Informational, Success, Redirection, ClientError, ServerError};
use super::{HTTPCodeInfo};

pub trait HTTPStatusInfo {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String>;
}

impl HTTPStatusInfo for Informational {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            100 => Ok(
                HTTPCodeInfo::new(Informational::Continue.value(),
                                  Informational::Continue.description())
            ),
            101 => Ok(
                HTTPCodeInfo::new(Informational::SwitchingProtocols.value(),
                                  Informational::SwitchingProtocols.description())
            ),
            102 => Ok(
                HTTPCodeInfo::new(Informational::Processing.value(),
                                  Informational::Processing.description())
            ),
            103 => Ok(
                HTTPCodeInfo::new(Informational::EarlyHints.value(),
                                  Informational::EarlyHints.description())
            ),
            104..=199 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Informational code!"))
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

impl HTTPStatusInfo for Redirection {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            300 => Ok(
                HTTPCodeInfo::new(Redirection::MultipleChoices.value(),
                                  Redirection::MultipleChoices.description())
            ),
            301 => Ok(
                HTTPCodeInfo::new(Redirection::MovedPermanently.value(),
                                  Redirection::MovedPermanently.description())
            ),
            302 => Ok(
                HTTPCodeInfo::new(Redirection::Found.value(),
                                  Redirection::Found.description())
            ),
            303 => Ok(
                HTTPCodeInfo::new(Redirection::SeeOther.value(),
                                  Redirection::SeeOther.description())
            ),
            304 => Ok(
                HTTPCodeInfo::new(Redirection::NotModified.value(),
                                  Redirection::NotModified.description())
            ),
            305 => Ok(
                HTTPCodeInfo::new(Redirection::UseProxy.value(),
                                  Redirection::UseProxy.description())
            ),
            306 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNUSED)
            ),
            307 => Ok(
                HTTPCodeInfo::new(Redirection::TemporaryRedirect.value(),
                                  Redirection::TemporaryRedirect.description())
            ),
            308 => Ok(
                HTTPCodeInfo::new(Redirection::PermanentRedirect.value(),
                                  Redirection::PermanentRedirect.description())
            ),
            309..=399 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Redirection code!"))
        }
    }
}

impl HTTPStatusInfo for ClientError {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            400 => Ok(
                HTTPCodeInfo::new(ClientError::BadRequest.value(),
                                  ClientError::BadRequest.description())
            ),
            401 => Ok(
                HTTPCodeInfo::new(ClientError::Unauthorized.value(),
                                  ClientError::Unauthorized.description())
            ),
            402 => Ok(
                HTTPCodeInfo::new(ClientError::PaymentRequired.value(),
                                  ClientError::PaymentRequired.description())
            ),
            403 => Ok(
                HTTPCodeInfo::new(ClientError::Forbidden.value(),
                                  ClientError::Forbidden.description())
            ),
            404 => Ok(
                HTTPCodeInfo::new(ClientError::NotFound.value(),
                                  ClientError::NotFound.description())
            ),
            405 => Ok(
                HTTPCodeInfo::new(ClientError::MethodNotAllowed.value(),
                                  ClientError::MethodNotAllowed.description())
            ),
            406 => Ok(
                HTTPCodeInfo::new(ClientError::NotAcceptable.value(),
                                  ClientError::NotAcceptable.description())
            ),
            407 => Ok(
                HTTPCodeInfo::new(ClientError::ProxyAuthenticationRequired.value(),
                                  ClientError::ProxyAuthenticationRequired.description())
            ),
            408 => Ok(
                HTTPCodeInfo::new(ClientError::RequestTimeout.value(),
                                  ClientError::RequestTimeout.description())
            ),
            409 => Ok(
                HTTPCodeInfo::new(ClientError::Conflict.value(),
                                  ClientError::Conflict.description())
            ),
            410 => Ok(
                HTTPCodeInfo::new(ClientError::Gone.value(),
                                  ClientError::Gone.description())
            ),
            411 => Ok(
                HTTPCodeInfo::new(ClientError::LengthRequired.value(),
                                  ClientError::LengthRequired.description())
            ),
            412 => Ok(
                HTTPCodeInfo::new(ClientError::PreconditionFailed.value(),
                                  ClientError::PreconditionFailed.description())
            ),
            413 => Ok(
                HTTPCodeInfo::new(ClientError::ContentTooLarge.value(),
                                  ClientError::ContentTooLarge.description())
            ),
            414 => Ok(
                HTTPCodeInfo::new(ClientError::URITooLong.value(),
                                  ClientError::URITooLong.description())
            ),
            415 => Ok(
                HTTPCodeInfo::new(ClientError::UnsupportedMediaType.value(),
                                  ClientError::UnsupportedMediaType.description())
            ),
            416 => Ok(
                HTTPCodeInfo::new(ClientError::RangeNotSatisfiable.value(),
                                  ClientError::RangeNotSatisfiable.description())
            ),
            417 => Ok(
                HTTPCodeInfo::new(ClientError::ExpectationFailed.value(),
                                  ClientError::ExpectationFailed.description())
            ),
            418 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNUSED)
            ),
            419..=420 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            421 => Ok(
                HTTPCodeInfo::new(ClientError::MisdirectedRequest.value(),
                                  ClientError::MisdirectedRequest.description())
            ),
            422 => Ok(
                HTTPCodeInfo::new(ClientError::UnprocessableContent.value(),
                                  ClientError::UnprocessableContent.description())
            ),
            423 => Ok(
                HTTPCodeInfo::new(ClientError::Locked.value(),
                                  ClientError::Locked.description())
            ),
            424 => Ok(
                HTTPCodeInfo::new(ClientError::FailedDependency.value(),
                                  ClientError::FailedDependency.description())
            ),
            425 => Ok(
                HTTPCodeInfo::new(ClientError::TooEarly.value(),
                                  ClientError::TooEarly.description())
            ),
            426 => Ok(
                HTTPCodeInfo::new(ClientError::UpgradeRequired.value(),
                                  ClientError::UpgradeRequired.description())
            ),
            427 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            428 => Ok(
                HTTPCodeInfo::new(ClientError::PreconditionRequired.value(),
                                  ClientError::PreconditionRequired.description())
            ),
            429 => Ok(
                HTTPCodeInfo::new(ClientError::TooManyRequests.value(),
                                  ClientError::TooManyRequests.description())
            ),
            430 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            431 => Ok(
                HTTPCodeInfo::new(ClientError::RequestHeaderFieldsTooLarge.value(),
                                  ClientError::RequestHeaderFieldsTooLarge.description())
            ),
            432..=450 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            451 => Ok(
                HTTPCodeInfo::new(ClientError::UnavailableForLegalReasons.value(),
                                  ClientError::UnavailableForLegalReasons.description())
            ),
            452..=499 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Client Error code!"))
        }
    }
}

impl HTTPStatusInfo for ServerError {
    fn info(code: &u16) -> Result<HTTPCodeInfo, String> {
        return match *code {
            500 => Ok(
                HTTPCodeInfo::new(ServerError::InternalServerError.value(),
                                  ServerError::InternalServerError.description())
            ),
            501 => Ok(
                HTTPCodeInfo::new(ServerError::NotImplemented.value(),
                                  ServerError::NotImplemented.description())
            ),
            502 => Ok(
                HTTPCodeInfo::new(ServerError::BadGateway.value(),
                                  ServerError::BadGateway.description())
            ),
            503 => Ok(
                HTTPCodeInfo::new(ServerError::ServiceUnavailable.value(),
                                  ServerError::ServiceUnavailable.description())
            ),
            504 => Ok(
                HTTPCodeInfo::new(ServerError::GatewayTimeout.value(),
                                  ServerError::GatewayTimeout.description())
            ),
            505 => Ok(
                HTTPCodeInfo::new(ServerError::HTTPVersionNotSupported.value(),
                                  ServerError::HTTPVersionNotSupported.description())
            ),
            506 => Ok(
                HTTPCodeInfo::new(ServerError::VariantAlsoNegotiates.value(),
                                  ServerError::VariantAlsoNegotiates.description())
            ),
            507 => Ok(
                HTTPCodeInfo::new(ServerError::InsufficientStorage.value(),
                                  ServerError::InsufficientStorage.description())
            ),
            508 => Ok(
                HTTPCodeInfo::new(ServerError::LoopDetected.value(),
                                  ServerError::LoopDetected.description())
            ),
            509 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            510 => Ok(
                HTTPCodeInfo::new(ServerError::NotExtended.value(),
                                  ServerError::NotExtended.description())
            ),
            511 => Ok(
                HTTPCodeInfo::new(ServerError::NetworkAuthenticationRequired.value(),
                                  ServerError::NetworkAuthenticationRequired.description())
            ),
            512..=599 => Ok(
                HTTPCodeInfo::new(*code,
                                  UNASSIGNED)
            ),
            _ => Err(String::from("Unknown Server Error code!"))
        }
    }
}
