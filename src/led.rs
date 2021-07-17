use super::gpio;

pub fn init(){
    unsafe{
        (*gpio::GPFSEL0).modify(gpio::GPFSEL0::FSEL5::Output + gpio::GPFSEL0::FSEL6::Output);
        (*gpio::GPFSEL1).modify(gpio::GPFSEL1::FSEL13::Output);
        (*gpio::GPSET0).write(gpio::GPSET0::GP5::UP + gpio::GPSET0::GP6::UP + gpio::GPSET0::GP13::UP);//clear
    }
}

#[allow(non_snake_case)] 
pub fn digitalWrite(red:isize,green:isize,blue:isize){        
    unsafe{
        (*gpio::GPSET0).write(gpio::GPSET0::GP5::UP + gpio::GPSET0::GP6::UP + gpio::GPSET0::GP13::UP);//clear
        match (red,green,blue){
            (0,0,1) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP5::UP),
            (0,1,0) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP6::UP),
            (1,0,0) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP13::UP),
            (1,1,0) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP5::UP + gpio::GPCLR0::GP6::UP),
            (0,1,1) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP6::UP + gpio::GPCLR0::GP13::UP),
            (1,0,1) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP5::UP + gpio::GPCLR0::GP13::UP),
            (1,1,1) => (*gpio::GPCLR0).write(gpio::GPCLR0::GP5::UP + gpio::GPCLR0::GP6::UP + gpio::GPCLR0::GP13::UP),
            (_ ,_ , _) =>(),
        }
    }
}