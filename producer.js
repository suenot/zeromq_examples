const zmq = require('zeromq');
const { randomInt } = require('crypto');

// Создаем контекст и сокет PUSH
const sock = new zmq.Push;

// Привязываем сокет к адресу
sock.bind('tcp://*:5555')
    .then(() => {
        console.log('Server is ready and bound to port 5555');

        // Функция для отправки сообщения каждые 1 секунду
        const sendMessage = async () => {
            while (true) {
                const message = `Message ${randomInt(1, 101)}`;
                await sock.send(message);
                console.log(`Sent: ${message}`);
                await new Promise(resolve => setTimeout(resolve, 1000));
            }
        };

        sendMessage();
    })
    .catch(err => {
        console.error('Failed to bind socket:', err);
    });
