use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref CV: Condvar = Condvar::new();
}

#[derive(Debug, Clone)]
pub struct FifoQueue<T> {
    data: Arc<Mutex<VecDeque<T>>>,
}

impl<T> FifoQueue<T> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        data.push_back(value);
        CV.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = CV.wait(data).unwrap();
        }
        data.pop_front().unwrap()
    }

    pub fn qsize(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }

    pub fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.is_empty()
    }
}