#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

use alloc::vec::Vec;
use console::println;
use dev::MMIO_ARR;
use header::INIT_FUNC_PRIOR_3;
use header::distributed_slice;
use virtio_drivers::DeviceType;
use virtio_drivers::Transport;

#[derive(Debug)]
struct AtaError;

fn ata_read(start_sector: u64, sector_count: u8) -> Result<Vec<u8>, AtaError> {
    todo!()
}


#[derive(Debug)]
enum DiskCursorIoError {
    UnexpectedEof,
    WriteZero,
}
impl fatfs::IoError for DiskCursorIoError {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        Self::UnexpectedEof
    }

    fn new_write_zero_error() -> Self {
        Self::WriteZero
    }
}

struct DiskCursor {
    device_id: usize,
    sector: u64,
    offset: usize,
}

impl DiskCursor {
    fn get_position(&self) -> usize {
        (self.sector * 0x200) as usize + self.offset
    }

    fn set_position(&mut self, position: usize) {
        self.sector = (position / 0x200) as u64;
        self.offset = position % 0x200;
    }

    fn move_cursor(&mut self, amount: usize) {
        self.set_position(self.get_position() + amount)
    }
}

impl fatfs::IoBase for DiskCursor {
    type Error = DiskCursorIoError;
}

impl fatfs::Read for DiskCursor {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, DiskCursorIoError> {
        let mut i = 0;
        while i < buf.len() {
            let data: Vec<u8> =
                ata_read(self.sector, ((buf.len() - i) / 0x200).max(1).try_into().unwrap()).expect("ata error");
            let data = &data[self.offset..];
            if data.len() == 0 {
                break;
            }
            let end = (i + data.len()).min(buf.len());
            let len = end - i;
            buf[i..end].copy_from_slice(&data[..len]);
            i += len;
            self.move_cursor(i);
        }
        Ok(i)
    }
}

impl fatfs::Write for DiskCursor {
    fn write(&mut self, buf: &[u8]) -> Result<usize, DiskCursorIoError> {
        todo!("Write");
    }

    fn flush(&mut self) -> Result<(), DiskCursorIoError> {
        Ok(())
    }
}

impl fatfs::Seek for DiskCursor {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, DiskCursorIoError> {
        match pos {
            fatfs::SeekFrom::Start(i) => {
                self.set_position(i as usize);
                Ok(i)
            }
            fatfs::SeekFrom::End(i) => {
                todo!("Seek from end")
            }
            fatfs::SeekFrom::Current(i) => {
                let new_pos = (self.get_position() as i64) + i;
                self.set_position(new_pos as usize);
                Ok(new_pos as u64)
            }
        }
    }
}

#[distributed_slice(INIT_FUNC_PRIOR_3)]
pub fn init() {
    let c = DiskCursor {
        device_id: 0,
        sector: 0,
        offset: 0,
    };

    let fs = fatfs::FileSystem::new(c, fatfs::FsOptions::new()).expect("open fs");
    let cursor = fs.root_dir().open_dir("/hello").expect("move to dir");

    for entry in cursor.iter() {
        let entry = entry.expect("Entry");
        println!("{:?}", entry);
    }

    MMIO_ARR.wait().iter().for_each(|device| {
        if let DeviceType::Block = device.device_type() {
            println!("[kernel] BLOCK DEVICE: {:?}", device);
        }
    });
}