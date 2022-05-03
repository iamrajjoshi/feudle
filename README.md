# feudle

#### Group Name: nullptr

#### Names & Net Ids: Raj Joshi (rajj3) & Abhinil Dutt (abhinil2)

#### Project Introduction

* Description: A multiplayer version of Wordle
* Goals, Objectives and Why we chose this project:
  - Want to explore advanced concepts in rust
  - Utilize multithreading and networking in Rust
  - Building a interesting version of a popular game

#### System Overview

- Networking/Game Logic
  - Allow players to connect to server and have the server control overall competition state while the clients control the game state
  - **Task List**
    - Client
      - [x] Receiving and updating game state
      - [x] Serializing and sending game state
      - [x] Seperate thread for game loop
    - Server
      - [x] Handling client connections and disconnections
      - [x] Broadcast game state
      - [x] Determine when someone wins in a game

#### Possible Challenges

* Multithreading the game loop with updating game state

* Connecting networking and game logic to frontend

#### References

* [GitHub - imathur1/snek-game](https://github.com/imathur1/snek-game)

