use std::sync::{Arc, Mutex};
use std::io::{self, Write, ErrorKind};

pub struct SynchronizedWriter<W: Write> {
    inner: Arc<Mutex<W>>
}

impl<W: Write> SynchronizedWriter<W> {
    pub fn new(writer: Arc<Mutex<W>>) -> SynchronizedWriter<W> {
        SynchronizedWriter {
            inner: writer
        }
    }
}

impl<W: Write> Write for SynchronizedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.flush()
    }
}