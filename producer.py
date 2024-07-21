import zmq
import time
import random

context = zmq.Context()
socket = context.socket(zmq.PUSH)
socket.bind("tcp://*:5555")

while True:
    message = f"Message {random.randint(1, 100)}"
    socket.send_string(message)
    print(f"Sent: {message}")
    time.sleep(1)