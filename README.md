Synchronized Writer
====================

[![CI](https://github.com/magiclen/synchronized-writer/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/synchronized-writer/actions/workflows/ci.yml)

A tiny implement for synchronously writing data.

## Examples

### SynchronizedWriter

```rust
use synchronized_writer::SynchronizedWriter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;

let data = Mutex::new(Vec::new());

let data_arc = Arc::new(data);

let mut threads = Vec::with_capacity(10);

for _ in 0..10 {
    let mut writer = SynchronizedWriter::new(data_arc.clone());

    let thread = thread::spawn(move || {
        writer.write(b"Hello world!").unwrap();
    });

    threads.push(thread);
}

for thread in threads {
    thread.join().unwrap();
}

assert_eq!(b"Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!".to_vec(), *data_arc.lock().unwrap());
```

### SynchronizedOptionWriter

```rust
use synchronized_writer::SynchronizedOptionWriter;
use std::sync::{Arc, Mutex};
use std::io::Write;

let data = Mutex::new(Some(Vec::new()));

let data_arc = Arc::new(data);

let mut writer = SynchronizedOptionWriter::new(data_arc.clone());

writer.write(b"Hello world!").unwrap();

writer.flush().unwrap();

let data = data_arc.lock().unwrap().take().unwrap(); // remove out the vec from arc

assert_eq!(b"Hello world!".to_vec(), data);
```

## Crates.io

https://crates.io/crates/synchronized-writer

## Documentation

https://docs.rs/synchronized-writer

## License

[MIT](LICENSE)