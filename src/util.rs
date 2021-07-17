use super::uart;

pub fn show_dec(data:u8){
    if data < 10{
        uart::send((0x30 | data) as char );
        uart::puts("\n");
    }
    else if data < 100{
        uart::send((0x30 | (data/10) ) as char);
        uart::send((0x30 | data-((data/10)*10)) as char);
        uart::puts("\n");
    }
    else if data < 255{
        uart::send((0x30 | (data/100)) as char);
        uart::send((0x30 |  (data/10) - ((data/100)*10) ) as char);
        uart::send((0x30 | data - ((data/10)*10)) as char);
        uart::puts("\n");
    }   
}

pub fn show_decs(data:*const u8){
    unsafe{
        for i in 0..700{
            for _ in 0..20{
                show_dec(*data.offset(i));
                uart::send(' ');
            }
            uart::puts("\n");
        }
        uart::puts("\n");
    }
}

pub fn show_hex(data:*const u8){
    unsafe{
        for i in 0..700{
            if *data.offset(i) < 16{
                if *data.offset(i) < 10{
                    uart::send((0x30 | *data.offset(i)) as char );
                }
                else{
                    uart::send((0x41 | *data.offset(i)-10) as char );
                }
            }
            if *data.offset(i) % 2 == 0{
                uart::puts(" ");
            }
        }
        uart::puts("\n");
    }
}