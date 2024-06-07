extern crate rand;
extern crate redis;
extern crate stopwatch;

use rand::Rng;
use redis::Commands;
use stopwatch::Stopwatch;

const REDIS_HOST: &str = "redis://127.0.0.1:6379/";

fn main() {
    // Connect to Redis
    let client = match redis::Client::open(REDIS_HOST) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to connect to Redis: {}", err);
            return;
        }
    };

    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(err) => {
            eprintln!("Failed to get Redis connection: {}", err);
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let stopwatch = Stopwatch::start_new();

    let num_operations = 10000;

    for _ in 0..num_operations {
        let key: u32 = rng.gen();
        let value: u32 = rng.gen();
        let key_str = key.to_string();
        let value_str = value.to_string();

        let result: redis::RedisResult<()> = con.set(&key_str, &value_str);
        match result {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to set key-value pair in Redis: {}", err);
                return;
            }
        }
    }

    let elapsed_time = stopwatch.elapsed_ms();
    println!(
        "Elapsed time for {} Redis SET operations: {} ms",
        num_operations, elapsed_time
    );
}
