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

use super::{DSOTYPE};
use super::{DSOTYPEInfo};
use super::{
    RESERVED, UNASSIGNED, RESERVED_FUTURE_EXPANSION_USE, RESERVED_EXPERIMENTAL_LOCAL_USE,
    UNASSIGNED_RESERVED_DSO_SESSION_MANAGEMENT_TLV
};


pub trait DSOTYPEConversion {
    fn encode(name: &str) -> Result<DSOTYPEInfo, String>;
    fn decode(dec: &u16) -> Result<DSOTYPEInfo, String>;
}

impl DSOTYPEConversion for DSOTYPE {
    fn encode(name: &str) -> Result<DSOTYPEInfo, String> {
        return match name {
            "KEEP_ALIVE" => Ok(
                DSOTYPEInfo::new(DSOTYPE::KEEP_ALIVE.name(),
                                 DSOTYPE::KEEP_ALIVE.code(),
                                 DSOTYPE::KEEP_ALIVE.rtt0())
            ),
            "RETRY_DELAY" => Ok(
                DSOTYPEInfo::new(DSOTYPE::RETRY_DELAY.name(),
                                 DSOTYPE::RETRY_DELAY.code(),
                                 DSOTYPE::RETRY_DELAY.rtt0())
            ),
            "ENCRYPTION_PADDING" => Ok(
                DSOTYPEInfo::new(DSOTYPE::ENCRYPTION_PADDING.name(),
                                 DSOTYPE::ENCRYPTION_PADDING.code(),
                                 DSOTYPE::ENCRYPTION_PADDING.rtt0())
            ),
            "SUBSCRIBE" => Ok(
                DSOTYPEInfo::new(DSOTYPE::SUBSCRIBE.name(),
                                 DSOTYPE::SUBSCRIBE.code(),
                                 DSOTYPE::SUBSCRIBE.rtt0())
            ),
            "PUSH" => Ok(
                DSOTYPEInfo::new(DSOTYPE::PUSH.name(),
                                 DSOTYPE::PUSH.code(),
                                 DSOTYPE::PUSH.rtt0())
            ),
            "UNSUBSCRIBE" => Ok(
                DSOTYPEInfo::new(DSOTYPE::UNSUBSCRIBE.name(),
                                 DSOTYPE::UNSUBSCRIBE.code(),
                                 DSOTYPE::UNSUBSCRIBE.rtt0())
            ),
            "RECONFIRM" => Ok(
                DSOTYPEInfo::new(DSOTYPE::RECONFIRM.name(),
                                 DSOTYPE::RECONFIRM.code(),
                                 DSOTYPE::RECONFIRM.rtt0())
            ),
            _ => Err(String::from("Can't encode DSO-TYPE!"))
        }
    }

    fn decode(decimal: &u16) -> Result<DSOTYPEInfo, String> {
        return match *decimal {
            0 => Ok(
                DSOTYPEInfo::new(RESERVED,
                                 *decimal,
                                 false)
            ),
            1 => Ok(
                DSOTYPEInfo::new(DSOTYPE::KEEP_ALIVE.name(),
                                 DSOTYPE::KEEP_ALIVE.code(),
                                 DSOTYPE::KEEP_ALIVE.rtt0())
            ),
            2 => Ok(
                DSOTYPEInfo::new(DSOTYPE::RETRY_DELAY.name(),
                                 DSOTYPE::RETRY_DELAY.code(),
                                 DSOTYPE::RETRY_DELAY.rtt0())
            ),
            3 => Ok(
                DSOTYPEInfo::new(DSOTYPE::ENCRYPTION_PADDING.name(),
                                 DSOTYPE::ENCRYPTION_PADDING.code(),
                                 DSOTYPE::ENCRYPTION_PADDING.rtt0())
            ),
            4..=63 => Ok(
                DSOTYPEInfo::new(UNASSIGNED_RESERVED_DSO_SESSION_MANAGEMENT_TLV,
                                 *decimal,
                                 false)
            ),
            64 => Ok(
                DSOTYPEInfo::new(DSOTYPE::SUBSCRIBE.name(),
                                 DSOTYPE::SUBSCRIBE.code(),
                                 DSOTYPE::SUBSCRIBE.rtt0())
            ),
            65 => Ok(
                DSOTYPEInfo::new(DSOTYPE::PUSH.name(),
                                 DSOTYPE::PUSH.code(),
                                 DSOTYPE::PUSH.rtt0())
            ),
            66 => Ok(
                DSOTYPEInfo::new(DSOTYPE::UNSUBSCRIBE.name(),
                                 DSOTYPE::UNSUBSCRIBE.code(),
                                 DSOTYPE::UNSUBSCRIBE.rtt0())
            ),
            67 => Ok(
                DSOTYPEInfo::new(DSOTYPE::RECONFIRM.name(),
                                 DSOTYPE::RECONFIRM.code(),
                                 DSOTYPE::RECONFIRM.rtt0())
            ),
            68..=63487 => Ok(
                DSOTYPEInfo::new(UNASSIGNED,
                                 *decimal,
                                 false)
            ),
            63488..=64511 => Ok(
                DSOTYPEInfo::new(RESERVED_EXPERIMENTAL_LOCAL_USE,
                                 *decimal,
                                 false)
            ),
            64512..=65535 => Ok(
                DSOTYPEInfo::new(RESERVED_FUTURE_EXPANSION_USE,
                                 *decimal,
                                 false)
            ),
            _ => Err(String::from("Can't decode DSO-TYPE!"))
        }
    }
}
