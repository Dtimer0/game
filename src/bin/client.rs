use bytes::Bytes;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use mini_redis::client;
use std::time::{Duration, Instant};
use local_ip_address::local_ip;
#[allow(unused_imports)]
#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    let fps = 60;
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();// server ip and port here 
    let formated_ip = format!("{}", local_ip().unwrap());
    println!("Your local ip is: {}", formated_ip);
    client.set(&formated_ip, "0,300".into()).await; // player starting position
    // enable_raw_mode().unwrap();
    loop {
        let start = Instant::now(); 
        // PUD = Player Update flag
        let player_pos_raw = client.get("PUD").await.unwrap().unwrap();/*looping player updates. 
        Probably a better way to do this, where the server sends all 
        clients currently connected updates every time it receives an update 
        but this is the best I got right now
        */
        println!("{:?}", player_pos_raw);
        if poll(std::time::Duration::from_millis(fps - 1)).unwrap() { // This is polling for input waiting for a keypress
            match read().unwrap() {
                Event::Key(key) => {
                    if &get_char(key.code).await == &' ' {} else { // Notice this is specifically parsing only when get char returns a char
                        println!("Text being sent: {:?}", &get_char(key.code).await);
                        println!("{:?}", client.get(&get_char(key.code).await.to_string()).await.unwrap().unwrap());
                        // client.get(&get_char(key.code).await.to_string()).await.unwrap().unwrap()
                        // above is the line of code that will tell the server to change the position based on what key was pressed
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