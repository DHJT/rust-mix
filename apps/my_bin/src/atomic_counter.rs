use std::sync::atomic::{AtomicUsize, Ordering};

// 线程安全的计数器实现
struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    fn new() -> Self {
        AtomicCounter {
            count: AtomicUsize::new(0),
        }
    }

    // 使用不同的内存排序语义进行递增
    fn increment_relaxed(&self) -> usize {
        // Relaxed 排序：没有同步保证
        self.count.fetch_add(1, Ordering::Relaxed)
    }

    fn increment_release(&self) -> usize {
        // Release 排序：所有之前的操作都变得可见
        self.count.fetch_add(1, Ordering::Release)
    }

    fn increment_seqcst(&self) -> usize {
        // 顺序一致性：最强的排序保证
        self.count.fetch_add(1, Ordering::SeqCst)
    }

    fn get(&self) -> usize {
        // Acquire 排序以与 Release 操作同步
        self.count.load(Ordering::Acquire)
    }
}