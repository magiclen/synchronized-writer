//! A tiny implement for synchronously writing data.
//!
//! ## Example
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

use std::sync::{Arc, Mutex};

use std::io::{self, Write};

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
        self.inner.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.lock().unwrap().flush()
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
