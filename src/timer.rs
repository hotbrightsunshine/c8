use std::{thread, time::Duration, sync::{Arc, atomic::AtomicU8}};
use std::sync::atomic::Ordering::Relaxed;

use crate::types::Data;

const PERIOD : u64 = 17;

pub struct Timer {
    value   : Arc<AtomicU8>,
}

impl<> Timer {
    pub fn new() -> Timer {
        Timer { value: Arc::new(AtomicU8::new(0)) }
    }

    pub fn set(&self, value: Data) {
        self.value.store(value, Relaxed);
    }

    pub fn get(&self) -> u8 {
        self.value.load(Relaxed)
    }

    pub fn start(&self) {
        let value = Arc::clone(&self.value);
        thread::spawn(move || {
            loop {
                if value.load(Relaxed) > 0 {
                    value.store(value.load(Relaxed) - 1, Relaxed);
                } else {}
                thread::sleep(Duration::from_millis(PERIOD));
            }
        });
    }
}