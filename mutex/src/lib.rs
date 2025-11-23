use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

struct MyMutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

struct MyMutexGuard<'a, T> {
    guard: &'a MyMutex<T>,
}

impl<T> Drop for MyMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.guard.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for MyMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: There can be only one MyMutexGuard at a time so there is only one
        //         reference to the data.
        unsafe { &*self.guard.data.get() }
    }
}

impl<T> DerefMut for MyMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: There can be only one MyMutexGuard at a time so there is only one
        //         reference to the data.
        unsafe { &mut *self.guard.data.get() }
    }
}

impl<T> MyMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> MyMutexGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) {}
        }
        MyMutexGuard { guard: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test1() {
        let m = Arc::new(MyMutex::new(10));
        let m2 = Arc::clone(&m);

        let h = std::thread::spawn(move || {
            *m2.lock() = 42;
        });

        h.join().unwrap();
        *m.lock() = 4242;
    }
}
