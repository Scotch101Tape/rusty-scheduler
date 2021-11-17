# Rusty Scheduler

A single threaded closure and function scheduler in Rust.

## Usage

```Rust
use rusty_scheduler::Scheduler;

let schedular = Scheduler::new();

let x = 1;
schedular.defer(move || {
    let y = 2;
    assert_eq!(x + y, 3);
});

schedular.defer(|| {
    assert_eq!(1, 1);
});

schedular.run();
```

This will run the first closure first, then the second closure when run is called.

## Why

This is a project to help me understand Rust and Cargo. I originally wanted to use this without the standerd library (so I could run it on microcontrollers), but that is beyond my capibilites for the moment. I may revisit and continue this in the future.
