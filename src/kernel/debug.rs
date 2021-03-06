/* diosix debugging code
 *
 * By default we write all debug information out to the serial port
 *
 * (c) Chris Williams, 2018.
 *
 * See LICENSE for usage and copying.
 */

use core::fmt;

/* tell the compiler the platform-specific serial port code is elsewhere */
extern
{
  fn platform_serial_write_byte(byte: u8);
  pub fn platform_acquire_debug_spin_lock();
  pub fn platform_release_debug_spin_lock();
  pub fn platform_get_cpu_id() -> usize;
}

/* top level debug macros - harmless logging */
#[macro_export]
macro_rules! klog
{
  ($fmt:expr) => (kprintln!("[CPU {}] {}", $crate::debug::platform_get_cpu_id(), $fmt));
  ($fmt:expr, $($arg:tt)*) => (kprintln!(concat!("[CPU {}] ", $fmt), $crate::debug::platform_get_cpu_id(), $($arg)*));
}

/* bad news: bug detection, failures, etc */
#[macro_export]
macro_rules! kalert
{
  ($fmt:expr) => (kprintln!("[CPU {}] ALERT: {}", $crate::debug::platform_get_cpu_id(), $fmt));
  ($fmt:expr, $($arg:tt)*) => (kprintln!(concat!("[CPU {}] ALERT: ", $fmt), $crate::debug::platform_get_cpu_id(), $($arg)*));
}

/* low-level macros for kernel-only kprintln and kprint debug output routines */
macro_rules! kprintln
{
  ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! kprint
{
  ($($arg:tt)*) =>
  ({
    use core::fmt::Write;
    unsafe
    {
      $crate::debug::platform_acquire_debug_spin_lock();
      $crate::debug::SERIALPORT.write_fmt(format_args!($($arg)*)).unwrap();
      $crate::debug::platform_release_debug_spin_lock();
    }
  });
}

/* create a generic global serial port */
pub struct SerialWriter;
pub static mut SERIALPORT: SerialWriter = SerialWriter{};

impl fmt::Write for SerialWriter
{
  fn write_str(&mut self, s: &str) -> ::core::fmt::Result
  {
    serial_write_string(s);
    Ok(())
  }
}

/* write a string out to the platform's serial port */
pub fn serial_write_string(s: &str)
{
    for c in s.bytes()
    {
      unsafe { platform_serial_write_byte(c); }
    }
}
