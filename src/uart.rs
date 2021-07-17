/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
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
use core::ops;
use super::gpio;
use register::mmio::*;

// Auxilary mini UART registers
//
// Descriptions taken from
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    /// Auxiliary enables
    AUX_ENABLES [
        /// If set the mini UART is enabled. The UART will immediately
        /// start receiving data, especially if the UART1_RX line is
        /// low.
        /// If clear the mini UART is disabled. That also disables any
        /// mini UART register access
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Mini Uart Interrupt Identify
    AUX_MU_IIR [
        /// Writing with bit 1 set will clear the receive FIFO
        /// Writing with bit 2 set will clear the transmit FIFO
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Rx = 0b01,
            Tx = 0b10,
            All = 0b11
        ]
    ],

    /// Mini Uart Line Control
    AUX_MU_LCR [
        /// Mode the UART works in
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],

    /// Mini Uart Line Status
    AUX_MU_LSR [
        /// This bit is set if the transmit FIFO can accept at least
        /// one byte.
        TX_EMPTY   OFFSET(5) NUMBITS(1) [],

        /// This bit is set if the receive FIFO holds at least 1
        /// symbol.
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],

    /// Mini Uart Extra Control
    AUX_MU_CNTL [
        /// If this bit is set the mini UART transmitter is enabled.
        /// If this bit is clear the mini UART transmitter is disabled.
        TX_EN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// If this bit is set the mini UART receiver is enabled.
        /// If this bit is clear the mini UART receiver is disabled.
        RX_EN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Mini Uart Baudrate
    AUX_MU_BAUD [
        /// Mini UART baudrate counter
        RATE OFFSET(0) NUMBITS(16) []
    ]
}

pub const MINI_UART_BASE: u32 = MMIO_BASE + 0x21_5000;

pub const AUX_ENABLES: *const ReadWrite<u32, AUX_ENABLES::Register>  = (MINI_UART_BASE + 0x04) as *const ReadWrite<u32, AUX_ENABLES::Register>;
pub const AUX_MU_IO  : *const ReadWrite<u32>                         = (MINI_UART_BASE + 0x40) as *const ReadWrite<u32>;// 0x40 - Mini Uart I/O Data
pub const AUX_MU_IER : *const WriteOnly<u32>                         = (MINI_UART_BASE + 0x44) as *const WriteOnly<u32>;// 0x44 - Mini Uart Interrupt Enable
pub const AUX_MU_IIR : *const WriteOnly<u32, AUX_MU_IIR::Register>   = (MINI_UART_BASE + 0x48) as *const WriteOnly<u32, AUX_MU_IIR::Register>;
pub const AUX_MU_LCR : *const WriteOnly<u32, AUX_MU_LCR::Register>   = (MINI_UART_BASE + 0x4C) as *const WriteOnly<u32, AUX_MU_LCR::Register>;
pub const AUX_MU_MCR : *const WriteOnly<u32>                         = (MINI_UART_BASE + 0x50) as *const WriteOnly<u32>;
pub const AUX_MU_LSR : *const ReadOnly<u32, AUX_MU_LSR::Register>    = (MINI_UART_BASE + 0x54) as *const ReadOnly<u32, AUX_MU_LSR::Register>;
pub const AUX_MU_CNTL: *const WriteOnly<u32, AUX_MU_CNTL::Register>  = (MINI_UART_BASE + 0x60) as *const WriteOnly<u32, AUX_MU_CNTL::Register>;
pub const AUX_MU_BAUD: *const WriteOnly<u32, AUX_MU_BAUD::Register>  = (MINI_UART_BASE + 0x68) as *const WriteOnly<u32, AUX_MU_BAUD::Register>;

    ///Set baud rate and characteristics (115200 8N1) and map to GPIO
    pub fn init() {
        // initialize UART
        unsafe{
            (*AUX_ENABLES).modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
            (*AUX_MU_IER).set(0);
            (*AUX_MU_CNTL).set(0);
            (*AUX_MU_LCR).write(AUX_MU_LCR::DATA_SIZE::EightBit);
            (*AUX_MU_MCR).set(0);
            (*AUX_MU_IER).set(0);
            (*AUX_MU_IIR).write(AUX_MU_IIR::FIFO_CLEAR::All);
            (*AUX_MU_BAUD).write(AUX_MU_BAUD::RATE.val(270)); // 115200 baud
        }      
        // map UART1 to GPIO pins
        unsafe {
            (*gpio::GPFSEL1).modify(gpio::GPFSEL1::FSEL14::TXD1 + gpio::GPFSEL1::FSEL15::RXD1);

            (*gpio::GPPUD).set(0); // enable pins 14 and 15
            for _ in 0..150 {
                asm!("nop" :::: "volatile");
            }

            (*gpio::GPPUDCLK0).write(
                gpio::GPPUDCLK0::PUDCLK14::AssertClock + gpio::GPPUDCLK0::PUDCLK15::AssertClock,
            );
            for _ in 0..150 {
                asm!("nop" :::: "volatile");
            }

            (*gpio::GPPUDCLK0).set(0);
        

        (*AUX_MU_CNTL)
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);
        }
    }

    /// Send a character
    pub fn send(c: char) {
        // wait until we can send
        unsafe{
        loop {
            if (*AUX_MU_LSR).is_set(AUX_MU_LSR::TX_EMPTY) {
                break;
            }
            asm!("nop" :::: "volatile");
        }
        // write the character to the buffer
        (*AUX_MU_IO).set(c as u32);
    }
    }

    /// Receive a character
    pub fn getc() -> char {
        // wait until something is in the buffer
        unsafe {
        loop {
            if (*AUX_MU_LSR).is_set(AUX_MU_LSR::DATA_READY) {
                break;
            }

            asm!("nop" :::: "volatile") ;
        }

        // read it and return
        let ret = (*AUX_MU_IO).get() as u8 as char;
        // convert carrige return to newline
        ret    
        }
    }

    /// Display a string
    pub fn puts(string: &str) {
        for c in string.chars() {
            // convert newline to carrige return + newline
            if c == '\n' {
                send('\r')
            }
            send(c);
        }
    }