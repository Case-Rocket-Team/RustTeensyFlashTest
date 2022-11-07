// Not implemented yet!!!!

use core::{any::Any, mem::transmute};

enum TaskStatus<TOutput> {
    Ready(TOutput),
    Pending
}

struct Concurrency<const TQUEUESIZE: usize>{
    queue: [usize; TQUEUESIZE],
    start: usize,
    end: usize
}

// Unimplemented

impl<const TQUEUESIZE: usize> Concurrency<TQUEUESIZE> {
    pub fn add<'a, T>(&mut self, func: fn() -> TaskStatus<&'a T>) {
        unsafe {
            self.queue[self.end] = transmute(func);
            self.end += 1;
        }
    }

    pub fn tick() {

    }
}