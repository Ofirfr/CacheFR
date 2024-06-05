import time
import random
import grpc
import commands_pb2
import commands_pb2_grpc
import concurrent.futures

# Address and port of the gRPC server
SERVER_ADDRESS = 'localhost'
SERVER_PORT = 50051

# Range for random number of operations
min_operations = 500
max_operations = 1500
commands = ["Get", "Set", "IntOperation", "ListOperation", "SetOperation"]

# Function to create gRPC client stub


def create_stub():
    channel = grpc.insecure_channel(f'{SERVER_ADDRESS}:{SERVER_PORT}')
    return commands_pb2_grpc.CommandsStub(channel)

# Function to execute a random command


def execute_command(stub, command):
    num_operations = random.randint(min_operations, max_operations)
    start_time = time.time()
    if command == "Get":
        for _ in range(num_operations):
            response = stub.Get(commands_pb2.FrKey(string_key="test_key"))
    elif command == "Set":
        for _ in range(num_operations):
            request = commands_pb2.SetRequest(
                key=commands_pb2.FrKey(string_key="test_key"),
                value=commands_pb2.FrValue(
                    atomic_value=commands_pb2.AtomicFrValue(string_value="test_value"))
            )
            response = stub.Set(request)
    elif command == "IntOperation":
        for _ in range(num_operations):
            request = commands_pb2.IntCommand(
                key=commands_pb2.FrKey(string_key="test_key"), increment_by=1)
            response = stub.IntOperation(request)
    elif command == "ListOperation":
        for _ in range(num_operations):
            request = commands_pb2.ListCommand(key=commands_pb2.FrKey(
                string_key="test_key"), append=commands_pb2.AtomicFrValue(string_value="test_value"))
            response = stub.ListOperation(request)
    elif command == "SetOperation":
        for _ in range(num_operations):
            request = commands_pb2.SetCommnad(key=commands_pb2.FrKey(
                string_key="test_key"), add=commands_pb2.AtomicFrValue(string_value="test_value"))
            response = stub.SetOperation(request)
    end_time = time.time()
    return (end_time - start_time) / num_operations * 1000

# Function to simulate clients executing random commands


def simulate_clients(num_clients):
    stub = create_stub()
    with concurrent.futures.ThreadPoolExecutor(max_workers=num_clients) as executor:
        futures = []
        for _ in range(num_clients):
            command = random.choice(commands)
            futures.append(executor.submit(execute_command, stub, command))
        results = [future.result()
                   for future in concurrent.futures.as_completed(futures)]
        avg_time = sum(results) / len(results)
        print(
            "Average time per operation across all clients: {:.2f} ms".format(avg_time))


# Run simulation with 5 clients
simulate_clients(5)
