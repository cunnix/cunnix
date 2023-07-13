#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(main);
fn main(_info: &'static mut bootloader_api::BootInfo) -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default.
    loop {}
}
