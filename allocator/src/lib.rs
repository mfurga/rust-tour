use std::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};

const ALLOC_MEM_SIZE: usize = 4 * 1024 * 1024;

pub struct MyAlloc {
    mem: UnsafeCell<[u8; ALLOC_MEM_SIZE]>,
    ptr: AtomicUsize,
}

unsafe impl Sync for MyAlloc {}

#[global_allocator]
static MY_ALLOC: MyAlloc = MyAlloc {
    mem: UnsafeCell::new([0; ALLOC_MEM_SIZE]),
    ptr: AtomicUsize::new(0),
};

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let mut offset = 0;

        if self
            .ptr
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |cur| {
                offset = (cur + align - 1) & !(align - 1);
                if offset + size >= ALLOC_MEM_SIZE {
                    return None;
                }
                Some(offset + size)
            })
            .is_err()
        {
            return null_mut();
        }

        unsafe { self.mem.get().cast::<u8>().add(offset) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Do nothing...
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let _ = Box::new(42);
    }

    #[test]
    fn test2() {
        let _ = vec![0u8; 2 * 1024 * 1024].into_boxed_slice();
    }
}
