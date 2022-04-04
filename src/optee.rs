// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for OP-TEE using libutee [`optee_utee::Random`]
use crate::Error;

use optee_utee::Random;

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    Random::generate(dest);
    
    Ok(())
}