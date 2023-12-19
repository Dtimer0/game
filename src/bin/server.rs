use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
#[allow(non_camel_case_types)]
type player_positions = Arc<tokio::sync::Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let player_positions: Arc<tokio::sync::Mutex<HashMap<String, Bytes>>> = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let player_positions = player_positions.clone();
        println!("Accepted from: {}", addr.ip());
        tokio::spawn(async move {
            process(socket, player_positions, addr).await;
        });
    }
}

async fn process(socket: TcpStream, player_positions: player_positions, sender: std::net::SocketAddr) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);
    let player_movement: HashMap<&str, [i8; 2]> = HashMap::from([("w", [0, 1]), ("a", [-1, 0]), ("s", [0, -1]), ("d", [1, 0])]);
    let mut player_positions = player_positions.lock().await;
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                player_positions.insert(sender.ip().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if cmd.key() == "PUD" {
                    let mut player_update = String::new();
                    for (key, value) in player_positions.iter() {
                        player_update.push_str(&format!("{}:{};", key, String::from_utf8(value.to_vec()).unwrap()));
                    }
                    Frame::Bulk(player_update.into())
                } else if cmd.key() == "w" || cmd.key() == "a" || cmd.key() == "s" || cmd.key() == "d" {
                    let mut player_pos_array = split_coords(String::from_utf8(player_positions.get(&sender.ip().to_string()).unwrap().to_vec()).unwrap()).await;
                    println!("Player pos array: {:?}", player_pos_array);
                    let player_movement = player_movement.get(cmd.key()).unwrap();
                    if player_movement[0] == 1 {
                        if player_pos_array[0] < 255 {
                            player_pos_array[0] += 1;
                        } 
                    }
                    if player_movement[0] == -1 {
                        if player_pos_array[0] > 0 {
                            player_pos_array[0] -= 1;
                        }
                    }
                    if player_movement[1] == 1 {
                        if player_pos_array[1] < 255 {
                            player_pos_array[1] += 1;
                        } 
                    }
                    if player_movement[1] == -1 {
                        if player_pos_array[1] > 0 {
                            player_pos_array[1] -= 1;
                        }
                    }
                    let player_pos = Bytes::from(format!("{},{}", player_pos_array[0], player_pos_array[1]));
                    player_positions.insert(sender.ip().to_string(), player_pos.clone());
                    Frame::Bulk(player_pos.clone())
                } else {
                    println!("here");
                    Frame::Null
                }
            }
            
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

async fn split_coords(text: String) -> [u8; 2] {
    let mut split_text = text.split(|x| x == ',');
    for i in split_text.clone() {
        println!("{:?}", i);
    }
    [split_text.next().unwrap().trim().parse::<u8>().unwrap(), split_text.next().unwrap().trim().parse::<u8>().unwrap()]
}