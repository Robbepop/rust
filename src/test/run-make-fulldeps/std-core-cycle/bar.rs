// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(allocator_api)]
#![crate_type = "rlib"]

use std::heap::*;

pub struct A;

unsafe impl<'a> Alloc for &'a A {
    unsafe fn alloc(&mut self, _: Layout) -> Result<*mut u8, AllocErr> {
        loop {}
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _: Layout) {
        loop {}
    }
}
