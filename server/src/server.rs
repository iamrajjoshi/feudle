use core::time;
use std::{thread};
use std::collections::HashMap;
use std::net::SocketAddr;
use crossbeam_channel::Sender;
use laminar::{ErrorKind, Packet, Socket, SocketEvent, Config};
use shared::{MessageType, PlayerId, MAGIC_BYTE, MAX_PLAYERS};
use rand::Rng;
struct ServerState {
    pub player_ids: Vec<PlayerId>,
    pub address_to_id: HashMap<SocketAddr, PlayerId>,
    pub id_to_address: HashMap<PlayerId, SocketAddr>,
    pub players_ready: HashMap<PlayerId, bool>,
    pub game_started: bool,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            player_ids: vec![],
            address_to_id: HashMap::new(),
            id_to_address: HashMap::new(),
            players_ready: HashMap::new(),
            game_started: false,
        }
    }

    pub fn get_player_count(&self) -> usize {
        self.player_ids.len()
    }
    
    pub fn get_next_player_id(&mut self) -> PlayerId {
        let id = self.player_ids.len() as PlayerId;
        self.player_ids.push(id + 1);
        id
    }
    
    pub fn bind_player(&mut self, address: SocketAddr, id: PlayerId) {
        self.address_to_id.insert(address, id);
        self.id_to_address.insert(id, address);
    }
    
    pub fn start_game(&mut self) {
        self.game_started = true;
    }
    
    pub fn end_game(&mut self) {
        self.game_started = false;
    }
}

fn send_packet(sender: &Sender<Packet>, address: SocketAddr, message_type: MessageType, payload: Vec<u8>) {
    //print message type as a string
    let message_type_string = match message_type {
        MessageType::Heartbeat => "Heartbeat",
        MessageType::JoinEvent => "JoinEvent",
        MessageType::AssignIdEvent => "AssignIdEvent",
        MessageType::ReadyEvent => "ReadyEvent",
        MessageType::StartEvent => "StartEvent",
        MessageType::GuessEvent => "GuessEvent",
        MessageType::FinishEvent => "FinishEvent",
        MessageType::EndEvent => "EndEvent",
        _ => "Unknown",
    };
    println!("Sending {} to {}", message_type_string, address);


    let mut final_payload = vec![MAGIC_BYTE, message_type as u8];
    final_payload.extend(payload);
   
    sender.send(Packet::reliable_sequenced(address, final_payload, Some(0))).unwrap();
}



fn handle_packet(sender: &Sender<Packet>, packet: &Packet, state: &mut ServerState) {
    
    let address = packet.addr();
    let payload = packet.payload();

    if payload[0] != MAGIC_BYTE {
        println!("Received packet with invalid magic byte");
        return;
    }
    
    let message_type = payload[1];
    
    let message_type_string = match message_type {
        x if x == MessageType::JoinEvent as u8 => "JoinEvent",
        x if x == MessageType::ReadyEvent as u8 => "ReadyEvent",
        x if x == MessageType::GuessEvent as u8 => "GuessEvent",
        x if x == MessageType::FinishEvent as u8 => "FinishEvent",
        x if x == MessageType::Heartbeat as u8 => "Heartbeat",
        x if x == MessageType::LoseEvent as u8 => "LoseEvent",
        _ => "Unknown",
    };
    println!("Received {}", message_type_string);

    
    let data = &payload[2..];

    match message_type {
        x if x == MessageType::JoinEvent as u8 => {
            if state.get_player_count() >= MAX_PLAYERS || state.address_to_id.contains_key(&address) {
                return
            }

            let id = state.get_next_player_id();
            state.bind_player(address, id);
            println!("Player {} joined", id);
            send_packet(sender, address, MessageType::AssignIdEvent, vec![id]);

        },
        x if x == MessageType::ReadyEvent as u8 => {
            state.players_ready.insert(data[0], true);
            if state.players_ready.len() == MAX_PLAYERS {
                println!("Starting game");
                state.start_game();
                let index: u8 = rand::thread_rng().gen_range(0..255);
                for (&player_address, _) in state.address_to_id.iter() {
                    send_packet(sender, player_address, MessageType::StartEvent, vec![index]);
                }
            }
        },
        x if x == MessageType::GuessEvent as u8 => {
            let id = data[0];
            let guess = &data[1..];
            let address = state.id_to_address.get(&id).unwrap();
            let mut payload = vec![id];
            payload.extend(guess);
            //send guess to other players
            for (&player_address, _) in state.address_to_id.iter() {
                if player_address != *address {
                    send_packet(sender, player_address, MessageType::GuessEvent, payload.clone());
                }
            }
        },
        x if x == MessageType::FinishEvent as u8 => {
            let id = data[0];
            println!("Player {} won", id);
            state.end_game();
            for (&player_address, _) in state.address_to_id.iter() {
                    send_packet(sender, player_address, MessageType::EndEvent, vec![id]);
            }
        },
        x if x == MessageType::LoseEvent as u8 => {
            let id = data[0];
            let winning_id = if id == 0 {1} else{0};
            // let num_guesses = payload[1];
            // let address = state.id_to_address.get(&id).unwrap();
            print!("Player {} lost", id);
            print!("Player {} won", winning_id);
            state.end_game();
            //send won event to all players, but send lost event to other player
            for (&player_address, _) in state.address_to_id.iter() {
                send_packet(sender, player_address, MessageType::EndEvent, vec![winning_id]);
            }
        },
        x  if x == MessageType::Heartbeat as u8 => {
            // println!("Heartbeat from {}", address);
            send_packet(sender, address, MessageType::Heartbeat, vec![]);
        },
        _ => {},
    }
}

pub fn server() -> Result<(), ErrorKind> {
    let mut state = ServerState::new();
    let config = Config {
        heartbeat_interval: Some(time::Duration::from_millis(10)),
        ..Config::default()
    };
    let mut socket = Socket::bind_with_config("192.168.0.110:8001", config).unwrap();
    println!("Listening on 192.168.0.110:8001");

    let (sender, receiver) = (
        socket.get_packet_sender(), socket.get_event_receiver());
    
        let _thread = thread::spawn(move || socket.start_polling());

    loop {
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Connect(address) => {
                    send_packet(&sender, address, MessageType::Heartbeat, vec![]);
                    println!("Client {} connected", address);
                },
                SocketEvent::Packet(packet) => handle_packet(&sender, &packet, &mut state),
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                },                
            }
        }
        std::thread::sleep(time::Duration::from_millis(100));
    }
}