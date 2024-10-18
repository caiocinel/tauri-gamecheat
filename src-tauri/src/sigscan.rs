// MIT License
//
// Copyright (c) 2018 frk <hazefrk+dev@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate serde_json;
use failure::Fail;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::memlib::Process;
use std::mem;

pub type Result<T> = ::std::result::Result<T, ScanError>;

#[derive(Debug, Fail)]
pub enum ScanError {
    #[fail(display = "Module not found")]
    ModuleNotFound,

    #[fail(display = "Pattern not found")]
    PatternNotFound,

    #[fail(display = "Offset out of module bounds")]
    OffsetOutOfBounds,

    #[fail(display = "rip_relative failed")]
    RIPRelativeFailed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    // Signature name.
    pub name: String,

    // Signature pattern.
    pub pattern: String,

    // Module name.
    pub module: String,

    // Signature offsets for dereferencing.
    #[serde(default)]
    pub offsets: Vec<isize>,

    // Extra to be added to the result.
    #[serde(default)]
    pub extra: isize,

    // If true, subtract module base from result.
    #[serde(default)]
    pub relative: bool,

    // If true, read a u32 at the position and add it to the result.
    #[serde(default)]
    pub rip_relative: bool,

    // Offset to the rip relative.
    #[serde(default)]
    pub rip_offset: isize,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            name: "".to_string(),
            pattern: "".to_string(),
            module: "".to_string(),
            offsets: vec![],
            extra: 0,
            relative: false,
            rip_relative: false,
            rip_offset: 0,
        }
    }
}

pub fn find_signature(sig: &Signature, process: &Process) -> Result<usize> {
    debug!("Begin scan: {}", sig.name);
    debug!("IsWow64: {:?}", process.is_wow64);
    debug!("Load module {}", sig.module);

    let module = process
        .get_module(&sig.module)
        .ok_or(ScanError::ModuleNotFound)?;
    debug!(
        "Module found: {} - Base: {:#X} Size: {:#X}",
        module.name, module.base, module.size
    );


    debug!("Searching pattern: {}", sig.pattern);
    let mut addr = module
        .find_pattern(&sig.pattern)
        .ok_or(ScanError::PatternNotFound)?;
    debug!(
        "Pattern found at: {:#X} (+ base = {:#X})",
        addr,
        addr + module.base
    );

    for (i, o) in sig.offsets.iter().enumerate() {
        debug!("Offset #{}: ptr: {:#X} offset: {:#X}", i, addr, o);

        let pos = (addr as isize).wrapping_add(*o) as usize;
        let data = module.data.get(pos).ok_or_else(|| {
            debug!("WARN OOB - ptr: {:#X} module size: {:#X}", pos, module.size);
            ScanError::OffsetOutOfBounds
        })?;

        let tmp = if process.is_wow64 {
            let raw: u32 = unsafe { std::ptr::read_unaligned(data as *const u8 as *const u32) };
            raw as usize
        } else {
            let raw: u64 = unsafe { std::ptr::read_unaligned(data as *const u8 as *const u64) };
            raw as usize
        };

        addr = tmp.wrapping_sub(module.base);
        debug!("Offset #{}: raw: {:#X} - base => {:#X}", i, tmp, addr);
    }

    if sig.rip_relative {
        debug!(
            "rip_relative: addr {:#X} + rip_offset {:#X}",
            addr, sig.rip_offset
        );
        addr = (addr as isize).wrapping_add(sig.rip_offset) as usize;
        debug!("rip_relative: addr = {:#X}", addr);

        let rip: u32 = module
            .get_raw(addr, true)
            .ok_or(ScanError::RIPRelativeFailed)?;

        debug!(
            "rip_relative: addr {:#X} + rip {:#X} + {:#X}",
            addr,
            rip,
            ::std::mem::size_of::<u32>()
        );
        addr = addr.wrapping_add(rip as usize + ::std::mem::size_of::<u32>());
        debug!("rip_relative: addr => {:#X}", addr);
    }

    debug!("Adding extra {:#X}", sig.extra);
    addr = (addr as isize).wrapping_add(sig.extra) as usize;
    if !sig.relative {
        debug!(
            "Not relative, addr {:#X} + base {:#X} => {:#X}",
            addr,
            module.base,
            addr.wrapping_add(module.base)
        );
        addr = addr.wrapping_add(module.base);
    }

    Ok(addr)
}
