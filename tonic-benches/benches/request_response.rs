#[macro_use]
extern crate criterion;

use criterion::*;
mod utils;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use crate::hello_world::{HelloReply, HelloRequest};

fn build_request(_name: String) {
    let _request = tonic::Request::new(HelloRequest { name: _name });
}

fn build_response(_message: String) {
    let _response = tonic::Request::new(HelloReply { message: _message });
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("Request_Response");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let tiny_string = utils::generate_rnd_string(100).unwrap();
    let short_string = utils::generate_rnd_string(1_000).unwrap();
    let medium_string = utils::generate_rnd_string(10_000).unwrap();
    let big_string = utils::generate_rnd_string(100_000).unwrap();
    let huge_string = utils::generate_rnd_string(1_000_000).unwrap();
    let massive_string = utils::generate_rnd_string(10_000_000).unwrap();

    for size in [tiny_string, short_string, medium_string, big_string, huge_string, massive_string].iter() {
        group.throughput(Throughput::Bytes(size.len() as u64));

        group.bench_with_input(BenchmarkId::new("request", size.len()), size, |b, i| {
            b.iter(|| build_request(i.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("response", size.len()), size, |b, i| {
            b.iter(|| build_response(i.to_string()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
