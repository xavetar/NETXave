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

///     Base:
///
///     ┌───────────────────────────────────────────────┐
///     │                  MESSAGE ID                   │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │  QR ┐                                         │ ┌─> QR     ─> 1 bit
///     │  │  └ OPCODE ┐                                │ ├─> OPCODE ─> 4 bits
///     │  │      │    └ AA ┐                           │ ├─> AA     ─> 1 bit
///     │  │      │      │  └ TC ┐                      │ ├─> TC     ─> 1 bit
///     │  │      │      │    │  └ RD ┐                 │ ├─> RD     ─> 1 bit
///     │  │      │      │    │    │  └ RA ┐            │ ├─> RA     ─> 1 bit
///     │  │      │      │    │    │    │  └ Z ┐        │ ├─> Z      ─> 3 bits
///     │  │      │      │    │    │    │    │ └ RCODE  │ ├─> RCODE  ─> 4 bits
///     ├──┴──┬───┴────┬─┴──┬─┴──┬─┴──┬─┴──┬─┴─┬───┴────┤ │
///     │  QR │ OPCODE │ AA │ TC │ RD │ RA │ Z │ RCODE  │ └─> Flags  ─> 16 bits
///     ├─────┴────────┴────┴────┴────┴────┴───┴────────┤
///     │                    QDCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    ANCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    NSCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    ARCOUNT                    │ <─> 16 bits
///     └───────────────────────────────────────────────┘
///
///     Security Extension (DNSSE):
///
///     ┌───────────────────────────────────────────────┐
///     │                  MESSAGE ID                   │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │QR ┐                                           │ ┌─> QR     ─> 1 bit
///     │ │ └ OPCODE ┐                                  │ ├─> OPCODE ─> 4 bits
///     │ │     │    └ AA┐                              │ ├─> AA     ─> 1 bit
///     │ │     │      │ └TC┐                           │ ├─> TC     ─> 1 bit
///     │ │     │      │  │ └RD┐                        │ ├─> RD     ─> 1 bit
///     │ │     │      │  │  │ └RA┐                     │ ├─> RA     ─> 1 bit
///     │ │     │      │  │  │  │ └None┐                │ ├─> None   ─> 1 bit <─┐
///     │ │     │      │  │  │  │   │  └AD┐             │ ├─> AD     ─> 1 bit <─┼──> Z ─> 3 bits
///     │ │     │      │  │  │  │   │   │ └CD┐          │ ├─> CD     ─> 1 bit <─┘
///     │ │     │      │  │  │  │   │   │  │ └─ RCODE   │ ├─> RCODE  ─> 4 bits
///     ├─┴┬────┴─────┬┴─┬┴─┬┴─┬┴─┬─┴──┬┴─┬┴─┬────┴─────┤ │
///     │QR│  OPCODE  │AA│TC│RD│RA│None│AD│CD│  RCODE   │ └─> Flags  ─> 16 bits
///     ├──┴──────────┴──┴──┴──┴──┴────┴──┴──┴──────────┤
///     │                    QDCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    ANCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    NSCOUNT                    │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │                    ARCOUNT                    │ <─> 16 bits
///     └───────────────────────────────────────────────┘
///
///     DSO:
///
///     ┌───────────────────────────────────────────────┐
///     │                  MESSAGE ID                   │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │  QR ┐                                         │ ┌─> QR     ─> 1 bit
///     │  │  └ OPCODE ┐                                │ ├─> OPCODE ─> 4 bits (6)
///     │  │      │    └─────────┐                      │ │
///     │  │      │              └ Z ┐                  │ ├─> Z      ─> 7 bits
///     │  │      │                │ └─────────┐        │ │
///     │  │      │                │           └ RCODE  │ ├─> RCODE  ─> 4 bits
///     ├──┴──┬───┴────┬───────────┴───────────┬───┴────┤ │
///     │  QR │ Opcode │           Z           │ RCODE  │ └─> Flags  ─> 16 bits
///     ├─────┴────────┴───────────────────────┴────────┤
///     │             QDCOUNT (MUST BE ZERO)            │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │             ANCOUNT (MUST BE ZERO)            │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │             NSCOUNT (MUST BE ZERO)            │ <─> 16 bits
///     ├───────────────────────────────────────────────┤
///     │             ARCOUNT (MUST BE ZERO)            │ <─> 16 bits
///     └───────────────────────────────────────────────┘

pub mod Flags;
