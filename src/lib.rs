#![feature(lang_items, const_fn)]
#![feature(unique)]
#![feature(alloc, collections)]
#![no_std]
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate x86;
extern crate hole_list_allocator;
#[macro_use]
extern crate bitflags;
extern crate alloc;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate once;

#[macro_use]
mod vga_buffer;
mod memory;

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};

    let wp_bit = 1 << 16;
    unsafe {cr0_write(cr0() | wp_bit)};
}

fn stack_overflow() {
    let x = [0; 99999];
}


#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize){
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    let boot_info = unsafe {multiboot2::load(multiboot_information_address)};
    enable_nxe_bit();
    enable_write_protect_bit();
    memory::init(boot_info);
    use memory::FrameAllocator;
    use alloc::boxed::Box;
    let heap_test = Box::new(42);
    println!("It did not crash!");
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality(){}
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("      {}", fmt);
    loop{}
}

#[no_mangle]
pub fn _Unwind_Resume(){}
