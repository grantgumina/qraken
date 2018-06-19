# Qraken

## Building Qraken
1. Make sure to install Rust and Cargo via your favorite package manager.
2. Clone the repo and cd into it
3. `cargo run -- -l XX.XX.XX.XX:PORT YY.YY.YY.YY:PORT -c "UNIX COMMAND HERE" -u USERNAME -p PASSWORD`

## Running Qraken
1. Copy and paste the Qraken executable (`target/debug/qraken`) somewhere safe
2. Include Qraken into your path
3. `Qraken -c <COMMAND> -l <IP_ADDRESS_LIST>... -p <PASSWORD> -u <USERNAME>`
