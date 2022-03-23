use std::net::SocketAddr;
use std::time::Instant;
use laminar::{ErrorKind, Packet, Socket, SocketEvent, Config};
use shared::{MessageType, MAGIC_BYTE, PlayerId};
use crate::feudle::Feudle;
use crossbeam_channel::Sender;
use std::{thread};
use std::sync::{Arc, Mutex};
use core::time;

fn send_packet(sender: &Sender<Packet>, address: SocketAddr, message_type: MessageType, payload: Vec<u8>) {
    let mut final_payload = vec![MAGIC_BYTE, message_type as u8];
    final_payload.extend(payload.iter());
    sender.send(Packet::reliable_sequenced(address, final_payload, Some(0))).unwrap();
}

fn handle_packet(packet: &Packet, game: Arc<Mutex<Feudle>>) -> bool {
    let mut game = game.lock().unwrap();
    let payload = packet.payload();
    
    if payload[0] != MAGIC_BYTE {
        println!("Received packet with invalid magic byte");
        return false;
    }    
    let message_type = payload[1];
    let data = &payload[2..];
    match message_type {
        x if x == MessageType::AssignIdEvent as u8 => {
            let assigned_id = data[0];
            println!("Assigned ID {}", assigned_id);
            game.set_id(assigned_id as PlayerId);
            return false;
        },
        x if x == MessageType::StartEvent as u8 => {
            let word = data[0];
            // println!("Starting game with word {}", word);
            //game.start_game(word);
            return true;
        },
        x if x == MessageType::GuessEvent as u8 => {
            let id = data[0];
            let guess = data[1];
            println!("Player {} guessed {}", id, guess);
            //TODO: UPDATE OPPONENT GAME DANCE
            return true;
        },
        x if x == MessageType::EndEvent as u8 => {
            let id = data[0];
            let num_guesses = data[1];
            println!("Player {} finished with {} guesses", id, num_guesses);
            //game.finish(id, num_guesses);
            return true;
        },
        x if x == MessageType::Heartbeat as u8 => {
            // println!("Received heartbeat");
            return false;
        },
        _ => {
            println!("Received unknown message type");
            return false;
        }
    }
}


fn main() -> Result<(), ErrorKind> {
    let mut socket: Socket;
    let config = Config {
        heartbeat_interval: Some(time::Duration::from_millis(10)),
        ..Config::default()
    };
    socket = Socket::bind_with_config("127.0.0.1:8439", config).unwrap();

    // Tell server to add the client
    let server_address = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let (sender, receiver) = (
        socket.get_packet_sender(), socket.get_event_receiver());
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {}...", server_address);

    let game = Arc::new(Mutex::new(Feudle::new("hello".to_string())));
    std::thread::sleep(time::Duration::from_millis(100));
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    let game_cpy = game.clone();
    let sender_cpy = sender.clone();
    let _game_thread = thread::spawn(move || {
        let mut word_guess = String::new();
    loop {
        println!("Guess a letter");
        
        std::io::stdin().read_line(&mut word_guess).expect("Failed to read line");
        game_cpy.lock().unwrap().guess(&word_guess);
        game_cpy.lock().unwrap().print_word();

        if game_cpy.lock().unwrap().check_win() {
            println!("You win!");
            send_packet(&sender_cpy, server_address, MessageType::FinishEvent, vec![game_cpy.lock().unwrap().get_id()]);
            break;
        }
        if game_cpy.lock().unwrap().guesses == game_cpy.lock().unwrap().total_guesses {
            send_packet(&sender_cpy, server_address, MessageType::LoseEvent, vec![game_cpy.lock().unwrap().get_id()]);
            println!("You lose!");
            break;
        }
    }
    std::thread::sleep(time::Duration::from_millis(100000));
    });
    
    loop {
        socket.manual_poll(Instant::now());

        let mut should_update = false;
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == server_address {

                        should_update = handle_packet(&packet, game.clone());
                    }
                },
                _ => {}
            }
        }
       
        std::thread::sleep(time::Duration::from_millis(100));
    }
}


pub fn client() {
    main();
}