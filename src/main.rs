#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn efi_main() -> Status {
    Status::SUCCESS
}

// Optional panic handler
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
