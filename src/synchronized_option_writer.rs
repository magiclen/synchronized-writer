use std::{
    fmt,
    io::{self, ErrorKind, IoSlice, Write},
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

/// A writer which writes data into a reference counted, mutex-protected writer that can be taken out.
///
/// Every write operation locks the inner mutex, so it waits until the lock is free and fails if the lock is poisoned.
/// Unlike [`SynchronizedWriter`](crate::SynchronizedWriter), the generic type cannot be unsized because it is stored in an `Option`.
#[derive(Debug)]
pub struct SynchronizedOptionWriter<W> {
    inner: Arc<Mutex<Option<W>>>,
}

/// The error returned by every write operation after the writer has been taken out.
#[inline]
fn writer_removed() -> io::Error {
    io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out")
}

impl<W> Clone for SynchronizedOptionWriter<W> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<W> From<Arc<Mutex<Option<W>>>> for SynchronizedOptionWriter<W> {
    #[inline]
    fn from(writer: Arc<Mutex<Option<W>>>) -> Self {
        Self::new(writer)
    }
}

impl<W> SynchronizedOptionWriter<W> {
    /// Create a new `SynchronizedOptionWriter` which writes data into the given reference counted writer.
    #[inline]
    pub const fn new(writer: Arc<Mutex<Option<W>>>) -> Self {
        Self {
            inner: writer
        }
    }

    /// Get a reference to the inner reference counted writer.
    #[inline]
    pub const fn get_ref(&self) -> &Arc<Mutex<Option<W>>> {
        &self.inner
    }

    /// Unwrap this `SynchronizedOptionWriter`, returning the inner reference counted writer.
    #[inline]
    pub fn into_inner(self) -> Arc<Mutex<Option<W>>> {
        self.inner
    }

    /// Take the writer out of the `Arc`, or return `None` if there are still other references to it or it has already been taken out.
    #[inline]
    pub fn try_into_writer(self) -> Option<W> {
        Arc::into_inner(self.inner)
            .and_then(|mutex| mutex.into_inner().unwrap_or_else(PoisonError::into_inner))
    }

    #[inline]
    fn lock(&self) -> io::Result<MutexGuard<'_, Option<W>>> {
        self.inner.lock().map_err(|_| io::Error::from(ErrorKind::Other))
    }
}

impl<W: Write> Write for SynchronizedOptionWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.lock()?.as_mut() {
            Some(writer) => writer.write(buf),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match self.lock()?.as_mut() {
            Some(writer) => writer.write_vectored(bufs),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self.lock()?.as_mut() {
            Some(writer) => writer.flush(),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.lock()?.as_mut() {
            Some(writer) => writer.write_all(buf),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        match self.lock()?.as_mut() {
            Some(writer) => writer.write_fmt(fmt),
            None => Err(writer_removed()),
        }
    }
}
