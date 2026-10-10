use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountAllocations;
std::thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}
fn record_allocation() {
    let _ = ALLOCATIONS.try_with(|count| {
        if let Some(value) = count.get() {
            count.set(Some(value + 1));
        }
    });
}
// SAFETY: Every allocation operation is delegated with unchanged arguments to
// System. The counter uses allocation-free, thread-local Cell access.
unsafe impl GlobalAlloc for CountAllocations {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record_allocation();
        unsafe { System.realloc(ptr, layout, size) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}
#[global_allocator]
static ALLOCATOR: CountAllocations = CountAllocations;

pub(crate) fn allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ALLOCATIONS.with(|count| count.set(None));
        }
    }
    ALLOCATIONS.with(|count| assert_eq!(count.replace(Some(0)), None));
    let _reset = Reset;
    let result = operation();
    let count = ALLOCATIONS.with(|count| count.get().unwrap());
    (result, count)
}

// Host mocks execute in this process; exclude their own decoding and retained
// records when measuring allocations made by the SDK before/after the ABI call.
#[allow(dead_code)]
pub(crate) fn suspend() -> Suspended {
    Suspended(ALLOCATIONS.with(|count| count.replace(None)))
}
#[allow(dead_code)]
pub(crate) struct Suspended(Option<usize>);
impl Drop for Suspended {
    fn drop(&mut self) {
        ALLOCATIONS.with(|count| count.set(self.0));
    }
}
