const zmq = require("zeromq");

async function run() {
    const sock = new zmq.Pull();

    sock.connect("tcp://localhost:5555");
    console.log("Connected to port 5555");

    for await (const [msg] of sock) {
        console.log(`Received: ${msg.toString()}`);
    }
}

run();