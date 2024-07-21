use zmq;

fn main() {
    // Создаем новый контекст ZMQ
    let context = zmq::Context::new();
    
    // Создаем сокет типа PULL
    let socket = context.socket(zmq::PULL).expect("Не удалось создать сокет");
    
    // Подключаемся к адресу tcp://localhost:5555
    socket.connect("tcp://localhost:5555").expect("Не удалось подключиться к серверу");
    
    // Бесконечный цикл для получения сообщений
    loop {
        // Получаем сообщение в виде строки
        let message = socket.recv_string(0).expect("Не удалось получить сообщение").expect("Получено пустое сообщение");
        
        // Печатаем полученное сообщение
        println!("Received: {}", message);
    }
}
