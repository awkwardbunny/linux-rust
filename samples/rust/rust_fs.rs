// SPDX-License-Identifier: GPL-2.0

//! Rust file system sample.

use kernel::prelude::*;
use kernel::{c_str, fs};

module! {
    type: FsModule,
    name: b"rust_fs",
    author: b"Rust for Linux Contributors",
    license: b"GPL",
}

struct RustFs;

#[vtable]
impl fs::Context<Self> for RustFs {
    type Data = ();

    kernel::define_fs_params! {(),
        {flag, "flag", |_, v| { pr_info!("flag passed-in: {v}\n"); Ok(()) } },
        {flag_no, "flagno", |_, v| { pr_info!("flagno passed-in: {v}\n"); Ok(()) } },
        {bool, "bool", |_, v| { pr_info!("bool passed-in: {v}\n"); Ok(()) } },
        {u32, "u32", |_, v| { pr_info!("u32 passed-in: {v}\n"); Ok(()) } },
        {u32oct, "u32oct", |_, v| { pr_info!("u32oct passed-in: {v}\n"); Ok(()) } },
        {u32hex, "u32hex", |_, v| { pr_info!("u32hex passed-in: {v}\n"); Ok(()) } },
        {s32, "s32", |_, v| { pr_info!("s32 passed-in: {v}\n"); Ok(()) } },
        {u64, "u64", |_, v| { pr_info!("u64 passed-in: {v}\n"); Ok(()) } },
        {string, "string", |_, v| { pr_info!("string passed-in: {v}\n"); Ok(()) } },
        {enum, "enum", [("first", 10), ("second", 20)], |_, v| {
            pr_info!("enum passed-in: {v}\n"); Ok(()) }
        },
    }

    fn try_new() -> Result {
        pr_info!("context created!\n");
        Ok(())
    }
}

impl fs::Type for RustFs {
    type Context = Self;
    const NAME: &'static CStr = c_str!("rustfs");
    const FLAGS: i32 = fs::flags::USERNS_MOUNT;
    const MAGIC: u32 = 0x72757374;
}

struct FsModule {
    _fs: Pin<Box<fs::Registration>>,
}

impl kernel::Module for FsModule {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let mut reg = Pin::from(Box::try_new(fs::Registration::new())?);
        reg.as_mut().register::<RustFs>(module)?;
        Ok(Self { _fs: reg })
    }
}
