#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io;
use bytes::{Bytes};

extern crate proto;
use proto::connection::{Streams};
use proto::frame::{Stream};
use proto::{Side, Dir, StreamId};


fuzz_target!(|data: &[u8]| {
    let mut client = Streams::new(Side::Client, 128, 128, 1024 * 1024, 1024 * 1024, 1024 * 1024);
    let id = StreamId::new(Side::Server, Dir::Uni, 0);
    let d = Bytes::copy_from_slice(data);
    client.received(Stream {
        id,
        offset: 0,
        fin: false,
        data: d
    });
});
