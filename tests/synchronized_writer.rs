extern crate synchronized_writer;

use std::io::Write;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use synchronized_writer::SynchronizedWriter;

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
fn write_via_multi_threads() {
    let data = Mutex::new(Vec::new());

    let data_arc = Arc::new(data);

    let mut writer = SynchronizedWriter::new(data_arc.clone());

    writer.write_all(b"Hello world!").unwrap();

    let (tx, rx) = mpsc::channel();

    for _ in 0..9 {
        let mut writer = SynchronizedWriter::new(data_arc.clone());

        let tx = tx.clone();

        thread::spawn(move || {
            writer.write_all(b"Hello world!").unwrap();
            tx.send(0).unwrap();
        });
    }

    writer.flush().unwrap();

    for _ in 0..9 {
        rx.recv().unwrap();
    }

    assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
}
