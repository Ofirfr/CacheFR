import time
import random
import redis
import concurrent.futures

# Initialize Redis connection
r = redis.StrictRedis(host='localhost', port=6379, db=0)

# Range for random number of operations
min_operations = 500
max_operations = 1500
commands = ["GET", "SET", "SADD", "SREM", "SISMEMBER"]


def execute_command(command):
    num_operations = random.randint(min_operations, max_operations)
    start_time = time.time()
    if command == "GET":
        for _ in range(num_operations):
            r.get("test_key")
    elif command == "SET":
        for _ in range(num_operations):
            r.set("test_key", "test_value")
    elif command == "SADD":
        for i in range(num_operations):
            r.sadd("test_set", "member{}".format(i))
    elif command == "SREM":
        for i in range(num_operations):
            r.srem("test_set", "member{}".format(i))
    elif command == "SISMEMBER":
        for i in range(num_operations):
            r.sismember("test_set", "member{}".format(i))
    end_time = time.time()
    return (end_time - start_time) / num_operations * 1000

# Function to simulate clients executing random commands


def simulate_clients(num_clients):
    with concurrent.futures.ThreadPoolExecutor(max_workers=num_clients) as executor:
        futures = []
        for _ in range(num_clients):
            command = random.choice(commands)
            futures.append(executor.submit(execute_command, command))
        results = [future.result()
                   for future in concurrent.futures.as_completed(futures)]
        avg_time = sum(results) / len(results)
        print(
            "Average time per operation across all clients: {:.2f} ms".format(avg_time))


# Run simulation with 5 clients
simulate_clients(5)
