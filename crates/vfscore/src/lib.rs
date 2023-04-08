#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

bitflags::bitflags! {
    pub struct OpenFlags: usize {
        // reserve 3 bits for the access mode
        const O_ACCMODE =  0x0007;
        const O_EXEC    =  1;
        const O_RDONLY  =  2;
        const O_RDWR    =  3;
        const O_SEARCH  =  4;
        const O_WRONLY  =  5;

        // these flags get their own bit
        const O_APPEND    = 0x000008;
        const O_CREAT     = 0x000010;
        const O_DIRECTORY = 0x000020;
        const O_EXCL      = 0x000040;
        const O_NOCTTY    = 0x000080;
        const O_NOFOLLOW  = 0x000100;
        const O_TRUNC     = 0x000200;
        const O_NONBLOCK  = 0x000400;
        const O_DSYNC     = 0x000800;
        const O_RSYNC     = 0x001000;
        const O_SYNC      = 0x002000;
        const O_CLOEXEC   = 0x004000;
        const O_PATH      = 0x008000;
        const O_LARGEFILE = 0x010000;
        const O_NOATIME   = 0x020000;
        const O_ASYNC     = 0x040000;
        const O_TMPFILE   = 0x080000;
        const O_DIRECT    = 0x100000;
    }
}

bitflags::bitflags! {
    pub struct MMapFlags: usize {
        const MAP_PRIVATE = 0x1;
        const MAP_SHARED = 0x2;
        const MAP_FIXED = 0x4;
        const MAP_ANONYOMUS = 0x8;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VfsError {
    NotLinkFile,
    NotDir,
    NotFile,
    NotSupported,
    FileNotFound,
    AlreadyExists,
    InvalidData,
    DirectoryNotEmpty,
    InvalidInput,
    StorageFull,
    UnexpectedEof,
    WriteZero,
    Io,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Device,
    Socket,
    Link,
}

#[derive(Debug, Copy, Clone)]
pub enum SeekFrom {
    SET(usize),
    CURRENT(isize),
    END(isize),
}

#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    pub inode: usize,
    pub file_type: FileType,
    pub size: usize,
    pub childrens: usize,
}

pub struct DirEntry {
    pub filename: String,
    pub len: usize,
    pub file_type: FileType,
}

pub trait FileSystem: Send + Sync {
    fn root_dir(&'static self) -> Arc<dyn INodeInterface>;
    fn name(&self) -> &str;
}

pub type VfsResult<T> = core::result::Result<T, VfsError>;

pub trait INodeInterface: Send + Sync {
    fn metadata(&mut self) -> VfsResult<Metadata> {
        Err(VfsError::NotSupported)
    }

    fn read(&mut self, _buffer: &mut [u8]) -> VfsResult<usize> {
        Err(VfsError::NotFile)
    }

    fn write(&mut self, _buffer: &[u8]) -> VfsResult<usize> {
        Err(VfsError::NotFile)
    }

    fn seek(&self, _seek: SeekFrom) -> VfsResult<usize> {
        Err(VfsError::NotFile)
    }

    fn mkdir(&self, _name: &str) -> VfsResult<Arc<dyn INodeInterface>> {
        Err(VfsError::NotDir)
    }

    fn rmdir(&self, _name: &str) -> VfsResult<()> {
        Err(VfsError::NotDir)
    }

    fn remove(&self, _name: &str) -> VfsResult<()> {
        Err(VfsError::NotDir)
    }

    fn touch(&self, _name: &str) -> VfsResult<Arc<dyn INodeInterface>> {
        Err(VfsError::NotDir)
    }

    fn read_dir(&self) -> VfsResult<Vec<DirEntry>> {
        Err(VfsError::NotDir)
    }

    fn lookup(&self, _name: &str) -> VfsResult<Arc<dyn INodeInterface>> {
        Err(VfsError::NotDir)
    }

    fn open(&self, _name: &str, _flags: OpenFlags) -> VfsResult<Arc<dyn INodeInterface>> {
        Err(VfsError::NotDir)
    }

    fn weak_filesystem(&self) -> VfsResult<Weak<dyn FileSystem>> {
        Err(VfsError::NotSupported)
    }

    fn ioctl(&self, _command: usize, _arg: usize) -> VfsResult<usize> {
        Err(VfsError::NotSupported)
    }

    fn truncate(&mut self, _size: usize) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    fn fcntl(&self, _cmd: usize, _arg: usize) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    fn flush(&mut self) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    fn resolve_link(&self) -> VfsResult<String> {
        Err(VfsError::NotSupported)
    }

    fn link(&self, _name: &str, _src: Arc<dyn INodeInterface>) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    fn unlink(&self, _name: &str) -> VfsResult<()> {
        Err(VfsError::NotSupported)
    }

    fn mmap(&self, _offset: usize, _size: usize, _flags: MMapFlags) -> VfsResult<usize> {
        Err(VfsError::NotSupported)
    }
}

/*
// Linux Vfs operations
struct file_operations {
    struct module *owner;
    loff_t (*llseek) (struct file *, loff_t, int);
    ssize_t (*read) (struct file *, char *, size_t, loff_t *);
    ssize_t (*write) (struct file *, const char *, size_t, loff_t *);
    int (*readdir) (struct file *, void *, filldir_t);
    unsigned int (*poll) (struct file *, struct poll_table_struct *);
    int (*ioctl) (struct inode *, struct file *, unsigned int, unsigned long);
    int (*mmap) (struct file *, struct vm_area_struct *);
    int (*open) (struct inode *, struct file *);
    int (*flush) (struct file *);
    int (*release) (struct inode *, struct file *);
    int (*fsync) (struct file *, struct dentry *, int datasync);
    int (*fasync) (int, struct file *, int);
    int (*lock) (struct file *, int, struct file_lock *);
    ssize_t (*readv) (struct file *, const struct iovec *, unsigned long, loff_t *);
    ssize_t (*writev) (struct file *, const struct iovec *, unsigned long, loff_t *);
};

*/