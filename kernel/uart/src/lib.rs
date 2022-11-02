#![no_std]

/// uart interface
pub trait Uart {
	fn read(&self, buffer: &mut [u8]) -> usize;
	fn write(&self, data: &[u8]) -> usize;
	fn flush(&self);
	fn init(&self);
}