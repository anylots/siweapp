use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    last_update: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64) -> Self {
        TokenBucket {
            capacity,
            tokens: capacity,
            last_update: Instant::now(),
        }
    }

    pub fn acquire(&mut self, tokens: u64) -> bool {
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

// 使用 Arc 和 Mutex 来定义全局变量 my_global_var
lazy_static::lazy_static! {
    pub static ref token_bucket: Arc<Mutex<TokenBucket>> =
        Arc::new(Mutex::new(TokenBucket::new(100)));
}

pub fn new_limiter() -> Arc<Mutex<TokenBucket>> {
    return token_bucket.clone();
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
