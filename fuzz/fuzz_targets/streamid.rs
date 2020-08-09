#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

extern crate proto;
use proto::{StreamId, Side, Dir};


#[derive(Arbitrary, Debug)]
struct StreamIdParams {
    side: Side,
    dir: Dir,
    initiator: u64
}

fuzz_target!(|data: StreamIdParams| {
    let s = StreamId::new(data.side, data.dir, data.initiator);
    assert_eq!(s.initiator(), data.side);
    assert_eq!(s.dir(), data.dir);
});
