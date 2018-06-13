# kraken

## Building Kraken
1. Make sure to install Rust and Cargo via your favorite package manager.
2. Clone the repo and cd into it
3. `cargo run -- -l XX.XX.XX.XX:PORT YY.YY.YY.YY:PORT -c "UNIX COMMAND HERE" -u USERNAME -p PASSWORD`

## Running Kraken
1. Copy and paste the Kraken executable (`target/debug/kraken`) somewhere safe
2. Include Kraken into your path
3. `kraken -c <COMMAND> -l <IP_ADDRESS_LIST>... -p <PASSWORD> -u <USERNAME>`
