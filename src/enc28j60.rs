use super::timer::delay_time;
use super::spi;
use super::uart;
use super::util;

const ADDR_MASK:u8       = 0x1F;
const BANK_MASK:u8       = 0x60;
const SPRD_MASK:u8       = 0x80;
//ALL-BANK
const EIE:u8             = 0x1B;
const EIR:u8             = 0x1C;
const ESTAT:u8           = 0x1D;
const ECON2:u8           = 0x1E;
const ECON1:u8           = 0x1F;
// SPI operation codes
const READ_CTRL_REG:u8   = 0x00; //制御レジスタ読み込み
const READ_BUF_MEM:u8    = 0x3A; //バッファ読み出し
const WRITE_CTRL_REG:u8  = 0x40; //制御レジスタ書き込み
const WRITE_BUF_MEM:u8   = 0x7A; //バッファ書き込み
const BIT_FIELD_SET:u8   = 0x80; //ビットフィールドセット
const BIT_FIELD_CLR:u8   = 0xA0; //ビットフィールドクリア
const SOFT_RESET:u8      = 0xFF; //システムコマンド リセット
// Bank 0 registers
const BANK0_ERDPT:u8     = (0x00|0x00);
const BANK0_EWRPT:u8     = (0x02|0x00);
const BANK0_ETXST:u8     = (0x04|0x00);
const BANK0_ETXND:u8     = (0x06|0x00);
const BANK0_ERXST:u8     = (0x08|0x00);
const BANK0_ERXND:u8     = (0x0A|0x00);
const BANK0_ERXRDPT:u8   = (0x0C|0x00);
//const BANK0_ERXWRPT:u8 = (0x0E|0x00);
const BANK0_EDMAST:u8    = (0x10|0x00);
const BANK0_EDMAND:u8    = (0x12|0x00);
//const BANK0_EDMADST:u8 = (0x14|0x00);
const BANK0_EDMACS:u8    = (0x16|0x00);
// Bank 1 registers
const BANK1_EHT0:u8      = (0x00|0x20);
const BANK1_EHT1:u8      = (0x01|0x20);
const BANK1_EHT2:u8      = (0x02|0x20);
const BANK1_EHT3:u8      = (0x03|0x20);
const BANK1_EHT4:u8      = (0x04|0x20);
const BANK1_EHT5:u8      = (0x05|0x20);
const BANK1_EHT6:u8      = (0x06|0x20);
const BANK1_EHT7:u8      = (0x07|0x20);
const BANK1_EPMM0:u8     = (0x08|0x20);
const BANK1_EPMM1:u8     = (0x09|0x20);
const BANK1_EPMM2:u8     = (0x0A|0x20);
const BANK1_EPMM3:u8     = (0x0B|0x20);
const BANK1_EPMM4:u8     = (0x0C|0x20);
const BANK1_EPMM5:u8     = (0x0D|0x20);
const BANK1_EPMM6:u8     = (0x0E|0x20);
const BANK1_EPMM7:u8     = (0x0F|0x20);
const BANK1_EPMCS:u8     = (0x10|0x20);
//const BANK1_EPMO:u8    = (0x14|0x20);
const BANK1_EWOLIE:u8    = (0x16|0x20);
const BANK1_EWOLIR:u8    = (0x17|0x20);
const BANK1_ERXFCON:u8   = (0x18|0x20);
const BANK1_EPKTCNT:u8   = (0x19|0x20);
// Bank 2 registers
const BANK2_MACON1:u8    = (0x00|0x40|SPRD_MASK);
const BANK2_MACON3:u8    = (0x02|0x40|SPRD_MASK);
const BANK2_MACON4:u8    = (0x03|0x40|SPRD_MASK);
const BANK2_MABBIPG:u8   = (0x04|0x40|SPRD_MASK);
const BANK2_MAIPG:u8     = (0x06|0x40|SPRD_MASK);
const BANK2_MACLCON1:u8  = (0x08|0x40|SPRD_MASK);
const BANK2_MACLCON2:u8  = (0x09|0x40|SPRD_MASK);
const BANK2_MAMXFL:u8    = (0x0A|0x40|SPRD_MASK);
const BANK2_MAPHSUP:u8   = (0x0D|0x40|SPRD_MASK);
const BANK2_MICON:u8     = (0x11|0x40|SPRD_MASK);
const BANK2_MICMD:u8     = (0x12|0x40|SPRD_MASK);
const BANK2_MIREGADR:u8  = (0x14|0x40|SPRD_MASK);
const BANK2_MIWR:u8      = (0x16|0x40|SPRD_MASK);
const BANK2_MIRD:u8      = (0x18|0x40|SPRD_MASK);
// Bank 3 registers
const BANK3_MAADR1:u8    = (0x00|0x60|SPRD_MASK);
const BANK3_MAADR0:u8    = (0x01|0x60|SPRD_MASK);
const BANK3_MAADR3:u8    = (0x02|0x60|SPRD_MASK);
const BANK3_MAADR2:u8    = (0x03|0x60|SPRD_MASK);
const BANK3_MAADR5:u8    = (0x04|0x60|SPRD_MASK);
const BANK3_MAADR4:u8    = (0x05|0x60|SPRD_MASK);
const BANK3_EBSTSD:u8    = (0x06|0x60);
const BANK3_EBSTCON:u8   = (0x07|0x60);
const BANK3_EBSTCS:u8    = (0x08|0x60);
const BANK3_MISTAT:u8    = (0x0A|0x60|SPRD_MASK);
const BANK3_EREVID:u8    = (0x12|0x60);
const BANK3_ECOCON:u8    = (0x15|0x60);
const BANK3_EFLOCON:u8   = (0x17|0x60);
const BANK3_EPAUS:u8     = (0x18|0x60);    

// ENC28J60 ERXFCON Register Bit Definitions
const ERXFCON_UCEN:u8    = 0x80;
const ERXFCON_ANDOR:u8   = 0x40;
const ERXFCON_CRCEN:u8   = 0x20;
const ERXFCON_PMEN:u8    = 0x10;
const ERXFCON_MPEN:u8    = 0x08;
const ERXFCON_HTEN:u8    = 0x04;
const ERXFCON_MCEN:u8    = 0x02;
const ERXFCON_BCEN:u8    = 0x01;
// ENC28J60 EIE Register Bit Definitions
const EIE_INTIE:u8       = 0x80;
const EIE_PKTIE:u8       = 0x40;
const EIE_DMAIE:u8       = 0x20;
const EIE_LINKIE:u8      = 0x10;
const EIE_TXIE:u8        = 0x08;
const EIE_WOLIE:u8       = 0x04;
const EIE_TXERIE:u8      = 0x02;
const EIE_RXERIE:u8      = 0x01;
// ENC28J60 EIR Register Bit Definitions
const EIR_PKTIF:u8       = 0x40;
const EIR_DMAIF:u8       = 0x20;
const EIR_LINKIF:u8      = 0x10;
const EIR_TXIF:u8        = 0x08;
const EIR_WOLIF:u8       = 0x04;
const EIR_TXERIF:u8      = 0x02;
const EIR_RXERIF:u8      = 0x01;
// ENC28J60 ESTAT Register Bit Definitions
const ESTAT_INT:u8       = 0x80;
const ESTAT_LATECOL:u8   = 0x10;
const ESTAT_RXBUSY:u8    = 0x04;
const ESTAT_TXABRT:u8    = 0x02;
const ESTAT_CLKRDY:u8    = 0x01;
// ENC28J60 ECON2 Register Bit Definitions
const ECON2_AUTOINC:u8   = 0x80;
const ECON2_PKTDEC:u8    = 0x40;
const ECON2_PWRSV:u8     = 0x20;
const ECON2_VRPS:u8      = 0x08;
// ENC28J60 ECON1 Register Bit Definitions
const ECON1_TXRST:u8     = 0x80;
const ECON1_RXRST:u8     = 0x40;
const ECON1_DMAST:u8     = 0x20;
const ECON1_CSUMEN:u8    = 0x10;
const ECON1_TXRTS:u8     = 0x08;
const ECON1_RXEN:u8      = 0x04;
const ECON1_BSEL1:u8     = 0x02;
const ECON1_BSEL0:u8     = 0x01;
// ENC28J60 MACON1 Register Bit Definitions
const MACON1_LOOPBK:u8   = 0x10;
const MACON1_TXPAUS:u8   = 0x08;
const MACON1_RXPAUS:u8   = 0x04;
const MACON1_PASSALL:u8  = 0x02;
const MACON1_MARXEN:u8   = 0x01;
// ENC28J60 MACON3 Register Bit Definitions
const MACON3_PADCFG2:u8  = 0x80;
const MACON3_PADCFG1:u8  = 0x40;
const MACON3_PADCFG0:u8  = 0x20;
const MACON3_TXCRCEN:u8  = 0x10;
const MACON3_PHDRLEN:u8  = 0x08;
const MACON3_HFRMLEN:u8  = 0x04;
const MACON3_FRMLNEN:u8  = 0x02;
const MACON3_FULDPX:u8   = 0x01;
// ENC28J60 MICMD Register Bit Definitions
const MICMD_MIISCAN:u8   = 0x02;
const MICMD_MIIRD:u8     = 0x01;
// ENC28J60 MISTAT Register Bit Definitions
const MISTAT_NVALID:u8   = 0x04;
const MISTAT_SCAN:u8     = 0x02;
const MISTAT_BUSY:u8     = 0x01;
// ENC28J60 EBSTCON Register Bit Definitions
const EBSTCON_PSV2:u8    = 0x80;
const EBSTCON_PSV1:u8    = 0x40;
const EBSTCON_PSV0:u8    = 0x20;
const EBSTCON_PSEL:u8    = 0x10;
const EBSTCON_TMSEL1:u8  = 0x08;
const EBSTCON_TMSEL0:u8  = 0x04;
const EBSTCON_TME:u8     = 0x02;
const EBSTCON_BISTST:u8  = 0x01;
// PHY registers
const PHCON1:u8          = 0x00;
const PHSTAT1:u8         = 0x01;
const PHHID1:u8          = 0x02;
const PHHID2:u8          = 0x03;
const PHCON2:u8          = 0x10;
const PHSTAT2:u8         = 0x11;
const PHIE:u8            = 0x12;
const PHIR:u8            = 0x13;
const PHLCON:u8          = 0x14;
// ENC28J60 PHY PHCON1 Register Bit Definitions
const PHCON1_PRST:u16    = 0x8000;
const PHCON1_PLOOPBK:u16 = 0x4000;
const PHCON1_PPWRSV:u16  = 0x0800;
const PHCON1_PDPXMD:u16  = 0x0100;
// ENC28J60 PHY PHSTAT1 Register Bit Definitions
const PHSTAT1_PFDPX:u16  = 0x1000;
const PHSTAT1_PHDPX:u16  = 0x0800;
const PHSTAT1_LLSTAT:u16 = 0x0004;
const PHSTAT1_JBSTAT:u16 = 0x0002;
// ENC28J60 PHY PHCON2 Register Bit Definitions
const PHCON2_FRCLINK:u16 = 0x4000;
const PHCON2_TXDIS:u16   = 0x2000;
const PHCON2_JABBER:u16  = 0x0400;
const PHCON2_HDLDIS:u16  = 0x0100;
// ENC28J60 Packet Control Byte Bit Definitions
const PKTCTRL_PHUGEEN:u8 = 0x08;
const PKTCTRL_PPADEN:u8  = 0x04;
const PKTCTRL_PCRCEN:u8  = 0x02;
const PKTCTRL_POVERRIDE:u8 = 0x01;

//eth buffer 
const RXSTART_INIT:u16    = 0x0000; 
const RXSTOP_INIT:u16     = 0x0BFF; 
const TXSTART_INIT:u16    = 0x0000; 
const TXSTOP_INIT:u16     = 0x0BFF;

const MAX_CHIP_CONNECTS:usize = 2;//最大接続数
const MAX_FRAME_SIZE:u16  = 1500;

const debug:bool = false;


struct PacketHeader{
    //パケットの前にポインタ情報を含む6バイトヘッダの格納
        next_packet :u16,  //[ 0 - 15] 次のパケットのポインタ
        byte_count  :u16,  //[16 - 31] フレームの長さ
        status     :u16,   //[32 - 47] 14項目のステータス情報
    }
    
    impl PacketHeader{
        fn new() -> PacketHeader{
            PacketHeader{
                next_packet: 0x0000,
                byte_count : 0x0000,
                status     : 0x0000,
            }
        }
    }


pub struct ENC28J60{
    chip_max            : u8,
    chip                : u8,
    bank                : [u8;MAX_CHIP_CONNECTS],
    broadcast_enabled   : [bool;MAX_CHIP_CONNECTS],
    promimscuos_enabled : [bool;MAX_CHIP_CONNECTS],
    next_packet_ptr     : [u16;MAX_CHIP_CONNECTS],
    unreleased_packet   : [bool;MAX_CHIP_CONNECTS],
    buffersize          : [u16;MAX_CHIP_CONNECTS],
    //buffer              : [u8;MAX_FRAME_SIZE as usize],
}
impl ENC28J60{

    pub fn new() -> ENC28J60{
        ENC28J60{
            chip                : 0,
            chip_max            : MAX_CHIP_CONNECTS as u8,
            bank                : [0;MAX_CHIP_CONNECTS],
            broadcast_enabled   : [false;MAX_CHIP_CONNECTS],
            promimscuos_enabled : [false;MAX_CHIP_CONNECTS],
            next_packet_ptr     : [0x0000;MAX_CHIP_CONNECTS],
            unreleased_packet   : [false;MAX_CHIP_CONNECTS],
            buffersize          : [0x0000;MAX_CHIP_CONNECTS],
            //buffer              : [0x00;MAX_FRAME_SIZE as usize],
        }
    }

    fn change_chip(&mut self){
        let chip = self.chip + 1;
        if chip > self.chip_max{
            self.chip = 0; 
        }else{
            self.chip = chip;
        }
        spi::select_chip(&self.chip);
    }

    fn write_op(&self, op:u8, address:u8,data:u8){
        spi::enable_chip();
        spi::write( op | (address & ADDR_MASK) );
        spi::write(data);
        spi::disable_chip();
    }

    fn read_op(&self, op:u8, address:u8) -> u8{
        spi::enable_chip();
        spi::write(op | (address & ADDR_MASK));
        //spi::write(0x00);
        if (address & SPRD_MASK) == SPRD_MASK {
            if debug {uart::puts("dummy byte\n")}
            //ダミーバイト. データシート 27ページ参照
            spi::write(0x00);
        }
        if debug {uart::puts("read data\n")}
        let ret = spi::read() as u8;

        loop{
            if debug {
                if spi::is_done(){uart::puts("true\n")}
                else {uart::puts("false\n");}
            }
            if spi::is_done() {
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        
        spi::disable_chip();
        ret
    }

    fn read_buf(&self,len:u16, data:*mut u8){
        spi::enable_chip();
        let mut nextbyte:u8;
        unsafe{
        if len != 0{
            spi::write(READ_BUF_MEM);
                for i in 0..len+1{
                    nextbyte = spi::read();
                    *data.offset(i as isize) = nextbyte;
                    //if spi::is_done(){ break; }
                }
            }
        }
        spi::disable_chip();
    }

    fn read_packet_header(&self,header:*mut u16){
        spi::enable_chip();
        let mut nextbyte:u16;
        unsafe{
            spi::write(READ_BUF_MEM);
            for i in 0..3{
                nextbyte = (spi::read() as u16)  << 8;
                nextbyte |= spi::read() as u16;
                *header.offset(i) = nextbyte;
                //if spi::is_done(){ break; }
            }
        }
        spi::disable_chip();
    }

    fn write_buf(&self, len:isize, data:*mut u8){
        spi::enable_chip();
        unsafe{
        if len != 0{
            spi::write(WRITE_BUF_MEM);
                for i in 0..len{
                    spi::write( *data.offset(i) );
                    loop{
                        if spi::is_done(){ break; }
                        unsafe{asm!("nop" :::: "volatile") ;}
                    }
                }
            }
        }
        spi::disable_chip();
    }

    fn set_bank(&mut self,address:u8){
        let chip:usize = self.chip as usize;//要素番号のためusize型に変換
        let bank = address & BANK_MASK;
        if bank != self.bank[chip]{
            if debug { uart::puts("change BANK\n\n"); }
            self.write_op(BIT_FIELD_CLR, ECON1, ECON1_BSEL1|ECON1_BSEL0);
            self.bank[chip] = bank;
            self.write_op(BIT_FIELD_SET, ECON1, self.bank[chip] >> 5);
        }
    }

    fn read_reg_byte(&mut self,address:u8) -> u8{
        self.set_bank(address);
        if debug {uart::puts("start read_reg\n");}
        self.read_op(READ_CTRL_REG,address)
    }

    fn read_reg(&mut self,address:u8) -> u16{
        let mut ret:u16 = (self.read_reg_byte(address) as u16) << 8 ;
        if debug {uart::puts("end read high byte\n");}
        ret |= (self.read_reg_byte(address+1)) as u16;
        if debug {uart::puts("end read low byte\n");}
        ret
    }

    fn write_reg_byte(&mut self, address:u8, data:u8){
        self.set_bank(address);
        self.write_op(WRITE_CTRL_REG, address, data);
    }

    fn write_reg(&mut self, address:u8, data:u16){
        self.write_reg_byte(address, data as u8);
        self.write_reg_byte(address+1, (data >> 8) as u8);
    }

    fn read_phy_byte(&mut self, address:u8) -> u16{
        self.write_reg_byte(BANK2_MIREGADR, address);
        self.write_reg_byte(BANK2_MICMD, MICMD_MIIRD);
        loop{
            if (self.read_reg_byte(BANK3_MISTAT) & MISTAT_BUSY) != MISTAT_BUSY{
                break; 
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        self.write_reg_byte(BANK2_MICMD, 0x00);
        self.read_reg_byte(BANK2_MIRD + 1) as u16
    }

    fn write_phy(&mut self, address:u8, data:u16){
        self.write_reg_byte(BANK2_MIREGADR, address);
        if debug {uart::puts("end mireadg\n");}
        self.write_reg(BANK2_MIWR,data);
        if debug {uart::puts("end miwr\n");}
        loop{
            if debug {uart::send( (0x30 | (self.read_reg_byte(BANK3_MISTAT) & MISTAT_BUSY)) as char);uart::puts("\n");}
            if (self.read_reg_byte(BANK3_MISTAT) & MISTAT_BUSY) != MISTAT_BUSY
            {
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
    
    }
    pub fn begin(&mut self, mac_addr:&[u8;6]) -> u8{
        spi::disable_chip();
        if debug {
            uart::puts("start begin enc28j60\n");
        }
        self.write_op(SOFT_RESET, 0, SOFT_RESET);
        
        delay_time(10);
        if debug {
            uart::puts("end reset enc28j60\n");
        }
        
        loop{
            if ( (!self.read_op(READ_CTRL_REG,ESTAT))  & ESTAT_CLKRDY) == ESTAT_CLKRDY 
            {
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }

        if debug {
            uart::puts("clock rdy\n");
        }

        self.write_reg(BANK0_ERXST,RXSTART_INIT);
        if debug { uart::puts("clock rdy\n"); }
        self.write_reg(BANK0_ERXRDPT,RXSTART_INIT);
        if debug { uart::puts("end conf erxpdpt\n"); }
        self.write_reg(BANK0_ERXND,RXSTOP_INIT);
        if debug { uart::puts("end conf erxnd\n"); }
        self.write_reg(BANK0_ETXST,TXSTART_INIT);
        if debug { uart::puts("end conf etxst\n"); }
        self.write_reg(BANK0_ETXND,TXSTOP_INIT);
        if debug { uart::puts("end conf etxnd\n"); }
        self.write_phy(PHLCON,0x476);//led
        if debug { uart::puts("end conf led\n"); }
        self.write_reg_byte(BANK1_ERXFCON, ERXFCON_UCEN | ERXFCON_CRCEN | ERXFCON_PMEN | ERXFCON_BCEN);
        self.write_reg(BANK1_EPMM0,0x303F);
        self.write_reg(BANK1_EPMCS,0xF7F9);
        self.write_reg_byte(BANK2_MACON1,MACON1_MARXEN);
        if debug{ uart::puts("end harf begin \n"); }
        self.write_op(BIT_FIELD_SET,BANK2_MACON3,MACON3_PADCFG0 | MACON3_TXCRCEN | MACON3_FRMLNEN);
        self.write_reg(BANK2_MAIPG,0x0C12);
        self.write_reg_byte(BANK2_MABBIPG,0x12);
        self.write_reg(BANK2_MAMXFL,MAX_FRAME_SIZE);
        self.write_reg_byte(BANK3_MAADR5,mac_addr[0]);
        self.write_reg_byte(BANK3_MAADR4,mac_addr[1]);
        self.write_reg_byte(BANK3_MAADR3,mac_addr[2]);
        self.write_reg_byte(BANK3_MAADR2,mac_addr[3]);
        self.write_reg_byte(BANK3_MAADR1,mac_addr[4]);
        self.write_reg_byte(BANK3_MAADR0,mac_addr[5]);        
        self.write_phy(PHCON2,PHCON2_HDLDIS);
        self.set_bank(ECON1);
        self.write_op(BIT_FIELD_SET,EIE,EIE_INTIE | EIE_PKTIE);
        self.write_op(BIT_FIELD_SET,ECON1,ECON1_RXEN);
        //self.write_phy(PHCON2,PHCON2_FRCLINK);
        let mut rev:u8 = self.read_reg_byte(BANK3_EREVID);
        uart::puts("revision : ");
        util::show_dec(rev);
        if debug{
            uart::puts("end all begin \n");
        }
        if rev > 5{
            rev +=1;
        }
        rev
    }

    pub fn is_linkup(&mut self) -> bool{
        uart::puts("linkup...\n");
        (self.read_phy_byte(PHSTAT2) >> 2 & 0x01) == 0x01 
    }
    
    pub fn send_packet(&mut self,len:isize,buffer:*mut u8){
        self.write_op(BIT_FIELD_SET, ECON1, ECON1_TXRST);
        self.write_op(BIT_FIELD_CLR, ECON1, ECON1_TXRST);
        self.write_op(BIT_FIELD_CLR, EIR, EIR_TXERIF | EIR_TXIF);
        self.write_reg(BANK0_EWRPT, TXSTART_INIT);
        self.write_reg(BANK0_ETXND, len as u16);
        self.write_op(WRITE_BUF_MEM, 0, 0x00);
        self.write_buf(len,buffer);
        self.write_op(BIT_FIELD_SET, ECON1, ECON1_TXRTS);
        //遅延対処
        let mut count:u16 = 0;
        loop{
            util::show_dec(self.read_reg_byte(EIR));
            if (self.read_reg_byte(EIR) & (EIR_TXIF | EIR_TXERIF)) == (EIR_TXIF | EIR_TXERIF)
            { 
                break; 
            }
            count+=1;
            if count > 1000{ break; }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        if !((self.read_reg_byte(EIR) & EIR_TXERIF) == EIR_TXERIF) && count < 1000
        {
            return; 
        }
        self.write_op(BIT_FIELD_CLR, ECON1, ECON1_TXRTS);
    }
    
    pub fn reveive_packet(&mut self ,buffer_ptr:*mut u8){
        
        if self.unreleased_packet[self.chip as usize]{
            if self.next_packet_ptr[self.chip as usize] == 0{
                self.write_reg(BANK0_ERXRDPT, RXSTOP_INIT);
            }
            else{
                self.write_reg(BANK0_ERXRDPT, self.next_packet_ptr[self.chip as usize] - 1);
            }
            self.unreleased_packet[self.chip as usize] = false;
        }
        if self.read_reg(BANK1_EPKTCNT) > 0{
            self.write_reg(BANK0_ERDPT, self.next_packet_ptr[self.chip as usize]);
            
            let mut packet_header:PacketHeader = PacketHeader::new();
        
            let packet_header_ptr = &mut packet_header.next_packet as *mut u16;
            self.read_packet_header(packet_header_ptr);

            self.next_packet_ptr[self.chip as usize] = packet_header.next_packet;
            let mut len:u16 = packet_header.byte_count - 4;
            if len > self.buffersize[self.chip as usize] - 1{
                len = self.buffersize[self.chip as usize] - 1;
            }
            if (packet_header.status & 0x80) == 0{
                uart::puts("no packet packet\n");
                len = 0;
            }
            else{
                uart::puts("receive packet...\n");
                self.read_buf(len,buffer_ptr);
            }
        }
    }
    
    //pub fn copy_out(&self){}
    
    //pub fn copy_in(&self){}
    
    //pub fn peek_in(&self){}
    
    pub fn power_down(&mut self){
        self.write_op(BIT_FIELD_CLR,ECON1,ECON1_RXEN);
        loop{
            if(self.read_reg_byte(ESTAT) & ESTAT_RXBUSY) != ESTAT_RXBUSY{
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        loop{
            if(self.read_reg_byte(ECON1) & ECON1_TXRTS) != ECON1_TXRTS{
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        self.write_op(BIT_FIELD_SET, ECON2, ECON2_VRPS);
        self.write_op(BIT_FIELD_SET, ECON2, ECON2_PWRSV);
    }
    
    pub fn power_on(&mut self){
        self.write_op(BIT_FIELD_CLR,ECON2,ECON2_PWRSV);
        loop{
            if (self.read_reg_byte(ESTAT) & ESTAT_CLKRDY) != ESTAT_CLKRDY{
                break;
            }
            unsafe{asm!("nop" :::: "volatile") ;}
        }
        self.write_op(BIT_FIELD_SET,ECON1,ECON1_RXEN);
    }
    
    pub fn enable_broadcast(&mut self, temp:bool){
        let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
        self.write_reg_byte(BANK1_ERXFCON, erxfcon | ERXFCON_BCEN);
        if !temp{
            self.broadcast_enabled[self.chip as usize] = true;
        }
    }
    
    pub fn disable_broadcast(&mut self, temp:bool){
        if !temp{
            self.broadcast_enabled[self.chip as usize] = false;
        }
        if !self.broadcast_enabled[self.chip as usize]{
            let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
            self.write_reg_byte(BANK1_ERXFCON, erxfcon & !ERXFCON_BCEN);
        }
    }
    
    pub fn enable_multicast(&mut self){
        let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
        self.write_reg_byte(BANK1_ERXFCON, erxfcon | ERXFCON_MCEN);
    }
    
    pub fn disable_multicast(&mut self){
        let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
        self.write_reg_byte(BANK1_ERXFCON, erxfcon & !ERXFCON_MCEN);
    }
    
    pub fn enable_promiscuous(&mut self, temp:bool){
        let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
        self.write_reg_byte(BANK1_ERXFCON, erxfcon & ERXFCON_CRCEN);
        if temp{
            self.promimscuos_enabled[self.chip as usize] = true;
        }
    }
    
    pub fn disable_promiscuous(&mut self, temp:bool){
        if !temp{
            self.promimscuos_enabled[self.chip as usize] = false;
        }
        if self.promimscuos_enabled[self.chip as usize] {
        let erxfcon = self.read_reg_byte(BANK1_ERXFCON);
        self.write_reg_byte(BANK1_ERXFCON, erxfcon & !ERXFCON_BCEN);
    }
    }
    
    //pub fn doBIST(&self){}//検査用
    //pub fn memcpy_to_enc(&self)
    //pub fn memcpy_from_enc(&self)
    //pub fn enc_malloc(&self)
    //pub fn fnc_freemem(&self)
    //pub fn readPacketslice(&self){}
    
}
