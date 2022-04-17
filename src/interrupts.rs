use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;


lazy_static!
{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() 
{
    IDT.load();
}

#[cfg(test)]
use crate::{serial_print, serial_println};
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    #[cfg(not(test))]
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    #[cfg(test)]
    serial_print!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_println... \n");
    x86_64::instructions::interrupts::int3();  
    serial_println!("[ok]");
}