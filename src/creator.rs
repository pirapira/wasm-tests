#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{ext, logger};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };

    if let Ok(addr) = ext::create(ext::value(), input.as_ref()) {
        logger::debug("Created contractwith code");
        result.done(addr.to_vec());
    } else {
        logger::debug("Error creating contract");
    }
}
