#![no_main]
#![no_std]
#![allow(deprecated)]

#[macro_use] extern crate pwasm_std;

use pwasm_std::{ext, write_u32, logger};
use pwasm_std::hash::Address;
use core::hash::{SipHasher, Hasher};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (_, result) = unsafe { pwasm_std::parse_args(desc) };

    let addr = Address::from([13u8, 19, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    let input = [1u8, 2, 3, 5, 7, 11];
    let mut temp = vec![0u8; 256];
    match ext::call_code(&addr, &input, &mut temp[..]) {
        Ok(_) => {
            logger::debug("Call succeed");
        },
        Err(_) => {
            logger::debug("Call failed");
        }
    }

    let mut hasher = SipHasher::new_with_keys(0, 0);
    hasher.write(&temp[..]);
    let hash = (hasher.finish() & 0x00000000ffffffff) as u32;
    logger::debug("Hashing succed");

    let mut res = [0u8; 4];
    write_u32(&mut res[..], hash);

    logger::debug("Exiting...");

    result.done(res.to_vec());
}
