use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

pub async fn run() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        let (socket, _) = listner.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);  // 연결을 위한 커넥션 획득

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT : {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap()
    }
}