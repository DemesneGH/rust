use crate::ffi::OsString;
use crate::marker::PhantomData;
use crate::vec;
use crate::fmt;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
}

pub unsafe fn cleanup() {
}

pub fn args() -> Args {
    panic!("unsupported")
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.as_slice().fmt(f)
    }
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        self.iter.as_slice()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { self.iter.len() }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> { self.iter.next_back() }
}
