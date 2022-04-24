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

// from https://github.com/scottluptowski/wordlet
pub fn dictionary_words(index: usize) -> String {
    let words = vec![
        "check".to_string(),
        "hunch".to_string(),
        "canoe".to_string(),
        "grunt".to_string(),
        "soapy".to_string(),
        "khaki".to_string(),
        "cheap".to_string(),
        "solid".to_string(),
        "force".to_string(),
        "droop".to_string(),
        "booby".to_string(),
        "sassy".to_string(),
        "totem".to_string(),
        "audio".to_string(),
        "agony".to_string(),
        "micro".to_string(),
        "lease".to_string(),
        "goody".to_string(),
        "banjo".to_string(),
        "inlay".to_string(),
        "scrub".to_string(),
        "brass".to_string(),
        "twine".to_string(),
        "forty".to_string(),
        "strut".to_string(),
        "finch".to_string(),
        "table".to_string(),
        "spite".to_string(),
        "flora".to_string(),
        "sworn".to_string(),
        "lilac".to_string(),
        "dicey".to_string(),
        "azure".to_string(),
        "early".to_string(),
        "shire".to_string(),
        "raspy".to_string(),
        "vicar".to_string(),
        "aider".to_string(),
        "snowy".to_string(),
        "nutty".to_string(),
        "stark".to_string(),
        "smear".to_string(),
        "clown".to_string(),
        "teeth".to_string(),
        "cycle".to_string(),
        "humid".to_string(),
        "dross".to_string(),
        "visit".to_string(),
        "floor".to_string(),
        "bezel".to_string(),
        "motif".to_string(),
        "scoop".to_string(),
        "quack".to_string(),
        "bloom".to_string(),
        "credo".to_string(),
        "abate".to_string(),
        "river".to_string(),
        "unwed".to_string(),
        "while".to_string(),
        "foray".to_string(),
        "coral".to_string(),
        "quail".to_string(),
        "shirk".to_string(),
        "today".to_string(),
        "ascot".to_string(),
        "wight".to_string(),
        "motto".to_string(),
        "hussy".to_string(),
        "dummy".to_string(),
        "clone".to_string(),
        "pesky".to_string(),
        "fudge".to_string(),
        "asset".to_string(),
        "chase".to_string(),
        "awake".to_string(),
        "trump".to_string(),
        "delve".to_string(),
        "flank".to_string(),
        "sorry".to_string(),
        "depot".to_string(),
        "posit".to_string(),
        "ozone".to_string(),
        "slave".to_string(),
        "preen".to_string(),
        "donut".to_string(),
        "nicer".to_string(),
        "pagan".to_string(),
        "drone".to_string(),
        "gassy".to_string(),
        "sepia".to_string(),
        "stony".to_string(),
        "bluer".to_string(),
        "bulky".to_string(),
        "snare".to_string(),
        "inbox".to_string(),
        "diver".to_string(),
        "jelly".to_string(),
        "corny".to_string(),
        "bliss".to_string(),
        "solar".to_string(),
        "civil".to_string(),
        "shelf".to_string(),
        "lunar".to_string(),
        "album".to_string(),
        "yacht".to_string(),
        "diner".to_string(),
        "medic".to_string(),
        "sprig".to_string(),
        "batch".to_string(),
        "wound".to_string(),
        "mammy".to_string(),
        "pence".to_string(),
        "quoth".to_string(),
        "vista".to_string(),
        "knoll".to_string(),
        "riper".to_string(),
        "plaza".to_string(),
        "stalk".to_string(),
        "bossy".to_string(),
        "gazer".to_string(),
        "tango".to_string(),
        "udder".to_string(),
        "elate".to_string(),
        "acrid".to_string(),
        "choke".to_string(),
        "jumpy".to_string(),
        "spike".to_string(),
        "snaky".to_string(),
        "pooch".to_string(),
        "knack".to_string(),
        "adopt".to_string(),
        "alert".to_string(),
        "grown".to_string(),
        "scout".to_string(),
        "churn".to_string(),
        "downy".to_string(),
        "latch".to_string(),
        "sweat".to_string(),
        "value".to_string(),
        "ladle".to_string(),
        "untie".to_string(),
        "grain".to_string(),
        "nadir".to_string(),
        "mushy".to_string(),
        "train".to_string(),
        "piper".to_string(),
        "shied".to_string(),
        "rough".to_string(),
        "evict".to_string(),
        "ruder".to_string(),
        "tweak".to_string(),
        "poker".to_string(),
        "annoy".to_string(),
        "wryly".to_string(),
        "valid".to_string(),
        "fifth".to_string(),
        "wooer".to_string(),
        "arose".to_string(),
        "smile".to_string(),
        "lynch".to_string(),
        "truth".to_string(),
        "greed".to_string(),
        "fling".to_string(),
        "befit".to_string(),
        "prose".to_string(),
        "agape".to_string(),
        "bully".to_string(),
        "tying".to_string(),
        "pitch".to_string(),
        "model".to_string(),
        "caulk".to_string(),
        "frill".to_string(),
        "dingy".to_string(),
        "nobly".to_string(),
        "alibi".to_string(),
        "hurry".to_string(),
        "green".to_string(),
        "ionic".to_string(),
        "tease".to_string(),
        "speak".to_string(),
        "igloo".to_string(),
        "fetal".to_string(),
        "aunty".to_string(),
        "proud".to_string(),
        "gulch".to_string(),
        "pasty".to_string(),
        "blade".to_string(),
        "edify".to_string(),
        "badge".to_string(),
        "write".to_string(),
        "wench".to_string(),
        "human".to_string(),
        "slice".to_string(),
        "wooly".to_string(),
        "shrug".to_string(),
        "tulip".to_string(),
        "ruler".to_string(),
        "daisy".to_string(),
        "flier".to_string(),
        "chant".to_string(),
        "chore".to_string(),
        "rhyme".to_string(),
        "manor".to_string(),
        "error".to_string(),
        "swill".to_string(),
        "added".to_string(),
        "brisk".to_string(),
        "slope".to_string(),
        "slide".to_string(),
        "third".to_string(),
        "gauge".to_string(),
        "shock".to_string(),
        "suing".to_string(),
        "unzip".to_string(),
        "agora".to_string(),
        "swarm".to_string(),
        "ethic".to_string(),
        "betel".to_string(),
        "above".to_string(),
        "bawdy".to_string(),
        "lapse".to_string(),
        "brave".to_string(),
        "juice".to_string(),
        "woman".to_string(),
        "lipid".to_string(),
        "trial".to_string(),
        "lever".to_string(),
        "cabby".to_string(),
        "quote".to_string(),
        "tonal".to_string(),
        "quilt".to_string(),
        "flirt".to_string(),
        "doing".to_string(),
        "fairy".to_string(),
        "annex".to_string(),
        "dogma".to_string(),
        "adept".to_string(),
        "flail".to_string(),
        "rural".to_string(),
        "input".to_string(),
        "board".to_string(),
        "grave".to_string(),
        "terse".to_string(),
        "punch".to_string(),
        "missy".to_string(),
        "eerie".to_string(),
        "sandy".to_string(),
        "cliff".to_string(),
        "karma".to_string(),
        "borne".to_string(),
        "frown".to_string(),
        "uncut".to_string(),
        "space".to_string(),
        "whoop".to_string(),
        "fraud".to_string(),
        "wagon".to_string(),
    ];
    let word = &words[index];
    return word.clone();
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
    //print message type as a string
    let message_type_string = match message_type {
        x if x == MessageType::AssignIdEvent as u8 => "AssignIdEvent",
        x if x == MessageType::StartEvent as u8 => "StartEvent",
        x if x == MessageType::GuessEvent as u8 => "GuessEvent",
        x if x == MessageType::EndEvent as u8 => "EndEvent",
        x  if x == MessageType::Heartbeat as u8 => "Heartbeat",
        _ => "Unknown",
    };
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
            let word = dictionary_words(index as usize);
            println!("Starting game with word {}", word);
            game.lock().unwrap().set_word(word.clone());
            state.lock().unwrap().set_word(word.to_uppercase());
            state.lock().unwrap().set_game_started(true);
            return true;
        },
        x if x == MessageType::GuessEvent as u8 => {
            let id = data[0];
            //get the guess
            let guess = String::from_utf8(data[1..].to_vec()).unwrap();
            // println!("Player {} guessed {}", id, guess);
            let mut color_vec = state.lock().unwrap().get_color_vec(guess.clone());
            print!("Opponent's Guess: {:?}\n", color_vec);
            io::stdout().flush().unwrap();
            //TODO: UPDATE OPPONENT GAME DANCE
            return true;
        },
        x if x == MessageType::EndEvent as u8 => {
            let id = data[0];
            state.lock().unwrap().set_game_over(true);
            if (id == state.lock().unwrap().get_id() as u8) {
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
    // socket = Socket::bind_with_config("127.0.0.1:8452", config).unwrap();

    // Tell server to add the client
    let server_address = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let (sender, receiver) = (
        socket.get_packet_sender(), socket.get_event_receiver());
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    socket.manual_poll(Instant::now());
    println!("Attempting to join server {}...", server_address);

    let game = Arc::new(Mutex::new(Feudle::new()));
    let state = Arc::new(Mutex::new(ClientState::new()));
    
    std::thread::sleep(time::Duration::from_millis(100));
    send_packet(&sender, server_address, MessageType::JoinEvent, vec![]);
    
    let game_cpy = game.clone();
    let sender_cpy = sender.clone();
    let state_cpy = state.clone();
    
    let _game_thread = thread::spawn(move || {
        while state_cpy.lock().unwrap().get_ready() == false {
            println!("Are you ready? (y/n)");
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
            let id = state_cpy.lock().unwrap().get_id();
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
        let mut should_update = false;
        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == server_address {

                        should_update = handle_packet(&packet, game.clone(), state.clone());
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