use std::{cell::Cell, fmt::Debug, ops::Deref, ptr::NonNull};

pub struct MyRc<T> {
    inner: NonNull<MyInner<T>>,
    _marker: std::marker::PhantomData<MyInner<T>>,
}

impl<T: Debug> Debug for MyRc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyRc ({:?})", unsafe { self.inner.as_ref() } )
    }
}

#[derive(Debug)]
pub struct MyInner<T> {
    inner: T,
    references: Cell<usize>
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(MyInner {
            inner: value,
            references: Cell::new(1)
        });

        Self {
            // SAFETY: Box::into_raw never returns null
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: std::marker::PhantomData
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: we ensure that the pointer is valid as long as there is at least one MyRc<T> alive
        let inner = unsafe { self.inner.as_ref() };
        &inner.inner
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // SAFETY: we ensure that the pointer is valid as long as there is at least one MyRc<T> alive
        let inner = unsafe { self.inner.as_ref() };
        inner.references.set(inner.references.get() + 1);

        Self {
            inner: self.inner,
            _marker: std::marker::PhantomData
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        // SAFETY: we ensure that the pointer is valid as long as there is at least one MyRc<T> alive
        let inner = unsafe { self.inner.as_ref() };
        inner.references.set(inner.references.get() - 1);

        if inner.references.get() == 0 {
            // SAFETY: we ensure that the pointer is valid as long as there is at least one MyRc<T> alived
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rc = MyRc::new(42);
        let rc2 = rc.clone();

        println!("{:?}", rc);
        println!("{:?}", rc2);
    }
}
