#![feature(thread_local_state)]
extern crate futures;
extern crate tokio;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

struct TinyDB {
    db: RwLock<HashMap<String, String>>,
}

use std::cell::RefCell;

thread_local!(static FOO: RefCell<u32> = RefCell::new(10));

fn main() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 10);
        *f.borrow_mut() = 2;
    });

    let join = thread::spawn(move || {
        println!("{:?}", FOO.state());
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 10);
            *f.borrow_mut() = 222;
        });
        println!("{:?}", FOO.state());
    });

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
    join.join().unwrap();
}
