#![feature(lang_items, const_fn)]
#![feature(unique)]
#![no_std]
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate x86;

#[macro_use]
extern crate bitflags;
#[macro_use]
mod vga_buffer;
mod memory;
use memory::FrameAllocator;

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
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("ELF-sections tag required");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end   = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end   = multiboot_start + (boot_info.total_size as usize);
    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);
    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        multiboot_start, multiboot_end,
        memory_map_tag.memory_areas());
    enable_nxe_bit();
    enable_write_protect_bit();
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    frame_allocator.allocate_frame();
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
