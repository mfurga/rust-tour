use std::{cell::{Cell, UnsafeCell}, ops::{Deref, DerefMut}};

#[derive(Debug)]
pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    references: Cell<i32>
}

#[derive(Debug)]
pub struct MyRef<'a, T> {
    refcell: &'a MyRefCell<T>
}

impl<'a, T> Deref for MyRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: there are only shared references
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'a, T> Drop for MyRef<'a, T> {
    fn drop(&mut self) {
        let rc = self.refcell.references.get();
        assert!(rc > 0);
        self.refcell.references.set(rc - 1);
    }
}

#[derive(Debug)]
pub struct MyRefMut<'a, T> {
    refcell: &'a MyRefCell<T>
}

impl<'a, T> Deref for MyRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: there is only one exclusive reference
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'a, T> DerefMut for MyRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: there is only one exclusive reference
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<'a, T> Drop for MyRefMut<'a, T> {
    fn drop(&mut self) {
        let rc = self.refcell.references.get();
        assert!(rc == -1);
        self.refcell.references.set(0);
    }
}

impl<T> MyRefCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            references: Cell::new(0)
        }
    }

    pub fn borrow(&self) -> Option<MyRef<'_, T>> {
        let rc = self.references.get();

        if rc < 0 {
            // you cannot have shared & exclusive refs at the same time
            return None
        }

        self.references.set(rc + 1);
        Some(MyRef { refcell: self })
    }

    pub fn borrow_mut(&self) -> Option<MyRefMut<'_, T>> {
        let rc = self.references.get();

        if rc > 0 {
            // you cannot have shared & exclusive refs at the same time
            return None
        }

        if rc < 0 {
            // you cannot have multiple exclusive refs at the same time
            return None
        }

        self.references.set(-1);
        Some(MyRefMut { refcell: self })
    }
}


#[cfg(test)]
mod tests {
    use std::cell::{RefCell, RefMut};
    use super::*;

    #[test]
    fn test1() {
        let rc = MyRefCell::new(0);

        let x1 = rc.borrow_mut();
        let x2 = rc.borrow_mut();
        let x3 = rc.borrow();

        assert!(x1.is_some());
        assert!(x2.is_none());
        assert!(x3.is_none());

        let mut x1 = x1.unwrap();
        *x1 = 42;
        assert_eq!(*x1, 42);
    }


    #[test]
    fn test2() {
        let rc = MyRefCell::new(42);

        let x1 = rc.borrow();
        let x2 = rc.borrow();
        let x3 = rc.borrow();

        assert!(x1.is_some());
        assert!(x2.is_some());
        assert!(x3.is_some());

        assert_eq!(*x1.unwrap(), 42);
        assert_eq!(*x2.unwrap(), 42);
        assert_eq!(*x3.unwrap(), 42);
    }

    #[test]
    fn test3() {
        let rc = MyRefCell::new(10);

        let x1 = rc.borrow();
        let x2 = rc.borrow();

        assert!(x1.is_some());
        assert!(x2.is_some());

        assert_eq!(*x1.unwrap(), 10);
        assert_eq!(*x2.unwrap(), 10);

        let x3 = rc.borrow_mut();
        assert!(x3.is_some());

        let mut x3 = x3.unwrap();
        *x3 = 20;
        *x3 += 10;

        assert_eq!(*x3, 30);

        let x4= rc.borrow();
        assert!(x4.is_none());

        drop(x3);

        let x5= rc.borrow();
        assert!(x5.is_some());

        assert_eq!(*x5.unwrap(), 30);
    }

}
