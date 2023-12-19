use bytes::Bytes;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use mini_redis::client;
use std::time::{Duration, Instant};
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
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let _ = client.set("127.0.0.1", "0,300".into()).await;
    // enable_raw_mode().unwrap();
    loop {
        let start = Instant::now();
        //--code--
        // PUD = Player Update flag
        // println!("{:?}", client.get("PUD").await.unwrap().unwrap().iter().collect::<Vec<&u8>>());
        // println!("{:?}", client.get("PUD").await.unwrap().unwrap());
        let player_pos_raw = client.get("PUD").await.unwrap().unwrap();
        println!("{:?}", player_pos_raw);
        if poll(std::time::Duration::from_millis(fps - 1)).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    if &get_char(key.code).await == &' ' {} else {// Notice this is specifically parsing only when get char returns a char
                        println!("Text being sent: {:?}", &get_char(key.code).await);
                        println!("{:?}", client.get(&get_char(key.code).await.to_string()).await.unwrap().unwrap());
                    }
                }
                _ => {continue;}
            }
        }
        //--code--
        let duration = start.elapsed();
        if duration < Duration::from_millis(fps) {
            tokio::time::sleep(Duration::from_millis(fps) - duration).await;
        }
    }
}


async fn get_char(key: KeyCode) -> char {
    match key {
        KeyCode::Char(c) => c,
        _ => ' ',
        
    }
}