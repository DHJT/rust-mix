// 基准测试：使用 criterion 或 bencher。
// 火焰图：使用 perf 或 flamegraph 库。

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_parse(c: &mut Criterion) {
    // let data = vec![0u8; 1024];
    // c.bench_function("parse_packet", |b| b.iter(|| parse_packet(&data)));
}

// fn parse_packet(buf: &mut BytesMut) -> Option<Packet> {
//     if buf.len() < 4 {
//         return None;
//     }
//
//     let len = {
//         let len_bytes = &buf[..4];
//         u32::from_be_bytes(len_bytes.try_into().unwrap()) as usize
//     };
//
//     if buf.len() < 4 + len {
//         return None;
//     }
//
//     buf.advance(4);
//     Some(buf.split_to(len))
// }

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);