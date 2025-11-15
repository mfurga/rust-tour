use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct MyCell<T> {
    // Rust's aliasing rules prohibit casting a shared reference (&T) to
    // an exclusive reference (&mut T).
    // `UnsafeCell` is the only legal way to enable interior mutability, allowing mutation
    // through a shared reference.
    inner: UnsafeCell<T>
}

impl<T> MyCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: This is no shared or exclusive reference to the value inside the cell so we
        //         are not invalidating any references.
        // SAFETY: No one else is modifying the value inside the cell (because !Sync)
        unsafe { *self.inner.get() = value; }
    }

    pub fn get(&self) -> T where T: Copy {
        // SAFETY: No one else is modifying the value inside the cell (because !Sync)
        unsafe { *self.inner.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use crate::MyCell;

    #[test]
    fn test1() {
        let c = MyCell::new(10);
        assert_eq!(c.get(), 10);

        c.set(20);
        assert_eq!(c.get(), 20);
    }

    /*
    #[test]
    fn test2() {
        let c = Arc::new(MyCell::new(0));

        let x1 = c.clone();
        let t1 = thread::spawn(move || {
            for _ in 0 .. 100000 {
                let x = x1.get();
                x1.set(x + 1);
            }
        });

        let x2 = c.clone();
        let t2 = thread::spawn(move || {
            for _ in 0 .. 100000 {
                let x = x2.get();
                x2.set(x + 1);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(c.get(), 200000);
    }
    */
}
