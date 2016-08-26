use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::io;

use handler::Handler;


pub struct Portio<H> {
    handler: H,
    queue: Arc<Mutex<Vec<String>>>,
}

impl<H: Handler> Portio<H> {

    pub fn new(handler: H) -> Self {
        Portio {
            handler: handler,
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn listen(&self) {
        let queue = self.queue.clone();

        thread::spawn(move || {
            let mut string = String::new();
            loop {
                match io::stdin().read_line(&mut string) {
                    Ok(_) => {
                        string.pop(); // remove newline char
                        Self::push_queue(&*queue, &string);
                        string.clear();
                    },
                    Err(_) => (),
                }
            }
        });

        loop {
            match self.queue.lock() {
                Ok(mut queue_guard) => self.handle(&mut queue_guard),
                Err(_) => (),
            }
        }
    }

    fn handle(&self, guard: &mut MutexGuard<Vec<String>>) {
        while let Some(string) = guard.pop() {
            self.handler.handle(&string);
        }
    }

    fn push_queue(mutex: &Mutex<Vec<String>>, string: &str) {
        match mutex.lock() {
            Ok(mut queue) => queue.push(String::from(string)),
            Err(_) => Self::push_queue(mutex, string),
        }
    }
}
