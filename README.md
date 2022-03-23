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
      - [ ] Receiving and updating game state
      - [ ] Serializing and sending game state
      - [ ] Seperate thread for game loop
    - Server
      - [ ] Handling client connections and disconnections
      - [ ] Broadcast game state
      - [ ] Determine when someone wins in a game
* Frontend
  
  * Present a UI for our game including components such as virtual keyboard, timer, settings and bridge the backend.
  
  * **Task List**
    
    - [ ] Feudle word boxes with accurate color transformations on word submission
    - [ ] Common timer for both the players
    - [ ] Game room with a ready button, to synchronize the start of the game when both players are ready
    - [ ] Virtual Keyboard attached at the bottom of the webpage

#### Possible Challenges

* Multithreading the game loop with updating game state

* Connecting networking and game logic to frontend

#### References

* [GitHub - imathur1/snek-game](https://github.com/imathur1/snek-game)

* 
