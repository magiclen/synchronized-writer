use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

use synchronized_writer::SynchronizedWriter;

/// A writer that accepts at most one byte per `write` call, so `write_all` has to loop.
struct OneByteWriter {
    data: Vec<u8>,
}

impl Write for OneByteWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match buf.first() {
            Some(&byte) => {
                self.data.push(byte);

                Ok(1)
            },
            None => Ok(0),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn write_to_vec() {
    let data = Mutex::new(Vec::new());

    let data_arc = Arc::new(data);

    let mut writer = SynchronizedWriter::new(data_arc.clone());

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();

    assert_eq!(b"Hello world!".to_vec(), *data_arc.lock().unwrap());
}

#[test]
fn write_fmt_to_vec() {
    let data_arc = Arc::new(Mutex::new(Vec::new()));

    let mut writer = SynchronizedWriter::new(data_arc.clone());

    let hello = "Hello";
    let world = "world!";

    write!(writer, "{hello} {world}").unwrap();

    assert_eq!(b"Hello world!".to_vec(), *data_arc.lock().unwrap());
}

#[test]
fn write_to_unsized_vec() {
    let data_arc: Arc<Mutex<dyn Write>> = Arc::new(Mutex::new(Vec::new()));

    let mut writer = SynchronizedWriter::new(data_arc);

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();
}

#[test]
fn try_into_writer() {
    let data_arc = Arc::new(Mutex::new(Vec::new()));

    let mut writer = SynchronizedWriter::new(data_arc.clone());

    writer.write_all(b"Hello world!").unwrap();

    drop(data_arc);

    assert_eq!(b"Hello world!".to_vec(), writer.try_into_writer().unwrap());
}

#[test]
fn write_via_multi_threads() {
    let data = Mutex::new(Vec::new());

    let data_arc = Arc::new(data);

    thread::scope(|s| {
        for _ in 0..10 {
            let mut writer = SynchronizedWriter::new(data_arc.clone());

            s.spawn(move || {
                writer.write_all(b"Hello world!").unwrap();
            });
        }
    });

    assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
}

#[test]
fn write_all_is_atomic_via_multi_threads() {
    const THREADS: usize = 8;
    const LEN: usize = 200;

    let data_arc = Arc::new(Mutex::new(OneByteWriter {
        data: Vec::new()
    }));

    thread::scope(|s| {
        for i in 0..THREADS {
            let mut writer = SynchronizedWriter::new(data_arc.clone());

            let chunk = vec![b'a' + i as u8; LEN];

            s.spawn(move || {
                writer.write_all(&chunk).unwrap();
            });
        }
    });

    let output = data_arc.lock().unwrap().data.clone();

    assert_eq!(THREADS * LEN, output.len());

    // Each `write_all` is atomic, so the output is `THREADS` contiguous runs, each a single distinct byte of length `LEN`.
    let mut seen = Vec::with_capacity(THREADS);

    for run in output.chunks(LEN) {
        let byte = run[0];

        assert!(run.iter().all(|&b| b == byte));

        seen.push(byte);
    }

    seen.sort_unstable();
    seen.dedup();

    assert_eq!(THREADS, seen.len());
}
