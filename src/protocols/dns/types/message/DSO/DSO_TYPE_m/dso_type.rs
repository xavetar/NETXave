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

pub enum DSO_TYPE {
    KEEP_ALIVE = 1,
    RETRY_DELAY = 2,
    ENCRYPTION_PADDING = 3,
    SUBSCRIBE = 64,
    PUSH = 65,
    UNSUBSCRIBE = 66,
    RECONFIRM = 67,
}

impl DSO_TYPE {
    pub fn code(&self) -> u16 {
        return match self {
            DSO_TYPE::KEEP_ALIVE => DSO_TYPE::KEEP_ALIVE as u16,
            DSO_TYPE::RETRY_DELAY => DSO_TYPE::RETRY_DELAY as u16,
            DSO_TYPE::ENCRYPTION_PADDING => DSO_TYPE::ENCRYPTION_PADDING as u16,
            DSO_TYPE::SUBSCRIBE => DSO_TYPE::SUBSCRIBE as u16,
            DSO_TYPE::PUSH => DSO_TYPE::PUSH as u16,
            DSO_TYPE::UNSUBSCRIBE => DSO_TYPE::UNSUBSCRIBE as u16,
            DSO_TYPE::RECONFIRM => DSO_TYPE::RECONFIRM as u16
        }
    }

    pub fn name(&self) -> &'static str {
        return match self {
            DSO_TYPE::KEEP_ALIVE => "Message to keep the connection alive",
            DSO_TYPE::RETRY_DELAY => "Delay before retry",
            DSO_TYPE::ENCRYPTION_PADDING => "Encryption Padding",
            DSO_TYPE::SUBSCRIBE => "Subscription Operation",
            DSO_TYPE::PUSH => "Send Data Operation",
            DSO_TYPE::UNSUBSCRIBE => "Unsubscribe Operation",
            DSO_TYPE::RECONFIRM => "Reconfirm Operation",
        }
    }

    pub fn rtt0(&self) -> bool {
        return match self {
            DSO_TYPE::KEEP_ALIVE => true,
            DSO_TYPE::RETRY_DELAY => false,
            DSO_TYPE::ENCRYPTION_PADDING => false,
            DSO_TYPE::SUBSCRIBE => true,
            DSO_TYPE::PUSH => false,
            DSO_TYPE::UNSUBSCRIBE => false,
            DSO_TYPE::RECONFIRM => false
        }
    }
}
