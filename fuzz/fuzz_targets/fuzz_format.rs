#![no_main]
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary)]
pub struct FuzzInput {
    pub datetime: String,
    pub format: String,
}

fuzz_target!(|data: &[u8]| {
    use chrono::prelude::*;

    if let Ok(input) = Unstructured::new(data).arbitrary::<FuzzInput>() {
        let _ = DateTime::parse_from_str(&input.datetime, &input.format);
    }
});
