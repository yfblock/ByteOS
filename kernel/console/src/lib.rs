#![no_std]
#![feature(used_with_arg)]

#[cfg(all(
    feature = "console_sbi", 
    feature = "console_uart"
))]
compile_error!("you should select just one method in console crate");

#[cfg(not(any(
    feature = "console_sbi", 
    feature = "console_uart"
)))]
compile_error!("you should select a output method in console crate");

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

#[cfg(feature = "console_uart")]
// 利用 mod 将整个代码包裹在 cfg 中
mod serial {
    use header::distributed_slice;
    use header::INIT_FUNC_PRIOR_0;
    use ns16550::Serial;
    use uart::Uart;
    use spin::lazy::Lazy;

    pub(crate) static SERIAL: Lazy<Serial> = Lazy::new(|| Serial::new(0x1000_0000));

    #[distributed_slice(INIT_FUNC_PRIOR_0)]
    fn init_uart() {
        SERIAL.init();
    }

    pub(crate) fn uart_write(data: &[u8]) {
        SERIAL.write(data);
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {

        let bytes = s.as_bytes();

        #[cfg(feature = "console_sbi")]
        bytes.iter().for_each(|show_char| console_putchar(*show_char));
        
        #[cfg(feature = "console_uart")]
        serial::uart_write(bytes);

        Ok(())
    }
}

/// 输出函数
/// 
/// 对参数进行输出 主要使用在输出相关的宏中 如println
#[inline]
pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}