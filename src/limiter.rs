use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

struct TokenBucket {
    capacity: u64,
    tokens: u64,
    last_update: Instant,
}

impl TokenBucket {
    fn new(capacity: u64) -> Self {
        TokenBucket {
            capacity,
            tokens: capacity,
            last_update: Instant::now(),
        }
    }

    fn acquire(&mut self, tokens: u64) -> bool {
        self.update_tokens();
        if tokens > self.tokens {
            return false;
        }
        self.tokens -= tokens;
        true
    }

    fn update_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        let tokens_generated = (elapsed.as_secs() as u64) * self.capacity;
        self.tokens = std::cmp::min(self.tokens + tokens_generated, self.capacity);
        self.last_update = now;
    }
}

fn main() {
    let bucket = Arc::new(Mutex::new(TokenBucket::new(100)));
    let handles = (0..200)
        .map(|i| {
            let b = bucket.clone();
            std::thread::spawn(move || {
                let mut bucket = b.lock().unwrap();
                let result = bucket.acquire(1);
                if result {
                    println!("Request {} succeeded", i);
                } else {
                    println!("Request {} failed", i);
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}
