use std::sync::{Arc, Mutex};
use std::ops::DerefMut;
use std::io::{self, Write, ErrorKind};

pub struct SynchronizedOptionWriter<W: Write> {
    inner: Arc<Mutex<Option<W>>>
}

impl<W: Write> SynchronizedOptionWriter<W> {
    #[inline]
    pub fn new(writer: Arc<Mutex<Option<W>>>) -> SynchronizedOptionWriter<W> {
        SynchronizedOptionWriter {
            inner: writer
        }
    }
}

impl<W: Write> Write for SynchronizedOptionWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.deref_mut() {
            Some(writer) => writer.write(buf),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<(), io::Error> {
        match self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.deref_mut() {
            Some(writer) => writer.flush(),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
    }
}