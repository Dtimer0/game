use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
#[allow(non_camel_case_types)]
type player_positions = Arc<tokio::sync::Mutex<HashMap<String, Bytes>>>; /*
I assume you know what a Mutex is, although I'm not sure if Mutex is it's rust specific term
Arc is a reference counted pointer, which is basically a pointer that can be shared across threads
Mutex is a guard that prevents multiple threads from accessing the same data at the same time
*/

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let player_positions: Arc<tokio::sync::Mutex<HashMap<String, Bytes>>> = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let player_positions = player_positions.clone(); // is vector: ip, position
        println!("Accepted from: {}", addr.ip());
        tokio::spawn(async move { // spawns a thread for each client connected
            process(socket, player_positions, addr).await;
        });
    }
}

async fn process(socket: TcpStream, player_positions: player_positions, sender: std::net::SocketAddr) {
    let screen_size = [300, 300];
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
                //TODO: Make sure when setting a player position, it doesn't go out of bounds
                player_positions.insert(sender.ip().to_string(), cmd.value().clone()); //sets player position based on sender ip
                Frame::Simple("OK".to_string()) // throwaway response
            }
            Get(cmd) => {
                if cmd.key() == "PUD" { // returns all the player positions
                    let mut player_update = String::new();
                    for (key, value) in player_positions.iter() {
                        player_update.push_str(&format!("{}:{};", key, String::from_utf8(value.to_vec()).unwrap()));
                    }
                    Frame::Bulk(player_update.into())
                } else if cmd.key() == "w" || cmd.key() == "a" || cmd.key() == "s" || cmd.key() == "d" { // handles player movement
                    let mut player_pos_array = split_coords(String::from_utf8(player_positions.get(&sender.ip().to_string()).unwrap().to_vec()).unwrap()).await;
                    println!("Player pos array: {:?}", player_pos_array);
                    let player_movement = player_movement.get(cmd.key()).unwrap();
                    if player_movement[0] == 1 { // this ugly shit is because we have to subtract i8 from u8, which is not allowed. Also makes sure bounds are checked.
                        if player_pos_array[0] < screen_size[0] {
                            player_pos_array[0] += 1;
                        } 
                    }
                    if player_movement[0] == -1 {
                        if player_pos_array[0] > 0 {
                            player_pos_array[0] -= 1;
                        }
                    }
                    if player_movement[1] == 1 {
                        if player_pos_array[1] < screen_size[1] {
                            player_pos_array[1] += 1;
                        } 
                    }
                    if player_movement[1] == -1 {
                        if player_pos_array[1] > 0 {
                            player_pos_array[1] -= 1;
                        }
                    }
                    let player_pos = Bytes::from(format!("{},{}", player_pos_array[0], player_pos_array[1]));
                    player_positions.insert(sender.ip().to_string(), player_pos.clone()); // updates database with new player position
                    Frame::Bulk(player_pos.clone()) // returns new position
                } else {
                    // here if get bytes do not match to PUD or directional keys
                    Frame::Null
                }
            }
            
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap(); 
    }
}

async fn split_coords(text: String) -> [u16; 2] { // splits bytes like "0,300" into [0, 300]
    let mut split_text = text.split(|x| x == ',');
    for i in split_text.clone() {
        println!("{:?}", i);
    }
    [split_text.next().unwrap().trim().parse::<u16>().unwrap(), split_text.next().unwrap().trim().parse::<u16>().unwrap()]
}