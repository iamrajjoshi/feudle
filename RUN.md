Commands
1. `git clone https://github.com/iamrajjoshi/feudle`
2. `cd feudle`
3. `cargo run --bin snek-server --release`
4. Open a new terminal window
5. `cargo run --bin snek-client`
6. `<insert ip of server from output from command 3` (address of the server)
7. Open a new terminal window
8. `cargo run --bin snek-client`

Two windows with open for two different clients for wordle.

We have the game working on a subnet between two computers (hardcoded the server ip to `192.168.0.110:8001`). If the server can't connect to that ip, it default to `127.0.0.1:8080`.
