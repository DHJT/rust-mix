use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use algorithm::{bubble_sort, quick_sort};
use algorithm::merge_sort;
use rand::Rng;

// 生成随机数组
fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(-1000..1000)).collect()
}

// 测试不同算法在不同数据规模下的性能
fn bench_sorting_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    let sizes = [100, 1000, 10_000]; // 测试不同数据规模

    for size in sizes {
        // 测试 algorithm 的快速排序
        group.bench_with_input(
            BenchmarkId::new("Bubble Sort (algorithm)", size),
            &size,
            |b, &size| {
                let arr = generate_random_array(size);
                b.iter(|| {
                    let mut arr_clone = arr.clone();
                    bubble_sort(black_box(&mut arr_clone));
                })
            },
        );

        // 测试 algorithm 的快速排序
        group.bench_with_input(
            BenchmarkId::new("Quick Sort (algorithm)", size),
            &size,
            |b, &size| {
                let arr = generate_random_array(size);
                b.iter(|| {
                    let mut arr_clone = arr.clone();
                    quick_sort(black_box(&mut arr_clone));
                })
            },
        );

        // 测试 algorithm 的归并排序
        group.bench_with_input(
            BenchmarkId::new("Merge Sort (algorithm)", size),
            &size,
            |b, &size| {
                let arr = generate_random_array(size);
                b.iter(|| {
                    let mut arr_clone = arr.clone();
                    merge_sort(black_box(&mut arr_clone));
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(3));
    targets = bench_sorting_algorithms
);
criterion_main!(benches);