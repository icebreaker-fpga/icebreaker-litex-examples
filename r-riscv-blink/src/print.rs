use icebesoc_pac::UART;

pub struct Uart {
    pub registers: Option<UART>,
}

impl Uart {
    pub fn putc(&self, c: u8) {
        match self.registers.as_ref() {
            Some(reg) => unsafe {
                // Wait until TXFULL is `0`
                while reg.txfull.read().bits() != 0 {
                    ()
                }
                reg.rxtx.write(|w| w.bits(c));
            },
            None => ()
        }
    }
}

use core::fmt::{Error, Write};
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

#[macro_use]
#[cfg(not(test))]
pub mod print_hardware {
    use crate::print::*;
    pub static mut SUPERVISOR_UART: Uart = Uart {
        registers: None,
    };

    pub fn set_hardware(uart: UART) {
        unsafe {
            SUPERVISOR_UART.registers = Some(uart);
        }
    }

    #[macro_export]
    macro_rules! print
    {
        ($($args:tt)+) => ({
                use core::fmt::Write;
                unsafe {
                    let _ = write!(crate::print::print_hardware::SUPERVISOR_UART, $($args)+);
                }
        });
    }
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

