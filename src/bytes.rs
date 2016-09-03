use std::fmt;
use std::iter::repeat;
use std::ops::Deref;
use memsec::{ memcmp, mlock, munlock };


/// Temporary Bytes.
///
/// ```
/// use seckey::{ SecKey, Bytes };
///
/// let secpass = SecKey::new(&[8; 8]).unwrap();
/// let bytes = secpass.read_map(|b| Bytes::new(b));
///
/// assert_eq!(bytes, [8; 8]);
/// ```
#[derive(Clone)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    /// Create a new Bytes.
    #[inline]
    pub fn new(input: &[u8]) -> Bytes {
        let input: Vec<u8> = input.into();
        Bytes::from(input)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(mut t: Vec<u8>) -> Bytes {
        unsafe { mlock(t.as_mut_ptr(), t.len()) };
        Bytes(t)
    }
}

impl<'a> From<&'a [u8]> for Bytes {
    fn from(t: &'a [u8]) -> Bytes {
        Bytes::new(t)
    }
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", repeat('*').take(self.0.len()).collect::<String>())
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl<A: AsRef<[u8]>> PartialEq<A> for Bytes {
    fn eq(&self, rhs: &A) -> bool {
        self.eq(rhs.as_ref())
    }
}

impl PartialEq<[u8]> for Bytes {
    /// Constant time eq.
    fn eq(&self, rhs: &[u8]) -> bool {
        if self.0.len() == rhs.len() {
            unsafe { memcmp(self.0.as_ptr(), rhs.as_ptr(), self.0.len()) == 0 }
        } else {
            false
        }
    }
}

impl Eq for Bytes {}

impl Drop for Bytes {
    /// When drop, it will call `munlock`.
    fn drop(&mut self) {
        unsafe { munlock(self.0.as_mut_ptr(), self.0.len()) };
    }
}
