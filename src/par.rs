use std::collections::VecDeque;
use std::thread;

pub struct Par {
    nthreads: usize,
    que: VecDeque<thread::JoinHandle<()>>,
}

impl Par {
    pub fn new(nthreads: usize) -> Par {
        Par {
            nthreads,
            que: VecDeque::with_capacity(nthreads),
        }
    }

    pub fn run<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        while self.que.len() >= self.nthreads {
            let t = self.que.pop_front().unwrap();
            t.join().unwrap();
        }
        let t = thread::spawn(f);
        self.que.push_back(t);
    }

    pub fn finish(self) {
        drop(self);
    }
}

impl Drop for Par {
    fn drop(&mut self) {
        while let Some(t) = self.que.pop_front() {
            t.join().unwrap();
        }
    }
}
