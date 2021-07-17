use super::MMIO_BASE;
use super::timer::ARM_TIMER_CLI;
use super::uart;
use register::mmio::*;

register_bitfields! {
    u32,
    IRQ_BASIC[
        ARM_TIMER OFFSET(0) NUMBITS(1) []
    ],
    IRQ_PEND1[
        SYSTEM_TIMER1 OFFSET(1) NUMBITS(1) [],
        AUX OFFSET(29) NUMBITS(1) []
    ],
    IRQ_ENABLE_BASIC[
        ARM_TIMER OFFSET(0) NUMBITS(1) []
    ],
    IRQ_ENABLE_PEND1[
        SYSTEM_TIMER1 OFFSET(1) NUMBITS(1) [],
        AUX OFFSET(29) NUMBITS(1) []
    ],
    IRQ_DISABLE_BASIC[
        ARM_TIMER OFFSET(0) NUMBITS(1) []
    ],
    IRQ_DISABLE_PEND1[
        SYSTEM_TIMER1 OFFSET(1) NUMBITS(1) [],
        AUX OFFSET(29) NUMBITS(1) []
    ]
}

const IRQ_BASE:u32 = MMIO_BASE + 0xB200 ;
pub const IRQ_BASIC        : *const ReadWrite<u32, IRQ_BASIC::Register>  = (IRQ_BASE + 0x00) as *const ReadWrite<u32, IRQ_BASIC::Register>;
const IRQ_PEND1        : *const ReadWrite<u32, IRQ_PEND1::Register>  = (IRQ_BASE + 0x04) as *const ReadWrite<u32, IRQ_PEND1::Register>;
//const IRQ_PEND2        : *const ReadWrite<u32>  = (IRQ_BASE + 0x08) as *const ReadWrite<u32>;
//const IRQ_FIQ_CONTROL  : *const ReadWrite<u32>  = (IRQ_BASE + 0x0C) as *const ReadWrite<u32>;
const IRQ_ENABLE_PEND1 : *const ReadWrite<u32, IRQ_ENABLE_PEND1::Register>  = (IRQ_BASE + 0x10) as *const ReadWrite<u32, IRQ_ENABLE_PEND1::Register>;
//const IRQ_ENABLE_PEND2 : *const ReadWrite<u32>  = (IRQ_BASE + 0x14) as *const ReadWrite<u32>;
const IRQ_ENABLE_BASIC : *const ReadWrite<u32, IRQ_ENABLE_BASIC::Register>  = (IRQ_BASE + 0x18) as *const ReadWrite<u32, IRQ_ENABLE_BASIC::Register>;
const IRQ_DISABLE_PEND1: *const ReadWrite<u32, IRQ_DISABLE_PEND1::Register>  = (IRQ_BASE + 0x1C) as *const ReadWrite<u32, IRQ_DISABLE_PEND1::Register>;
//const IRQ_DISABLE_PEND2: *const ReadWrite<u32>  = (IRQ_BASE + 0x20) as *const ReadWrite<u32>;
const IRQ_DISABLE_BASIC: *const ReadWrite<u32, IRQ_DISABLE_BASIC::Register>  = (IRQ_BASE + 0x24) as *const ReadWrite<u32, IRQ_DISABLE_BASIC::Register>;


static mut LED_CNT:u32 = 0;

pub fn irq_enable_begin(){
    unsafe{
        (*IRQ_ENABLE_BASIC).write(IRQ_ENABLE_BASIC::ARM_TIMER::SET);
    }
    //(*IRQ_ENABLE_PEND1).write(IRQ_ENABLE_PEND1::AUX::SET);
}





/*
pub struct Task_t{
    stack_addr:u32,
    sp:*mut u32,
    r_reg:[u32,13];
    lr:*mut u8,
    pc:*mut u8,
    cpsr:u32,
}
 */