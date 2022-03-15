//Things to Keep in Mind
// We have to make sure to keep polling the server for new messages
// Perhaps we should have a separate thread for this?
// we should also have a seperate thread for the game logic and rendering
use std::net::SocketAddr;
use std::time::Instant;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use shared::{MessageType, MAGIC_BYTE};
use crate::feudle::Feudle;

use core::time;
fn send_packet(sender: &mut Socket, address: SocketAddr, message_type: MessageType, payload: Vec<u8>) {
    let mut final_payload = vec![MAGIC_BYTE, message_type as u8];
    final_payload.extend(payload.iter());
    sender.send(Packet::reliable_ordered(address, final_payload, Some(0))).unwrap();
    sender.manual_poll(Instant::now());
}

fn handle_packet(sender: &mut Socket, packet: &Packet, game: &mut Feudle) -> bool {
    let payload = packet.payload();
    //print payload to console
    println!("{:?}", payload);
    
    if payload[0] != MAGIC_BYTE {
        println!("Received packet with invalid magic byte");
        return false;
    }
    
    let message_type = payload[1];
    let data = &payload[2..];
    //print message type
    // println!("Received message type: {:?}", message_type);
    match message_type {
        x if x == MessageType::AssignIdEvent as u8 => {
            let assigned_id = data[0];
            println!("Assigned ID {}", assigned_id);
            //TODO: Add to game implementation to store id
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
            println!("Received heartbeat");
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
    

        socket = Socket::bind("127.0.0.1:8432").unwrap();
    //////////////////////////////////////////////////////////////////////////////////////////////////////

    // Tell server to add the client
    let server_address = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    // let server_address = match server_address_str.parse::<SocketAddr>() {
    //     Ok(address) => address,
    //     Err(_) => "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
    // };
    send_packet(&mut socket, server_address, MessageType::JoinEvent, vec![]);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {}...", server_address);

    let mut game = Feudle::new("hello".to_string());
        
    std::thread::sleep(time::Duration::from_millis(100));
    send_packet(&mut socket, server_address, MessageType::JoinEvent, vec![]);
    let mut last_heartbeat_time: f64 = -10.0;

    loop {
        //print server joined

        socket.manual_poll(Instant::now());

        let mut should_update = false;
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                // print!("Received packet from {:?}", packet);
                if packet.addr() == server_address {
                    should_update = handle_packet(&mut socket, &packet, &mut game);
                    // send_packet(MessageType::Heartbeat, vec![], server_address, 
                    //     StreamId::Heartbeat as u8, &mut socket);
                }
            },
            _ => {}
        }
        // send_packet(MessageType::DeathEvent, vec![], server_address, StreamId::Heartbeat as u8, &mut socket);
        // Handles the game state 
        
        // if game.has_started() {
            //check win, if win send finished even
            //if opponent has finished, display end screen

        // }
        
        // Send heartbeat if no event has occurred during specified period to prevent timeout
        std::thread::sleep(time::Duration::from_millis(100));
            send_packet(&mut socket, server_address, MessageType::Heartbeat, vec![]);

        
        game.update(should_update);
        // game.handle_events();


        // next_frame().await;
    }
}


pub fn client() {
    main();
}