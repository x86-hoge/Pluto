/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 * 
 * Copyright (c) 2020 kuriyama takumi <k018c1134@it-neec.jp>
 * 
 * Copyright (c) 2020 wakabayashi kensuke <k018c1128@it-neec.jp>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::MMIO_BASE;
use register::mmio::ReadWrite;

/*
000 input
001 output
100 function 0
101 function 1
110 function 2
111 function 3
011 function 4
010 function 5
*/

register_bitfields! {
    u32,

    /// GPIO Function Select 0
    GPFSEL0 [
        /// Pin 2
        FSEL2 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SDA1= 0b100
        ],

        /// Pin 3
        FSEL3 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SCL1= 0b100
        ],

        /// Pin 5  LED RED
        FSEL5 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        /// Pin 6 LED GREEN
        FSEL6 OFFSET(18) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        /// Pin 7
        FSEL7 OFFSET(21) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_CE1_IN = 0b100
        ],

        /// Pin 8
        FSEL8 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_CE0_IN = 0b100 // SPI0 Function 0
        ],

        /// Pin 9
        FSEL9 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_MISO = 0b100   // SPI0 Function 0
        ]
    ],

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 10
        FSEL10 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_MISI = 0b100
        ],

        /// Pin 11
        FSEL11 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_SLCK = 0b100
        ],

        /// Pin 13 LED BLUE
        FSEL13 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            SPI0_SLCK = 0b100
        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD1 = 0b010  // Mini UART - Alternate function 5
        ],
        
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD1 = 0b010  // Mini UART - Alternate function 5
        ]
    ],
    /// gpio set 0-31
    GPSET0[
        GP5  OFFSET( 5) NUMBITS(1)[UP=1],
        GP6  OFFSET( 6) NUMBITS(1)[UP=1],
        GP13 OFFSET(13) NUMBITS(1)[UP=1]
    ],
    GPCLR0[
        GP5  OFFSET( 5) NUMBITS(1)[UP=1],
        GP6  OFFSET( 6) NUMBITS(1)[UP=1],
        GP13 OFFSET(13) NUMBITS(1)[UP=1]
    ],
    /// GPIO Pull-up/down Clock Register 0
    GPPUDCLK0 [
        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

pub const GPFSEL0: *const ReadWrite<u32, GPFSEL0::Register> =
    (MMIO_BASE + 0x0020_0000) as *const ReadWrite<u32, GPFSEL0::Register>;

pub const GPFSEL1: *const ReadWrite<u32, GPFSEL1::Register> =
    (MMIO_BASE + 0x0020_0004) as *const ReadWrite<u32, GPFSEL1::Register>;

pub const GPSET0: *const ReadWrite<u32, GPSET0::Register> =
    (MMIO_BASE + 0x0020_001C) as *const ReadWrite<u32, GPSET0::Register>;

pub const GPCLR0: *const ReadWrite<u32, GPCLR0::Register> =
    (MMIO_BASE + 0x0020_0028) as *const ReadWrite<u32, GPCLR0::Register>;

//pub const GPFSEL2: *const ReadWrite<u32, GPFSEL2::Register> =
//    (MMIO_BASE + 0x0020_0008) as *const ReadWrite<u32, GPFSEL2::Register>;

pub const GPPUD: *const ReadWrite<u32> = (MMIO_BASE + 0x0020_0094) as *const ReadWrite<u32>;

pub const GPPUDCLK0: *const ReadWrite<u32, GPPUDCLK0::Register> =
    (MMIO_BASE + 0x0020_0098) as *const ReadWrite<u32, GPPUDCLK0::Register>;
