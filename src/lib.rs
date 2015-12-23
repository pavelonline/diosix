/* 
 * diosix microkernel 'menchi'
 *
 * Glue for portable Rust kernel code
 *
 * Maintainer: Chris Williams (diosix.org)
 *
 */

#![feature(no_std, lang_items, core_str_ext, const_fn, asm)]
#![no_std]

/* turn off some of the warnings that fire false positives */
#![allow(unused_assignments, unused_imports)]

/* needed for Box heap objects */
#![feature(box_syntax, box_patterns)]
#[lang = "owned_box"]
pub struct Box<T>(*mut T);

/* provides kprintln! and kprint! */
#[macro_use]
mod debug;

/* provides kalloc! and kfree! */
#[macro_use]
mod heap;

mod errors;

/* bare-metal libc, needed to provide various runtime
 * things like memcpy - see: https://crates.io/crates/rlibc */
extern crate rlibc;

/* bare-metal atomic operations because we can't use the std lib.
 * see: https://crates.io/crates/spin */
extern crate spin;

/* select appropriate platform-specific routines from build target's arch */
#[cfg(target_arch = "x86_64")]
#[path = "../platform/x86/src/mod.rs"] pub mod hardware;

/* entry point for our kernel */
#[no_mangle]
pub extern fn kmain()
{
    /* display boot banner */
    kprintln!("\ndiosix {} 'menchi' now running\n", env!("CARGO_PKG_VERSION"));

    /* initialize interrupts so we can catch exceptions at this early stage */
    hardware::interrupts::init().ok().expect("failed during interrupt init");

    /* initialize physical memory */
    hardware::physmem::init().ok().expect("failed during physical mem init");

    /* initialize hypervisor */
    hardware::hv::init().ok().expect("failed during hypervisor init");

    kprintln!("\nHalting kernel...");
}

/* handle panics by writing to the debug log and bailing out */
#[lang = "panic_fmt"]
extern fn panic_fmt(args: ::core::fmt::Arguments, file: &'static str, line: usize) -> !
{
    kprintln!("==> PANIC! {:?} in {}:{}", args, file, line);
    kprintln!("Halting.");
    loop{} /* end of the road */
}

#[lang = "eh_personality"] extern fn eh_personality() {} /* defined internally for panic()s but not needed */
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}

