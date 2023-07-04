# niconico_watcher_rust
This is a discord and misskey bot that watch niconico and notify a channel new videos, inspired by `zen510` and `nirsmmy` 's niconico_watcher.

## Usage
- With ``cargo run``
    1. Copy ``bot_config-template.toml`` to ``bot_config.toml``
    2. Get tokens and channel id from discord dev portal and fill all fields of the ``bot_config.toml`` with them
    3. Run ``cargo run --release``

- With binary built from ``cargo build --release``
    1. Copy ``bot_config-template.toml`` as ``bot_config.toml`` into the directory of your choice.
    2. Get tokens and channel id from discord dev portal and fill all fields of the ``bot_config.toml`` with them
    3. Run ``cargo build --release``
    4. Copy ``target/release/niconico-watcher-rust[.exe]`` to the directory.
    - Directory structure will:
        - bot_config.toml
        - ``niconico-watcher-rust[.exe]``

## Reference
- @zen510 @nirsmmy, ニコニコ動画用投稿通知 bot, https://github.com/zen510/niconico-watcher
- RSSフィード一覧, ニコニコ動画まとめwiki, http://nicowiki.com/RSS%E3%83%95%E3%82%A3%E3%83%BC%E3%83%89%E4%B8%80%E8%A6%A7.html