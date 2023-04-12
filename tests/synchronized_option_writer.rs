use std::{
    io::Write,
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
