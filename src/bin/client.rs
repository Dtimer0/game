use bytes::Bytes;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::enable_raw_mode;
use mini_redis::client;
use tokio::sync::mpsc;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let first_pos: Bytes = vec![0, 0].into();
    let second_pos: Bytes = vec![5, 5].into();
    client.set("P1", first_pos).await.unwrap();
    client.set("P2", second_pos).await.unwrap();
    println!(
        "{}",
        format!("{:?}", client.get("P1").await.unwrap().unwrap())
    );
    // enable_raw_mode().unwrap();
    loop {
        let start = Instant::now();
        //--code--
        println!("{:?}", client.get("PlayerUpdate").await.unwrap().unwrap().iter().collect::<Vec<&u8>>());
        println!("{:?}", client.get("PlayerUpdate").await.unwrap().unwrap());
        if poll(std::time::Duration::from_millis(200)).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    change_pos(key.code, "P1", &mut client).await;
                }
                _ => {continue;}
            }
        }

        //--code--
        let duration = start.elapsed();
        if duration < Duration::from_millis(60) {
            tokio::time::sleep(Duration::from_millis(60) - duration).await;
        }
    }
}

async fn change_pos(key: KeyCode, player_name: &str, client: &mut client::Client) {
    let current_pos = client.get(player_name).await.unwrap().unwrap();
    match key {
        KeyCode::Char('w') => {
            if current_pos[1] == 255 {
                println!("Would go out of bounds");
                return;
            } else {
                client
                    .set(player_name, vec![current_pos[0], current_pos[1] + 1].into())
                    .await
                    .unwrap();
            }
        }
        KeyCode::Char('a') => {
            if current_pos[0] == 0 {
                println!("Would go out of bounds");
                return;
            } else {
                client
                    .set(player_name, vec![current_pos[0] - 1, current_pos[1]].into())
                    .await
                    .unwrap();
            }
        }
        KeyCode::Char('s') => {
            if current_pos[1] == 0 {
                println!("Would go out of bounds");
                return;
            } else {
                client
                    .set(player_name, vec![current_pos[0], current_pos[1] - 1].into())
                    .await
                    .unwrap();
            }
        }
        KeyCode::Char('d') => {
            if current_pos[0] == 255 {
                println!("Would go out of bounds");
                return;
            } else {
                client
                    .set(player_name, vec![current_pos[0] + 1, current_pos[1]].into())
                    .await
                    .unwrap();
            }
        }
        _ => {}
    }
    println!(
        "{}, {}",
        client.get(player_name).await.unwrap().unwrap()[0],
        client.get(player_name).await.unwrap().unwrap()[1]
    );
}
