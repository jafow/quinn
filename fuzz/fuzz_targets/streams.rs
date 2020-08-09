#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

extern crate proto;
use proto::fuzzing::Streams;

use proto::Side;

#[derive(Arbitrary, Debug)]
struct StreamsParams {
    side: Side,
    max_remote_uni: u64,
    max_remote_bi: u64,
    send_window: u64,
    receive_window: u64,
    stream_receive_window: u64
}

fuzz_target!(|data: StreamsParams| {
    Streams::new(data.side, data.max_remote_uni, data.max_remote_bi, data.send_window, data.receive_window, data.stream_receive_window);
});
