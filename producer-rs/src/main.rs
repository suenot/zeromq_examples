use zmq::Context;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn main() {
    // Создаем контекст ZeroMQ
    let context = Context::new();
    // Создаем сокет с типом PUSH
    let socket = context.socket(zmq::PUSH).unwrap();
    // Привязываем сокет к адресу
    socket.bind("tcp://*:5555").unwrap();

    loop {
        // Генерируем случайное сообщение
        let message = format!("Message {}", rand::thread_rng().gen_range(1..101));
        // Отправляем сообщение
        socket.send(message.as_str(), 0).unwrap();
        println!("Sent: {}", message);
        // Пауза на 1 секунду
        sleep(Duration::from_secs(1));
    }
}
