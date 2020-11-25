# niconico_watcher_rust
This is a discord bot that watch niconico and notify a channel new videos, inspired by zen510's niconico_watcher (https://github.com/zen510/niconico-watcher).

## Usage
1. Copy ``.env-template`` to ``.env``
2. Write token, channel id, and keyword to ``.env``
3. Run the way one of that
* Run ``cargo run``
* Run ``cargo build --release``, and in the directory that .env file exists, run ``target/release/niconico-watcher-rust[.exe]``
