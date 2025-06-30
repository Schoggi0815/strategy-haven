# Strategy Haven

## How to run

### Game

1. Make sure you have Rust installed
2. Make sure you have Rust nightly installed
3. To run the game, you can just run `cargo run`

### Spacetime Server

1. Setup a spacetime server
   - The easiest way is to just install spacetime-db and run `spacetime start`
   - There is also a docker image, which you can run anywhere
2. Publish the server to a spacetime-db database
   - You can do this by running `spacetime publish -p server database-name`
