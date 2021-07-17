use super::uart;
const INPUT_MAXSIZE:usize = 64;

pub struct Shell;

impl Shell{
    pub fn new() -> Shell{
        Shell{}
    }
    pub fn init(&self){
        //uart::init();
        //uart::getc();
        uart::puts("\n\n");
        uart::puts("ROUTER VERSION 0.11 ,2019/12/01\n");
    }
    pub fn listen(&self){
        let mut data:[char;INPUT_MAXSIZE] = [' ';INPUT_MAXSIZE];
        uart::puts("ROUTER> ");
        let data_len:usize = self.getstr(&mut data[0] as *mut char);
        uart::puts("\n");
        self.cmd_test(&data[0] as *const char,&data_len);
    }
    pub fn print(&self,data:&str){
        uart::puts(&data);
    }
    pub fn send_char(&self,data:char){
        uart::send(data);
    }

    fn getstr(&self,p:*mut char) -> usize{
        let mut cnt:isize = 0;
        let mut temp:char = ' ';
        loop{    
            temp = uart::getc();
            unsafe{*p.offset(cnt) = temp;}
            if temp == '\r'{
                uart::send('\r');
                uart::send('\n');
                cnt -=1;
                break;
            }
            else if temp == '\x08'{
                if cnt > 0{
                    uart::puts("\x08 \x08");
                    cnt-=1;
                }
                continue;
            }
            //arrow keys
            else if temp == (0x41 as char) && 
                    temp == (0x42 as char) &&
                    temp == (0x43 as char) && 
                    temp == (0x44 as char){
                continue;
            }
            else{
                uart::send(temp);
            }
            
            if cnt as usize > INPUT_MAXSIZE{
                uart::puts(" \n");
                break;
            }
            else {
                cnt+=1;
            }
        }
        cnt as usize
    }
    /*fn parse(&self,p:*const char,len:&usize){
        let start_cmd:[&str;5] = ["set","ip","show","clear","data"];
        for i in 0..len{
            let mut cnt = 0;
            let mut isfound = true;
            let mut cmdID = 5;
            
            if unsafe{*p.offset(cnt)} == ' '{
                continue;
            }

            for j in start_cmd{
                for k in j.chars(){
                    if j != unsafe{*p.offset(cnt)} {
                        isfound = false;
                        break;
                    }
                    cnt+=1;
                }
            }
            if isfound{
                uart::puts("\nmatch command is ");
                uart::puts(i);
                uart::send('\r');
                uart::send('\n');
            }
        }
    } */
    
    fn cmd_test(&self,p:*const char,len:&usize){
        let start_cmd:[&str;5] = ["set","ip","show","clear","data"];
        for i in &start_cmd{
            let mut cnt = 0;
            let mut isfound = true;
            for j in i.chars(){
                loop{
                    if unsafe{*p.offset(cnt)} == ' '{
                        cnt+=1;
                    }
                    else{break}
                }
                
                if j != unsafe{*p.offset(cnt)} {
                    isfound = false;
                    break;
                }
                cnt+=1;
            }
            if isfound{
                uart::puts("\nmatch command is ");
                uart::puts(i);
                uart::send('\r');
                uart::send('\n');
            }
        }
    }
}

