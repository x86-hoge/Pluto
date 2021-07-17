use super::MMIO_BASE;
use super::gpio;
use register::mmio::*;
use core::ops;
use super::uart;

register_bitfields! {
    u32,

    /// Auxiliary enables
    AUX_ENABLES [
        SPI1_ENABLE OFFSET(1) NUMBITS(1) [],
        SPI2_ENABLE OFFSET(2) NUMBITS(1) []
    ],
    ///spi
    CS [
        CS   OFFSET(0)  NUMBITS(2) [
            CS0 = 0b00,
            CS1 = 0b01
        ],
        CLR  OFFSET(4)  NUMBITS(2) [
            NO = 0b00,
            TX = 0b01,
            RX = 0b10,
            ALL = 0b11
        ],
        TA   OFFSET(7)  NUMBITS(1)[
            ACTIVE = 1,
            INACTIVE = 0
        ],
        DONE OFFSET(16) NUMBITS(1)[],
        RXD  OFFSET(17) NUMBITS(1)[],
        TXD  OFFSET(18) NUMBITS(1)[],
        RXR  OFFSET(19) NUMBITS(1)[],
        RXF  OFFSET(20) NUMBITS(1)[]
    ]
}
const AUX_ENABLES:*const ReadWrite<u32, AUX_ENABLES::Register> = (MMIO_BASE + 0x21_5004) as *const ReadWrite<u32, AUX_ENABLES::Register>;
const SPI_BASE: u32 = MMIO_BASE + 0x20_4000;
pub const CS  :*const ReadWrite<u32, CS::Register> = (SPI_BASE + 0x00) as *const ReadWrite<u32, CS::Register>;//select main control 
pub const FIFO:*const ReadWrite<u32>               = (SPI_BASE + 0x04) as *const ReadWrite<u32>;//data read or write
pub const CLK :*const ReadWrite<u32>               = (SPI_BASE + 0x08) as *const ReadWrite<u32>;//clock set
pub const DLEN:*const ReadWrite<u32>               = (SPI_BASE + 0x0c) as *const ReadWrite<u32> ;//data length
pub const LTOH:*const ReadWrite<u32>               = (SPI_BASE + 0x10) as *const ReadWrite<u32> ;//output hold delay 
pub const DC  :*const ReadWrite<u32>               = (SPI_BASE + 0x14) as *const ReadWrite<u32> ;//dreq and panic signals (dma etc..)

    pub fn init(){
        unsafe{
            (*AUX_ENABLES).modify(AUX_ENABLES::SPI1_ENABLE::SET);
            (*gpio::GPFSEL0).modify(
                gpio::GPFSEL0::FSEL7::SPI0_CE1_IN +//alt0
                gpio::GPFSEL0::FSEL8::SPI0_CE0_IN +//alt0
                gpio::GPFSEL0::FSEL9::SPI0_MISO);  //alt0
            (*gpio::GPFSEL1).modify(
                gpio::GPFSEL1::FSEL10::SPI0_MISI + //alt0
                gpio::GPFSEL1::FSEL11::SPI0_SLCK); //alt0
            (*CLK).set(32); // 250MHz/16 = 15Mhz *enc28j60 max speed is 20Mhz
            (*CS).write(CS::CLR::ALL + CS::CS::CS0);//clear FIFO and set cs0
            (*DLEN).set(1);
        }
    }
    pub fn test(){
        enable_chip();
        
        write(65);
        let mut ret = read();
        uart::send( ret as char );
        write(65+1);
        ret = read();
        uart::send( ret as char );
        write(65+2);
        ret = read();
        uart::send( ret as char );
        uart::puts("\n");
        disable_chip();
    }
    pub fn select_chip(_cs:&u8){
        unsafe{
            match _cs{
                0 => (*CS).modify(CS::CS::CS0),
                1 => (*CS).modify(CS::CS::CS1),
                _ => (),
            }
        }
    }
    pub fn enable_chip(){
        unsafe{
            (*CS).modify(CS::CLR::ALL + CS::TA::ACTIVE);
        }
    }
    pub fn disable_chip(){
        unsafe{
            (*CS).modify(CS::CLR::NO + CS::TA::INACTIVE); //clear set
        }
    }
    pub fn write(_x:u8){
        unsafe {
            loop{ 
                if (*CS).is_set(CS::TXD){ //送信可能待ち
                    break; 
                }
                asm!("nop" :::: "volatile") ;
            }
            (*FIFO).set(_x as u32); 

            loop{
                if is_done() {
                    break;
                }
                asm!("nop" :::: "volatile") ;
            }
        }
    }
    pub fn read() -> u8{
        unsafe{
            loop{ 
                if (*CS).is_set(CS::RXD){ //１バイト受信待ち
                    break;
            }
                unsafe { asm!("nop" :::: "volatile") };
            }
            let ret = (*FIFO).get() as u8;
            ret //return RX fifo data
        }
    }
    pub fn is_done() -> bool{
        unsafe{
            (*CS).is_set(CS::DONE)
            }
    }