//! A tiny implement for synchronously writing data.
//!
//! ## Examples
//!
//! ### SynchronizedWriter
//!
//! ```
//! extern crate synchronized_writer;
//!
//! use synchronized_writer::SynchronizedWriter;
//! use std::sync::{Arc, Mutex, mpsc};
//! use std::thread;
//! use std::io::Write;
//!
//! let data = Mutex::new(Vec::new());
//!
//! let data_arc = Arc::new(data);
//!
//! let (tx, rx) = mpsc::channel();
//!
//! for _ in 0..10 {
//!     let mut writer = SynchronizedWriter::new(data_arc.clone());
//!
//!     let tx = tx.clone();
//!
//!     thread::spawn(move || {
//!         writer.write(b"Hello world!").unwrap();
//!         tx.send(0).unwrap();
//!     });
//! }
//!
//! for _ in 0..10 {
//!     rx.recv().unwrap();
//! }
//!
//! assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
//! ```
//!
//! ### SynchronizedOptionWriter
//!
//! ```
//! extern crate synchronized_writer;
//!
//! use synchronized_writer::SynchronizedOptionWriter;
//! use std::sync::{Arc, Mutex};
//! use std::io::Write;
//!
//! let data = Mutex::new(Some(Vec::new()));
//!
//! let data_arc = Arc::new(data);
//!
//! let mut writer = SynchronizedOptionWriter::new(data_arc.clone());
//!
//! writer.write(b"Hello world!").unwrap();
//!
//! writer.flush().unwrap();
//!
//! let data = data_arc.lock().unwrap().take().unwrap(); // remove out the vec from arc
//!
//! assert_eq!(b"Hello world!".to_vec(), data);
//! ```

use std::sync::{Arc, Mutex};

use std::ops::DerefMut;

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


pub struct SynchronizedOptionWriter<W: Write> {
    inner: Arc<Mutex<Option<W>>>
}

impl<W: Write> SynchronizedOptionWriter<W> {
    pub fn new(writer: Arc<Mutex<Option<W>>>) -> SynchronizedOptionWriter<W> {
        SynchronizedOptionWriter {
            inner: writer
        }
    }
}

impl<W: Write> Write for SynchronizedOptionWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.deref_mut() {
            Some(writer) => writer.write(buf),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        match self.inner.lock().map_err(|err| io::Error::new(ErrorKind::WouldBlock, err.to_string()))?.deref_mut() {
            Some(writer) => writer.flush(),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;
    use std::sync::mpsc;

    #[test]
    fn write_to_vec() {
        let data = Mutex::new(Vec::new());

        let data_arc = Arc::new(data);

        let mut writer = SynchronizedWriter::new(data_arc.clone());

        writer.write(b"Hello world!").unwrap();

        writer.flush().unwrap();

        assert_eq!(b"Hello world!".to_vec(), *data_arc.lock().unwrap());
    }

    #[test]
    fn write_to_option_vec() {
        let data = Mutex::new(Some(Vec::new()));

        let data_arc = Arc::new(data);

        let mut writer = SynchronizedOptionWriter::new(data_arc.clone());

        writer.write(b"Hello world!").unwrap();

        writer.flush().unwrap();

        let data = data_arc.lock().unwrap().take().unwrap(); // remove out the vec from arc

        assert_eq!(b"Hello world!".to_vec(), data);
    }

    #[test]
    fn write_via_multi_threads() {
        let data = Mutex::new(Vec::new());

        let data_arc = Arc::new(data);

        let mut writer = SynchronizedWriter::new(data_arc.clone());

        writer.write(b"Hello world!").unwrap();

        let (tx, rx) = mpsc::channel();

        for _ in 0..9 {
            let mut writer = SynchronizedWriter::new(data_arc.clone());

            let tx = tx.clone();

            thread::spawn(move || {
                writer.write(b"Hello world!").unwrap();
                tx.send(0).unwrap();
            });
        }

        writer.flush().unwrap();

        for _ in 0..9 {
            rx.recv().unwrap();
        }

        assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
    }
}
