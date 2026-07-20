use std::{
    fmt,
    io::{self, ErrorKind, IoSlice, Write},
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

/// A writer which writes data into a reference counted, mutex-protected writer.
///
/// Every write operation locks the inner mutex, so it waits until the lock is free and fails if the lock is poisoned.
#[derive(Debug)]
pub struct SynchronizedWriter<W: ?Sized> {
    inner: Arc<Mutex<W>>,
}

impl<W: ?Sized> Clone for SynchronizedWriter<W> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<W: ?Sized> From<Arc<Mutex<W>>> for SynchronizedWriter<W> {
    #[inline]
    fn from(writer: Arc<Mutex<W>>) -> Self {
        Self::new(writer)
    }
}

impl<W: ?Sized> SynchronizedWriter<W> {
    /// Create a new `SynchronizedWriter` which writes data into the given reference counted writer.
    #[inline]
    pub const fn new(writer: Arc<Mutex<W>>) -> Self {
        Self {
            inner: writer
        }
    }

    /// Get a reference to the inner reference counted writer.
    #[inline]
    pub const fn get_ref(&self) -> &Arc<Mutex<W>> {
        &self.inner
    }

    /// Unwrap this `SynchronizedWriter`, returning the inner reference counted writer.
    #[inline]
    pub fn into_inner(self) -> Arc<Mutex<W>> {
        self.inner
    }

    #[inline]
    fn lock(&self) -> io::Result<MutexGuard<'_, W>> {
        self.inner.lock().map_err(|_| io::Error::from(ErrorKind::Other))
    }
}

impl<W> SynchronizedWriter<W> {
    /// Take the writer out of the `Arc`, or return `None` if there are still other references to it.
    #[inline]
    pub fn try_into_writer(self) -> Option<W> {
        Arc::into_inner(self.inner)
            .map(|mutex| mutex.into_inner().unwrap_or_else(PoisonError::into_inner))
    }
}

impl<W: Write + ?Sized> Write for SynchronizedWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.lock()?.write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.lock()?.write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.lock()?.flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.lock()?.write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.lock()?.write_fmt(fmt)
    }
}
