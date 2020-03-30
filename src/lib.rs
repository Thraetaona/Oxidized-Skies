// Crate-level documentation (Overview) resides in the 'README.md' file.


#![no_std]
#![no_main]

#![allow(unused_attributes)]


use rasm::core::log::*;


#[no_mangle]
pub fn main() {
    let _arr = (42, "Exiting.");

    //rasm::console!(time);

    rasm::console!(log, "Hello, World!");
    rasm::console!(error, "Received : {}, {}", _arr.0, _arr.1);
    rasm::console!(warn, "this is a warning");
    rasm::console!(trace, "this message should be traced back to its original souce");
    rasm::console!(log, "Condition {} met, exiting now.", 404);
    rasm::console!(info, "1 + 3 = {}. It Works!", 1 + 3);


    //rasm::console!(time_end);
}