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

impl Redirection {
    pub fn value(&self) -> u16 {
        return match self {
            Redirection::MultipleChoices => Redirection::MultipleChoices as u16,
            Redirection::MovedPermanently => Redirection::MovedPermanently as u16,
            Redirection::Found => Redirection::Found as u16,
            Redirection::SeeOther => Redirection::SeeOther as u16,
            Redirection::NotModified => Redirection::NotModified as u16,
            Redirection::UseProxy => Redirection::UseProxy as u16,
            Redirection::TemporaryRedirect => Redirection::TemporaryRedirect as u16,
            Redirection::PermanentRedirect => Redirection::PermanentRedirect as u16
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            Redirection::MultipleChoices => "Multiple Choices",
            Redirection::MovedPermanently => "Moved Permanently",
            Redirection::Found => "Found",
            Redirection::SeeOther => "See Other",
            Redirection::NotModified => "Not Modified",
            Redirection::UseProxy => "Use Proxy",
            Redirection::TemporaryRedirect => "Temporary Redirect",
            Redirection::PermanentRedirect => "Permanent Redirect"
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
