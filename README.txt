Install rust compiler:

1. Install curl if not installed:
sudo apt update
sudo apt install curl

2. Download and install rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Configure Your Environment:
source "$HOME/.cargo/env"

4. Verify installation:
rustc --version

to uninstall run: 
rustup self uninstall

to compile 'rustup' then 'cargo build'
