#![allow(dead_code)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum COMMAND {
#[allow(non_camel_case_types)]
    COMMAND_21,
#[allow(non_camel_case_types)]
    COMMAND_42,
}

impl COMMAND {
    pub fn get_val(&self) -> u32 {
        match self {
            COMMAND::COMMAND_21 => 0x21,
            COMMAND::COMMAND_42 => 0x42
        }
    }
}

pub fn print_command_value(value: COMMAND) {
    println!("Val: {:#x}", value.get_val());
    return;
}

fn main() {
    println!("CMD21: {:?}", COMMAND::COMMAND_21); // COMMAND_21
    println!("CMD42: {:?}", COMMAND::COMMAND_42); // COMMAND_42
    println!("Val21: {:#x}", COMMAND::COMMAND_21.get_val()); // 0x21
    println!("Val42: {:#x}", COMMAND::COMMAND_42.get_val()); // 0x42
    print_command_value(COMMAND::COMMAND_21); // 0x21
    print_command_value(COMMAND::COMMAND_42); // 0x42
}
