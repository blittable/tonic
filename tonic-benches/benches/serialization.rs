#[macro_use]
extern crate criterion;

use criterion::*;
mod utils;


 pub mod hello_world {
     tonic::include_proto!("helloworld");
 }

use crate::hello_world::{HelloRequest, HelloReply};

fn build_request(_name: String) {
    let _request = tonic::Request::new(HelloRequest { name: _name });
}

fn build_response(_message: String) {
    let _response = tonic::Request::new(HelloReply { message: _message });
}

fn bench(c: &mut Criterion) {
    let tiny_string = &utils::generate_rnd_string(100).unwrap();
    let short_string = &utils::generate_rnd_string(1_000).unwrap();
    let medium_string = &utils::generate_rnd_string(10_000).unwrap();
    let big_string = &utils::generate_rnd_string(100_000).unwrap();
    let huge_string = &utils::generate_rnd_string(1_000_000).unwrap();

    let mut group = c.benchmark_group("Request-Response");

    // group.throughput(Throughput::Bytes(tiny_string.len() as u64));
    // group.bench_function("build_request string 100", |b| {
    //     b.iter(|| build_request(tiny_string.to_string()))
    // });

    // group.throughput(Throughput::Bytes(tiny_string.len() as u64));
    // group.bench_function("build_response string 100", |b| {
    //     b.iter(|| build_response(tiny_string.to_string()))
    // });

    // group.throughput(Throughput::Bytes(short_string.len() as u64));
    // group.bench_function("build_request string 1000", |b| {
    //     b.iter(|| build_response(short_string.to_string()))
    // });

    // group.throughput(Throughput::Bytes(short_string.len() as u64));
    // group.bench_function("build_response string 1000", |b| {
    //     b.iter(|| build_response(short_string.to_string()))
    // });

    group.throughput(Throughput::Bytes(medium_string.len() as u64));
    group.bench_function("build_request string 10000", |b| {
        b.iter(|| build_request(medium_string.to_string()))
    });
    group.throughput(Throughput::Bytes(medium_string.len() as u64));
    group.bench_function("build_response string 10000", |b| {
        b.iter(|| build_response(medium_string.to_string()))
    });

    group.throughput(Throughput::Bytes(big_string.len() as u64));
    group.bench_function("build_request string 100000", |b| {
        b.iter(|| build_request(big_string.to_string()))
    });
    group.throughput(Throughput::Bytes(big_string.len() as u64));
    group.bench_function("build_response string 100000", |b| {
        b.iter(|| build_response(big_string.to_string()))
    });

    group.throughput(Throughput::Bytes(huge_string.len() as u64));
    group.bench_function("build_request string 1000000", |b| {
        b.iter(|| build_request(huge_string.to_string()))
    });

    group.throughput(Throughput::Bytes(huge_string.len() as u64));
    group.bench_function("build_response string 1000000", |b| {
        b.iter(|| build_response(huge_string.to_string()))
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
