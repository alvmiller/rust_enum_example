#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
//#[repr(packed)] // Error
pub enum COMMAND {
#[allow(non_camel_case_types)]
    COMMAND_21,
#[allow(non_camel_case_types)]
    COMMAND_42,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMMAND1 {
#[allow(non_camel_case_types)]
    COMMAND_21(u8),
#[allow(non_camel_case_types)]
    COMMAND_42(u8),
}

impl COMMAND {
    pub fn get_val(&self) -> u8 {
        match self {
            COMMAND::COMMAND_21 => 0x21,
            COMMAND::COMMAND_42 => 0x42
        }
    }
}

pub fn print_command_value(value: COMMAND) {
    println!("CMD: {:?} | Val: {:#x}", value, value.get_val());
    return;
}

fn main() {
    println!("{}", std::mem::size_of::<COMMAND>()); // 1
    println!("{}", std::mem::size_of::<Option<COMMAND>>()); // 1
    println!("CMD21: {:?}", COMMAND::COMMAND_21); // COMMAND_21
    println!("CMD42: {:?}", COMMAND::COMMAND_42); // COMMAND_42
    println!("Val21: {:#x}", COMMAND::COMMAND_21.get_val()); // 0x21
    println!("Val42: {:#x}", COMMAND::COMMAND_42.get_val()); // 0x42
    print_command_value(COMMAND::COMMAND_21); // COMMAND_21 | 0x21
    print_command_value(COMMAND::COMMAND_42); // COMMAND_42 | 0x42
    println!("-------------");
    println!("{}", std::mem::size_of::<COMMAND1>()); // 2
    println!("{}", std::mem::size_of::<Option<COMMAND1>>()); // 2
}
