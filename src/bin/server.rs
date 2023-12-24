use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
#[allow(non_camel_case_types)]
type object_positions= Arc<tokio::sync::Mutex<Vec<Object>>>; 
struct Coordinate {
    x: u16,
    y: u16,

}
struct Obstacle {
    coordinates: Coordinate,
    size: u16,
}
struct Player {
    name: String,
    coordinates: Coordinate,
    size: u16,
}
struct Projectile {
    owner: Player,
    coordinates: Coordinate,
    size: u16,
    velocity: (u16, u16), // angle, speed
}
enum Object {
    Obstacle(Obstacle),
    Player(Player),
    Projectile(Projectile),
}





#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    println!("Listening");
    let object_positions: Arc<tokio::sync::Mutex<Vec<Object>>> = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Accepted from: {}", addr.ip());
        let object_positions = object_positions.clone(); // is vector: ip, position
        tokio::spawn(async move { // spawns a thread for each client connected
            process(socket, object_positions, addr).await;
        });
    }
}

async fn process(socket: TcpStream, object_positions: object_positions, sender: std::net::SocketAddr) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                
            }
            Get(cmd) => {
            }
            
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // Write the response to the client
        connection.write_frame(&response).await.unwrap(); 
    }
}