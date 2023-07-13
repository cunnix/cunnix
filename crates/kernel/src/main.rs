#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(main);
fn main(info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x00;
        }
    }

    loop {}
}
