use bytes::Bytes;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use mini_redis::client::{self, Client};
use std::sync::Arc;
use std::time::{Duration, Instant};
use local_ip_address::local_ip;

#[allow(unused_imports)]
#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}
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
    disable_raw_mode().unwrap();
    let fps = 60;
    // Establish a connection to the server 
    let mut object_positions: Arc<tokio::sync::Mutex<Vec<Object>>> = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut client = client::connect("127.0.01:6379").await.unwrap();// server ip and port 
    let formated_ip = format!("{}", local_ip().unwrap());
    println!("Your local ip is: {}", formated_ip);
    loop {
        let start = Instant::now(); 
        // PUD = Player Update flag
        if poll(std::time::Duration::from_millis(fps - 1)).unwrap() { // This is polling for input waiting for a keypress
            match read().unwrap() {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('w') => {

                        }
                        KeyCode::Char('a') => {
                            
                        }
                        KeyCode::Char('s') => {
                            
                        }
                        KeyCode::Char('d') => {
                            
                        }
                        _ => {},

                    }
                }
                _ => {continue;}
            }
        }
        let duration = start.elapsed();
        if duration < Duration::from_millis(fps) { 
            tokio::time::sleep(Duration::from_millis(fps) - duration).await; // hardsets fps to 60
        }
    }
}


async fn get_char(key: KeyCode) -> char { // converts KeyCode::Char to char, and returns ' ' if not a char
    match key {
        KeyCode::Char(c) => c,
        _ => ' ',
        
    }
}
