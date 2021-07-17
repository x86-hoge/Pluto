#![no_std]
#![no_main]
#![feature(asm,lang_items,global_asm)]
#![feature(alloc_error_handler)]
#[macro_use]
extern crate register;

const MMIO_BASE: u32 = 0x2000_0000;

use enc28j60::ENC28J60;
use linked_list_allocator::LockedHeap;
use core::alloc::*;
use task::IRQ_BASIC;
use timer::ARM_TIMER_CLI;
use util::*;

mod gpio;
mod uart;
mod shell;
mod led;
mod spi;
mod enc28j60;
mod timer;
mod task;
mod util;



#[global_allocator]
static ALLOCATOR:LockedHeap = LockedHeap::empty();

extern{
    pub fn enable_irq();
}


#[no_mangle]
fn kernel_entry() -> ! {
    uart::init();
    led::init();
    spi::init();
    let shell = shell::Shell::new();
    shell.init();
    let mut enc28j60 = ENC28J60::new();
    let mac_addr:[u8;6] = [0x12,0x34,0x24,0xa3,0xff,0x12];
    shell.print("inited\n");
    timer::arm_timer_begin();
    task::irq_enable_begin();
    //unsafe{enable_irq();}
    //spi::test();
    let ret_begin = enc28j60.begin(&mac_addr); uart::puts("begin enc28j60...\n");
    if ret_begin == 0{
        uart::puts(" failed to access ethrnet controller\n");
        loop{unsafe{asm!("nop" :::: "volatile") ;}}
    }
    let mut linkup = enc28j60.is_linkup();
    let mut buf:[u8;700] = [0x00;700];
    let mut buf_ptr = &mut buf[0] as *mut u8;
    timer::delay_time(100);
    enc28j60.power_down();
    let mut cnt:u8 = 0;
    loop{
        uart::puts("\n======start======\n");
        if linkup != enc28j60.is_linkup(){
            linkup = enc28j60.is_linkup();
            uart::puts("link change\n");
            timer::delay_time(100);
        }
        uart::puts("show link state:");
        if linkup{
            uart::puts("true\n");
        }else{
            uart::puts("false\n");
        }
        enc28j60.reveive_packet(buf_ptr);
        if buf[0] != 0x00 {
            uart::puts("in data!\n");
            show_hex(buf_ptr);
            buf = [0x00;700];
        }
        else{
            uart::puts("no packet\n");
        }

        
        if cnt<255{
            cnt+=1;
            show_dec(cnt);
        }
        timer::delay_time(200);
    }
    //enc28j60.enable_promiscuous(true);uart::puts("enable promiscuous\n");
    
    loop{
        //enc28j60.reveive_packet(buf_ptr);
        //uart::puts("\n ---- show packet ----\n");
        //sam(buf_ptr);
    }
}



#[no_mangle]
pub extern "C" fn led_test(){
    uart::puts("LED_ON\n");
    led::digitalWrite(1,0,0);
    timer::delay_time(1000);
    led::digitalWrite(0,1,0);
    timer::delay_time(1000);
    led::digitalWrite(0,0,1);
    timer::delay_time(1000);
    uart::puts("LED_OFF\n");
    led::digitalWrite(0,0,0);
    timer::delay_time(1000);
}

#[no_mangle]
pub extern "C" fn irq_handler(){
    uart::puts("timer irq\n");
    unsafe{
        if (*IRQ_BASIC).is_set(IRQ_BASIC::ARM_TIMER) {
            (*ARM_TIMER_CLI).set(0);
            uart::puts("irq_handler\n");
        }
    }
}


global_asm!(include_str!("boot.S"));

#[panic_handler] fn my_panic(_info: &core::panic::PanicInfo) -> ! {uart::puts("\nERROR\n"); loop {} }

#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0 () {}

#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr1 () {}



#[alloc_error_handler]
fn foo(_:Layout) -> !{
    loop{}
}