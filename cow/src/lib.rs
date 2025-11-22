use std::{borrow::{Borrow}, ops::{Deref}};

pub enum MyCow<'a, T> where T: ToOwned + ?Sized {
    Borrowed(&'a T),
    Owned(<T as ToOwned>::Owned)
}

impl<T: ToOwned + ?Sized> MyCow<'_, T> {
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            MyCow::Borrowed(_) => true,
            MyCow::Owned(_) => false
        }
    }

    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    fn make_owned(&mut self) {
        match self {
            MyCow::Owned(x) => (),
            MyCow::Borrowed(x) => *self = MyCow::Owned(x.to_owned())
        }
    }

    pub fn into_owned(self) -> <T as ToOwned>::Owned {
        match self {
            MyCow::Owned(x) => x,
            MyCow::Borrowed(x) => x.to_owned()
        }
    }

    pub fn to_mut(&mut self) -> &mut <T as ToOwned>::Owned {
        self.make_owned();

        match self {
            MyCow::Owned(x) => x,
            MyCow::Borrowed(x) => unreachable!()
        }
    }
}

impl<T> Deref for MyCow<'_, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Borrow<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MyCow::Borrowed(x) => *x,
            MyCow::Owned(x) => x.borrow()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = MyCow::Borrowed("foo");
        assert!(s.is_borrowed());
        assert!(!s.is_owned());
    }

    #[test]
    fn test2() {
        let s = MyCow::Borrowed("bar");
        assert!(s.into_owned() == "bar".to_string());
    }

    #[test]
    fn test3() {
        let mut s = MyCow::Borrowed("bar");

        assert!(s.is_borrowed());

        let rs = s.to_mut();
        assert!(rs == &"bar".to_string());

        assert!(s.is_owned());
    }
}
