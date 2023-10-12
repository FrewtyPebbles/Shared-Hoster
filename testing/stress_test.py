import http.client as client
import sys
from threading import Thread
import time

# arg1 is number of ports, arg2 is number of requests

num_of_ports = int(sys.argv[1])
num_of_requests = int(sys.argv[2])


def measure_server(port):
    global total
    # returns average
    requests:list[Thread] = []
    average_obj = [0]
    for _ in range(num_of_requests):
        requests.append(Thread(target=lambda:measure_request(port, average_obj)))
    
    time1 = time.thread_time_ns()
    for request in requests:
        request.start()
        request.join()
    time2 = time.thread_time_ns()
    average = average_obj[0]/len(requests)

    print(f"[PORT] {port} [----]")
    print(f"[Total Time] port ({port}) = {(time2 - time1) / 100000}ms")
    print(f"[Average Time] port ({port}) = {average}ms")
    print(f"[----------------]")

    total += average

def measure_request(port, average):

    time1 = time.thread_time_ns()

    con = client.HTTPConnection("127.0.0.1", port)
    con.request("GET", "/", headers={
        "Content-Type": "text/plain",
        "User-Agent": "PostmanRuntime/7.33.0",
        "Accept": "*/*",
        "Cache-Control": "no-cache",
        "Postman-Token": "52735b94-c3f5-4e85-9338-7bd522958ce2",
        "Host": "localhost:8071",
        "Accept-Encoding": "gzip, deflate, br",
        "Connection": "keep-alive",
        "Content-Length": "0"
    })
    con.getresponse()

    time2 = time.thread_time_ns()

    average[0] += (time2 - time1) / 100000


total = 0

servers:list[Thread] = []

for port in range(65536 - num_of_ports, 65536):
    servers.append(Thread(target=measure_server, args=[port]))

for server in servers:
    server.run()

final_average = total/len(servers)
print("\nFINAL RESULTS:")
print(f"[Test Total] port (ALL) = {total}ms")
print(f"[Test Average] port (ALL) = {final_average}ms")