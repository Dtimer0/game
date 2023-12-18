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
    let fps = 60;
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
        // PUD = Player Update flag
        // println!("{:?}", client.get("PUD").await.unwrap().unwrap().iter().collect::<Vec<&u8>>());
        // println!("{:?}", client.get("PUD").await.unwrap().unwrap());
        let player_pos_raw = client.get("PUD").await.unwrap().unwrap();
        let player_positions: Vec<(&str, [u8; 2])> = split_bytes(&player_pos_raw).await;
        for (player_name, coords) in player_positions {
            println!("{}: {:?}", player_name, coords);
        }
        if poll(std::time::Duration::from_millis(fps - 1)).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    change_pos(key.code, "P1", &mut client).await;
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


async fn split_bytes(bytes: &Bytes) -> Vec<(&str, [u8; 2])> {
    let mut player_positions: Vec<(&str, [u8; 2])> = Vec::new();
    let mut split_bytes= bytes.split(|x| x == &b';');
    for player in split_bytes {
        let mut split_player = player.split(|x| x == &b':');
        let player_name = std::str::from_utf8(split_player.next().expect("not enough values")).expect("invalid utf8");
        let coords: [u8; 2] = [split_player.next().unwrap()[0], split_player.next().unwrap()[0]];
        player_positions.push((player_name, coords));
    }
    player_positions
}