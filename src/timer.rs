use super::MMIO_BASE;
use core::ops;
use register::mmio::*;

const SYSTEM_TIMER_BASE: u32 = MMIO_BASE + 0x3000;
const SYSTEM_TIMER_CLO: *const ReadWrite<u32> = (SYSTEM_TIMER_BASE + 0x04) as *const ReadWrite<u32>; // lower 32bit 
const SYSTEM_TIMER_CHI: *const ReadWrite<u32> = (SYSTEM_TIMER_BASE + 0x08) as *const ReadWrite<u32>; // higher 32bit

fn system_time() -> u64{
    let clo;
    let chi;
    unsafe{
        clo = (*SYSTEM_TIMER_CLO).get();
        chi = (*SYSTEM_TIMER_CHI).get();
    }
    let mut ret:u64 = (chi as u64) << 32;
    ret |= clo as u64;
    ret
}

//引数で受け取ったNミリ秒処理を停止
pub fn delay_time(msec:u64){
    let end_time:u64;
    end_time = system_time() + msec * 1000;
    loop{
        if system_time() > end_time{
            break;
        }
    }
}

register_bitfields! {
    u32,
    /// Auxiliary enables
    ARM_TIMER_CTL[
        COUNTER OFFSET(1) NUMBITS(1) [
            COUNTER_16BIT = 0,
            COUNTER_23BIT = 1
        ],
        TIMER_IRQ_ENABLE OFFSET(5) NUMBITS(1) [
            ENABLE = 1,
            DISABLE = 0
        ],
        TIMER_ENABLE OFFSET(7) NUMBITS(1) [
            ENABLE = 1,
            DISABLE = 0
        ],
        FREE_RUN_CNT OFFSET(16) NUMBITS(8) [
            RESET = 0x3E
        ]
    ]
}

const ARM_TIMER_BASE: u32 = MMIO_BASE + 0xB400;
const ARM_TIMER_LOD : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x00) as *const ReadWrite<u32>;
const ARM_TIMER_VAL : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x04) as *const ReadWrite<u32>;
const ARM_TIMER_CTL : *const ReadWrite<u32, ARM_TIMER_CTL::Register> = (ARM_TIMER_BASE + 0x08) as *const ReadWrite<u32, ARM_TIMER_CTL::Register>;
pub const ARM_TIMER_CLI : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x0C) as *const ReadWrite<u32>;
const ARM_TIMER_RIS : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x10) as *const ReadWrite<u32>;
const ARM_TIMER_MIS : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x14) as *const ReadWrite<u32>;
const ARM_TIMER_RLD : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x18) as *const ReadWrite<u32>;
const ARM_TIMER_DIV : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x1C) as *const ReadWrite<u32>;
const ARM_TIMER_CNT : *const ReadWrite<u32> = (ARM_TIMER_BASE + 0x20) as *const ReadWrite<u32>;


pub fn arm_timer_begin(){
    unsafe{
        (*ARM_TIMER_CTL).write(ARM_TIMER_CTL::COUNTER::COUNTER_23BIT + ARM_TIMER_CTL::FREE_RUN_CNT::RESET);
        (*ARM_TIMER_LOD).set(2_000_000 -1);//500ms
        (*ARM_TIMER_RLD).set(1_000_000 -1);//リロード
        (*ARM_TIMER_DIV).set(0x0000_00F9);//1Mhz
        (*ARM_TIMER_CLI).set(0);//初期化
        (*ARM_TIMER_CTL).modify(ARM_TIMER_CTL::TIMER_ENABLE::ENABLE + ARM_TIMER_CTL::TIMER_IRQ_ENABLE::ENABLE);
    }
}