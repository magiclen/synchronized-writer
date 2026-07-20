use std::{
    io::{ErrorKind, Write},
    sync::{Arc, Mutex},
};

use synchronized_writer::SynchronizedOptionWriter;

#[test]
fn write_to_option_vec() {
    let data = Mutex::new(Some(Vec::new()));

    let data_arc = Arc::new(data);

    let mut writer = SynchronizedOptionWriter::new(data_arc.clone());

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();

    let data = data_arc.lock().unwrap().take().unwrap(); // remove out the vec from arc

    assert_eq!(b"Hello world!".to_vec(), data);
}

#[test]
fn write_fmt_to_option_vec() {
    let data_arc = Arc::new(Mutex::new(Some(Vec::new())));

    let mut writer = SynchronizedOptionWriter::new(data_arc.clone());

    let hello = "Hello";
    let world = "world!";

    write!(writer, "{hello} {world}").unwrap();

    let data = data_arc.lock().unwrap().take().unwrap();

    assert_eq!(b"Hello world!".to_vec(), data);
}

#[test]
fn write_after_the_writer_is_removed_out() {
    let data_arc = Arc::new(Mutex::new(Some(Vec::new())));

    let mut writer = SynchronizedOptionWriter::new(data_arc.clone());

    data_arc.lock().unwrap().take().unwrap();

    assert_eq!(ErrorKind::BrokenPipe, writer.write(b"Hello world!").unwrap_err().kind());

    assert_eq!(ErrorKind::BrokenPipe, writer.write_all(b"Hello world!").unwrap_err().kind());

    assert_eq!(ErrorKind::BrokenPipe, writer.flush().unwrap_err().kind());
}
