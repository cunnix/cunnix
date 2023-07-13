#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use bootloader_api::*;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;
use core::panic::PanicInfo;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
pub(crate) fn init_logger(framebuffer_info: &'static mut info::FrameBuffer) {
    let info = framebuffer_info.info();
    let framebuffer = framebuffer_info.buffer_mut();

    let logger = LOGGER.get_or_init(move || LockedLogger::new(framebuffer, info, true, false));
    log::set_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Blue Kernel!");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(main);

fn main(info: &'static mut BootInfo) -> ! {
    let framebuffer = info.framebuffer.as_mut().unwrap();

    init_logger(framebuffer);

    loop {}
}
