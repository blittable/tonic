#[macro_use]
extern crate criterion;

use criterion::*;
mod utils;

pub mod benchmarks {
    tonic::include_proto!("benchmarks.proto3");
}

use crate::benchmarks::{GoogleMessage1, GoogleMessage1SubMessage};

fn build_request(_name: String) {

    let sub_message = GoogleMessage1SubMessage {
        field1: 10,
        field2: 20,
        field3: 30,
        field15: String::from("clandestine"),
        field12: false,
        field13: 70,
        field14: 80,
        field16: 90,
        field19: 100,
        field20: true,
        field28: false,
        field21: 110,
        field22: 120,
        field23: false,
        field206: true,
        field203: 233,
        field204: 333,
        field205: String::from("idopathic"),
        field207: 4000,
        field300: 4000,
    }; 

 //   let _request = tonic::Request::new(GoogleMessage1 { field1: _name });
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("Request_Response_Diverse_Types");

    //log plot to get everything on the graph 
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let tiny_string = utils::generate_rnd_string(100).unwrap();
    let short_string = utils::generate_rnd_string(1_000).unwrap();
    let medium_string = utils::generate_rnd_string(10_000).unwrap();
    let big_string = utils::generate_rnd_string(100_000).unwrap();
    let huge_string = utils::generate_rnd_string(1_000_000).unwrap();

    for size in [tiny_string, short_string, medium_string].iter() {
        group.throughput(Throughput::Bytes(size.len() as u64));

        // group.bench_with_input(BenchmarkId::new("request", size.len()), size, |b, i| {
        //     b.iter(|| build_request(i.to_string()))
        // });
    }
    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
