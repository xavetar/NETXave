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

pub enum PAYLOAD_SIZE {
    PS_1024 = 1024,
    PS_2048 = 2048,
    PS_3072 = 3072,
    PS_4096 = 4096,
    PS_5120 = 5120,
    PS_6144 = 6144,
    PS_7168 = 7168,
    PS_8192 = 8192,
    PS_9216 = 9216,
    PS_10240 = 10240,
    PS_11264 = 11264,
    PS_12288 = 12288,
    PS_13312 = 13312,
    PS_14336 = 14336,
    PS_15360 = 15360,
    PS_16384 = 16384,
    PS_17408 = 17408,
    PS_18432 = 18432,
    PS_19456 = 19456,
    PS_20480 = 20480,
    PS_21504 = 21504,
    PS_22528 = 22528,
    PS_23552 = 23552,
    PS_24576 = 24576,
    PS_25600 = 25600,
    PS_26624 = 26624,
    PS_27648 = 27648,
    PS_28672 = 28672,
    PS_29696 = 29696,
    PS_30720 = 30720,
    PS_31744 = 31744,
    PS_32768 = 32768,
    PS_33792 = 33792,
    PS_34816 = 34816,
    PS_35840 = 35840,
    PS_36864 = 36864,
    PS_37888 = 37888,
    PS_38912 = 38912,
    PS_39936 = 39936,
    PS_40960 = 40960,
    PS_41984 = 41984,
    PS_43008 = 43008,
    PS_44032 = 44032,
    PS_45056 = 45056,
    PS_46080 = 46080,
    PS_47104 = 47104,
    PS_48128 = 48128,
    PS_49152 = 49152,
    PS_50176 = 50176,
    PS_51200 = 51200,
    PS_52224 = 52224,
    PS_53248 = 53248,
    PS_54272 = 54272,
    PS_55296 = 55296,
    PS_56320 = 56320,
    PS_57344 = 57344,
    PS_58368 = 58368,
    PS_59392 = 59392,
    PS_60416 = 60416,
    PS_61440 = 61440,
    PS_62464 = 62464,
    PS_63488 = 63488,
    PS_64512 = 64512,
    PS_65535 = 65535,
}

impl PAYLOAD_SIZE {
    pub fn code(&self) -> u16 {
        return match self {
            PAYLOAD_SIZE::PS_1024 => PAYLOAD_SIZE::PS_1024 as u16,
            PAYLOAD_SIZE::PS_2048 => PAYLOAD_SIZE::PS_2048 as u16,
            PAYLOAD_SIZE::PS_3072 => PAYLOAD_SIZE::PS_3072 as u16,
            PAYLOAD_SIZE::PS_4096 => PAYLOAD_SIZE::PS_4096 as u16,
            PAYLOAD_SIZE::PS_5120 => PAYLOAD_SIZE::PS_5120 as u16,
            PAYLOAD_SIZE::PS_6144 => PAYLOAD_SIZE::PS_6144 as u16,
            PAYLOAD_SIZE::PS_7168 => PAYLOAD_SIZE::PS_7168 as u16,
            PAYLOAD_SIZE::PS_8192 => PAYLOAD_SIZE::PS_8192 as u16,
            PAYLOAD_SIZE::PS_9216 => PAYLOAD_SIZE::PS_9216 as u16,
            PAYLOAD_SIZE::PS_10240 => PAYLOAD_SIZE::PS_10240 as u16,
            PAYLOAD_SIZE::PS_11264 => PAYLOAD_SIZE::PS_11264 as u16,
            PAYLOAD_SIZE::PS_12288 => PAYLOAD_SIZE::PS_12288 as u16,
            PAYLOAD_SIZE::PS_13312 => PAYLOAD_SIZE::PS_13312 as u16,
            PAYLOAD_SIZE::PS_14336 => PAYLOAD_SIZE::PS_14336 as u16,
            PAYLOAD_SIZE::PS_15360 => PAYLOAD_SIZE::PS_15360 as u16,
            PAYLOAD_SIZE::PS_16384 => PAYLOAD_SIZE::PS_16384 as u16,
            PAYLOAD_SIZE::PS_17408 => PAYLOAD_SIZE::PS_17408 as u16,
            PAYLOAD_SIZE::PS_18432 => PAYLOAD_SIZE::PS_18432 as u16,
            PAYLOAD_SIZE::PS_19456 => PAYLOAD_SIZE::PS_19456 as u16,
            PAYLOAD_SIZE::PS_20480 => PAYLOAD_SIZE::PS_20480 as u16,
            PAYLOAD_SIZE::PS_21504 => PAYLOAD_SIZE::PS_21504 as u16,
            PAYLOAD_SIZE::PS_22528 => PAYLOAD_SIZE::PS_22528 as u16,
            PAYLOAD_SIZE::PS_23552 => PAYLOAD_SIZE::PS_23552 as u16,
            PAYLOAD_SIZE::PS_24576 => PAYLOAD_SIZE::PS_24576 as u16,
            PAYLOAD_SIZE::PS_25600 => PAYLOAD_SIZE::PS_25600 as u16,
            PAYLOAD_SIZE::PS_26624 => PAYLOAD_SIZE::PS_26624 as u16,
            PAYLOAD_SIZE::PS_27648 => PAYLOAD_SIZE::PS_27648 as u16,
            PAYLOAD_SIZE::PS_28672 => PAYLOAD_SIZE::PS_28672 as u16,
            PAYLOAD_SIZE::PS_29696 => PAYLOAD_SIZE::PS_29696 as u16,
            PAYLOAD_SIZE::PS_30720 => PAYLOAD_SIZE::PS_30720 as u16,
            PAYLOAD_SIZE::PS_31744 => PAYLOAD_SIZE::PS_31744 as u16,
            PAYLOAD_SIZE::PS_32768 => PAYLOAD_SIZE::PS_32768 as u16,
            PAYLOAD_SIZE::PS_33792 => PAYLOAD_SIZE::PS_33792 as u16,
            PAYLOAD_SIZE::PS_34816 => PAYLOAD_SIZE::PS_34816 as u16,
            PAYLOAD_SIZE::PS_35840 => PAYLOAD_SIZE::PS_35840 as u16,
            PAYLOAD_SIZE::PS_36864 => PAYLOAD_SIZE::PS_36864 as u16,
            PAYLOAD_SIZE::PS_37888 => PAYLOAD_SIZE::PS_37888 as u16,
            PAYLOAD_SIZE::PS_38912 => PAYLOAD_SIZE::PS_38912 as u16,
            PAYLOAD_SIZE::PS_39936 => PAYLOAD_SIZE::PS_39936 as u16,
            PAYLOAD_SIZE::PS_40960 => PAYLOAD_SIZE::PS_40960 as u16,
            PAYLOAD_SIZE::PS_41984 => PAYLOAD_SIZE::PS_41984 as u16,
            PAYLOAD_SIZE::PS_43008 => PAYLOAD_SIZE::PS_43008 as u16,
            PAYLOAD_SIZE::PS_44032 => PAYLOAD_SIZE::PS_44032 as u16,
            PAYLOAD_SIZE::PS_45056 => PAYLOAD_SIZE::PS_45056 as u16,
            PAYLOAD_SIZE::PS_46080 => PAYLOAD_SIZE::PS_46080 as u16,
            PAYLOAD_SIZE::PS_47104 => PAYLOAD_SIZE::PS_47104 as u16,
            PAYLOAD_SIZE::PS_48128 => PAYLOAD_SIZE::PS_48128 as u16,
            PAYLOAD_SIZE::PS_49152 => PAYLOAD_SIZE::PS_49152 as u16,
            PAYLOAD_SIZE::PS_50176 => PAYLOAD_SIZE::PS_50176 as u16,
            PAYLOAD_SIZE::PS_51200 => PAYLOAD_SIZE::PS_51200 as u16,
            PAYLOAD_SIZE::PS_52224 => PAYLOAD_SIZE::PS_52224 as u16,
            PAYLOAD_SIZE::PS_53248 => PAYLOAD_SIZE::PS_53248 as u16,
            PAYLOAD_SIZE::PS_54272 => PAYLOAD_SIZE::PS_54272 as u16,
            PAYLOAD_SIZE::PS_55296 => PAYLOAD_SIZE::PS_55296 as u16,
            PAYLOAD_SIZE::PS_56320 => PAYLOAD_SIZE::PS_56320 as u16,
            PAYLOAD_SIZE::PS_57344 => PAYLOAD_SIZE::PS_57344 as u16,
            PAYLOAD_SIZE::PS_58368 => PAYLOAD_SIZE::PS_58368 as u16,
            PAYLOAD_SIZE::PS_59392 => PAYLOAD_SIZE::PS_59392 as u16,
            PAYLOAD_SIZE::PS_60416 => PAYLOAD_SIZE::PS_60416 as u16,
            PAYLOAD_SIZE::PS_61440 => PAYLOAD_SIZE::PS_61440 as u16,
            PAYLOAD_SIZE::PS_62464 => PAYLOAD_SIZE::PS_62464 as u16,
            PAYLOAD_SIZE::PS_63488 => PAYLOAD_SIZE::PS_63488 as u16,
            PAYLOAD_SIZE::PS_64512 => PAYLOAD_SIZE::PS_64512 as u16,
            PAYLOAD_SIZE::PS_65535 => PAYLOAD_SIZE::PS_65535 as u16
        }
    }
}
