Synchronized Writer
====================

[![Build Status](https://travis-ci.org/magiclen/synchronized-writer.svg?branch=master)](https://travis-ci.org/magiclen/synchronized-writer)
[![Build status](https://ci.appveyor.com/api/projects/status/sbngpx43rkw15api/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/synchronized-writer/branch/master)

A tiny implement for synchronously writing data.

## Example

```rust
extern crate synchronized_writer;

use synchronized_writer::SynchronizedWriter;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::io::Write;

let data = Mutex::new(Vec::new());

let data_arc = Arc::new(data);

let (tx, rx) = mpsc::channel();

for _ in 0..10 {
    let mut writer = SynchronizedWriter::new(data_arc.clone());

    let tx = tx.clone();

    thread::spawn(move || {
        writer.write(b"Hello world!").unwrap();
        tx.send(0).unwrap();
    });
}

for _ in 0..10 {
    rx.recv().unwrap();
}

assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
```

## Crates.io

https://crates.io/crates/synchronized-writer

## Documentation

https://docs.rs/synchronized-writer

## License

[MIT](LICENSE)