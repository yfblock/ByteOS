#![no_std]

use core::cell::RefCell;

use uart_16550::MmioSerialPort;
use uart::Uart;

pub struct Serial {
    serial: RefCell<MmioSerialPort>
}

unsafe impl Sync for Serial {}

impl Serial {
    pub fn new(addr: usize) -> Self {
        Self {
            serial: RefCell::new(unsafe { MmioSerialPort::new(addr) })
        }
    }
}

impl Uart for Serial {
    fn read(&self, buffer: &mut [u8]) -> usize {
        if buffer.len() == 0 { return 0; }

        let char = self.serial.borrow_mut().receive();
        buffer[0] = char;
        1
    }

    fn write(&self, data: &[u8]) -> usize {
        let mut serial = self.serial.borrow_mut();
        for i in data {
            serial.send(*i);
        }
        data.len()
    }

    fn flush(&self) {
        
    }

    fn init(&self) {
        self.serial.borrow_mut().init();
    }
}