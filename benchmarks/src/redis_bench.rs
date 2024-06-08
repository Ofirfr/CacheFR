extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate redis;
extern crate stopwatch;

use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2::Pool, RedisConnectionManager};
use rand::Rng;
use std::sync::Arc;
use std::thread;
use stopwatch::Stopwatch;

const REDIS_HOST: &str = "redis://127.0.0.1:6379/";

fn main() {
    // Create a connection manager
    let manager = RedisConnectionManager::new(REDIS_HOST).unwrap();

    // Create a connection pool
    let pool = Pool::builder().build(manager).unwrap();

    let num_operations = 10000;
    let num_threads = 5; // You can adjust this based on your system's capabilities

    let mut handles = vec![];

    let stopwatch = Stopwatch::start_new();

    for _ in 0..num_threads {
        let pool = pool.clone(); // Clone the connection pool for each thread

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut conn = pool.get().unwrap(); // Get a connection from the pool

            for _ in 0..num_operations / num_threads {
                let key: u32 = rng.gen();
                let value: u32 = rng.gen();
                let key_str = key.to_string();
                let value_str = value.to_string();

                let result: r2d2_redis::redis::RedisResult<()> = conn.set(&key_str, &value_str);
                if let Err(err) = result {
                    eprintln!("Failed to set key-value pair in Redis: {}", err);
                    return;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = stopwatch.elapsed_ms();
    println!(
        "Elapsed time for {} Redis SET operations using {} threads: {} ms",
        num_operations, num_threads, elapsed_time
    );
}
