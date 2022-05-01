use std::net::SocketAddr;
use std::time::Instant;
use std::io;
use laminar::{ErrorKind, Packet, Socket, SocketEvent, Config};
use shared::{MessageType, MAGIC_BYTE, PlayerId};
use crate::feudle::Feudle;
use crossbeam_channel::Sender;
use std::{thread};
use std::sync::{Arc, Mutex};
use core::time;
use std::io::Write;
use resource::get_word;
struct ClientState {
    id: PlayerId,
    ready: bool,
    game_started: bool,
    game_over: bool,
    word: String,
}

impl  ClientState {
    pub fn new() -> Self {
        ClientState {
            id: 0,
            ready: false,
            game_started: false,
            game_over: false,
            word: String::new(),
        }
    }

    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    pub fn get_ready(&self) -> bool {
        self.ready
    }

    pub fn set_game_started(&mut self, game_started: bool) {
        self.game_started = game_started;
    }

    pub fn get_game_started(&self) -> bool {
        self.game_started
    }

    pub fn set_id(&mut self, id: PlayerId) {
        self.id = id;
    }

    pub fn get_id(&self) -> PlayerId {
        self.id
    }

    pub fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    pub fn set_word(&mut self, word: String) {
        self.word = word;
    }

    pub fn get_color_vec(&self, guess: String) -> Vec<char> {
        let uppercase_guess = guess.to_uppercase();
        let mut color_vec = Vec::new();
        for (i, c ) in uppercase_guess.chars().enumerate() {
            if self.word.contains(c) && self.word.to_string().chars().nth(i) == Some(c) {
                color_vec.push('G');
            }
            else if self.word.contains(c) && self.word.to_string().chars().nth(i) != Some(c) {
                color_vec.push('Y');
            }
            else {
                color_vec.push('_');
            }
        }
        color_vec
    }
}

fn send_packet(sender: &Sender<Packet>, address: SocketAddr, message_type: MessageType, payload: Vec<u8>) {
    let mut final_payload = vec![MAGIC_BYTE, message_type as u8];
    final_payload.extend(payload.iter());
    // print!("Sending packet to {}: {:?}", address, final_payload);

    sender.send(Packet::reliable_sequenced(address, final_payload, Some(0))).unwrap();
}

fn handle_packet(packet: &Packet, game: Arc<Mutex<Feudle>>, state: Arc<Mutex<ClientState>>) -> bool {
    let payload = packet.payload();
    // print!("Received packet: {:?}", payload);
    if payload[0] != MAGIC_BYTE {
        println!("Received packet with invalid magic byte");
        return false;
    }    
    let message_type = payload[1];
    let data = &payload[2..];

    // let message_type_string = match message_type {
    //     x if x == MessageType::AssignIdEvent as u8 => "AssignIdEvent",
    //     x if x == MessageType::StartEvent as u8 => "StartEvent",
    //     x if x == MessageType::GuessEvent as u8 => "GuessEvent",
    //     x if x == MessageType::EndEvent as u8 => "EndEvent",
    //     x  if x == MessageType::Heartbeat as u8 => "Heartbeat",
    //     _ => "Unknown",
    // };
    // println!("Received {}", message_type_string);

    match message_type {
        x if x == MessageType::AssignIdEvent as u8 => {
            let assigned_id = data[0];
            // println!("Assigned ID {}", assigned_id);
            state.lock().unwrap().set_id(assigned_id as PlayerId);
            return false;
        },
        x if x == MessageType::StartEvent as u8 => {
            let index = data[0];
            let word = get_word(index as usize);
            println!("Starting game with word {}", word);
            game.lock().unwrap().set_word(word.clone());
            state.lock().unwrap().set_word(word.to_uppercase());
            state.lock().unwrap().set_game_started(true);
            return true;
        },
        x if x == MessageType::GuessEvent as u8 => {
            // let id = data[0];
            let guess = String::from_utf8(data[1..].to_vec()).unwrap();
            // println!("Player {} guessed {}", id, guess);
            let color_vec = state.lock().unwrap().get_color_vec(guess.clone());
            print!("Opponent's Guess: {:?}\n", color_vec);
            io::stdout().flush().unwrap();
            //TODO: UPDATE OPPONENT GAME DANCE
            return true;
        },
        x if x == MessageType::EndEvent as u8 => {
            let id = data[0];
            state.lock().unwrap().set_game_over(true);
            if id == state.lock().unwrap().get_id() as u8 {
                println!("You won!");
            } else {
                println!("You lost!");
            }
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
    let mut port = 8432;
    let mut addr: String;
    loop {
        addr = format!("{}:{}", "127.0.0.1", port);
        match Socket::bind_with_config(&addr, config.clone()) {
            Ok(s) => {
                socket = s;
                break;
            },
            Err(_) => {
                port += 1;
            }
        };
    }

    println!("Listening on {}", addr);
    // socket = Socket::bind_with_config("127.0.0.1:8452", config).unwrap();

    // Tell server to add the client
    let server_address = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let (sender, receiver) = (
        socket.get_packet_sender(), socket.get_event_receiver());
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {}", server_address);

    let game = Arc::new(Mutex::new(Feudle::new()));
    let state = Arc::new(Mutex::new(ClientState::new()));
    
    std::thread::sleep(time::Duration::from_millis(100));
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    
    let game_cpy = game.clone();
    let sender_cpy = sender.clone();
    let state_cpy = state.clone();
    
    let _game_thread = thread::spawn(move || {
        while state_cpy.lock().unwrap().get_ready() == false {
            print!("Are you ready? (y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut &mut input).unwrap();
            input = input.trim().to_string();
            if input == "y" {
                state_cpy.lock().unwrap().set_ready(true);
                send_packet(&sender_cpy, server_address, MessageType::ReadyEvent, vec![state_cpy.lock().unwrap().get_id() as u8]);
            }
        }
        while state_cpy.lock().unwrap().get_game_started() == false {
            std::thread::sleep(time::Duration::from_millis(100));
        }
        loop {
        if state_cpy.lock().unwrap().get_game_over() == true {
            println!("Game over!");
            break;
        }
        println!("Guess a letter");
        let mut word_guess = String::new();
        std::io::stdin().read_line(&mut word_guess).expect("Failed to read line");
        word_guess = word_guess.trim().to_string();
        game_cpy.lock().unwrap().guess(&word_guess);
        let mut word_vec = word_guess.chars().collect::<Vec<char>>().iter().map(|c| *c as u8).collect::<Vec<_>>();
        let mut payload = vec![state_cpy.lock().unwrap().get_id() as u8];
        payload.append(&mut word_vec);
        send_packet(&sender_cpy, server_address, MessageType::GuessEvent, payload);
        game_cpy.lock().unwrap().print_word();

        if game_cpy.lock().unwrap().check_win() {
            // let id = state_cpy.lock().unwrap().get_id();
            send_packet(&sender_cpy, server_address, MessageType::FinishEvent, vec![state_cpy.lock().unwrap().get_id()]);
            break;
        }
        if game_cpy.lock().unwrap().check_lose() {
            send_packet(&sender_cpy, server_address, MessageType::LoseEvent, vec![state_cpy.lock().unwrap().get_id()]);
            break;
        }
        
        // std::thread::sleep(time::Duration::from_millis(10));
    }
    });
    
    loop {
        if state.clone().lock().unwrap().get_game_over() == true {
            return Ok(());
        }
        socket.manual_poll(Instant::now());
        // let mut should_update = false;
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == server_address {
                        handle_packet(&packet, game.clone(), state.clone());
                    }
                },
                _ => {}
            }
        }
        std::thread::sleep(time::Duration::from_millis(100));
    }
}


pub fn client() {
    match main() {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}