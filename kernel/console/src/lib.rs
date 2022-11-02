#![no_std]
#![feature(used_with_arg)]

use core::fmt::{Write, Arguments, Result};

/* 下面的代码跟输出相关，如果不需要输出则直接将相应的代码
 * 注释掉，如果需要输出，则取消注释。在实体代码被注释掉时
 * 相应的代码不会被有效编译，不占内存空间。
 */

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

struct Stdout;

/// 对`Stdout`实现输出的Trait

#[cfg(feature = "console_sbi")]
use sbi::console_putchar;
use uart::Uart;

#[cfg(feature = "console_uart")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "console_uart")]
mod serial {
    use header::distributed_slice;
    use header::INIT_FUNC_PRIOR_0;
    use ns16550::Serial;
    use uart::Uart;

    lazy_static! {
        pub(crate) static ref SERIAL: Serial = Serial::new(0x1000_0000);
    }

    #[distributed_slice(INIT_FUNC_PRIOR_0)]
    fn init_uart() {
        SERIAL.init();
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            #[cfg(feature = "console_sbi")]
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point);
            }
            #[cfg(feature = "console_uart")]
            serial::SERIAL.write(c.encode_utf8(&mut buffer).as_bytes());
        }
        Ok(())
    }
}

/// 输出函数
/// 
/// 对参数进行输出 主要使用在输出相关的宏中 如println
pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}