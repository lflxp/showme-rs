mod parse;
mod ping;
mod service;
mod waitgroup;
mod output;

pub use parse::*;
pub use ping::*;
pub use service::*;
pub use waitgroup::*;
pub use output::*;
pub const LOGO: &str = r"
 ▄▄▄▄    ██▓     ▄▄▄       ▄████▄   ██ ▄█▀ █     █░ ▄▄▄      ▄▄▄█████▓▓█████  ██▀███
▓█████▄ ▓██▒    ▒████▄    ▒██▀ ▀█   ██▄█▒ ▓█░ █ ░█░▒████▄    ▓  ██▒ ▓▒▓█   ▀ ▓██ ▒ ██▒
▒██▒ ▄██▒██░    ▒██  ▀█▄  ▒▓█    ▄ ▓███▄░ ▒█░ █ ░█ ▒██  ▀█▄  ▒ ▓██░ ▒░▒███   ▓██ ░▄█ ▒
▒██░█▀  ▒██░    ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▓██ █▄ ░█░ █ ░█ ░██▄▄▄▄██ ░ ▓██▓ ░ ▒▓█  ▄ ▒██▀▀█▄
░▓█  ▀█▓░██████▒ ▓█   ▓██▒▒ ▓███▀ ░▒██▒ █▄░░██▒██▓  ▓█   ▓██▒  ▒██▒ ░ ░▒████▒░██▓ ▒██▒
░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░░ ░▒ ▒  ░▒ ▒▒ ▓▒░ ▓░▒ ▒   ▒▒   ▓▒█░  ▒ ░░   ░░ ▒░ ░░ ▒▓ ░▒▓░
▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░  ░  ▒   ░ ░▒ ▒░  ▒ ░ ░    ▒   ▒▒ ░    ░     ░ ░  ░  ░▒ ░ ▒░
 ░    ░   ░ ░     ░   ▒   ░        ░ ░░ ░   ░   ░    ░   ▒     ░         ░     ░░   ░
 ░          ░  ░      ░  ░░ ░      ░  ░       ░          ░  ░            ░  ░   ░
      ░                   ░

Black Water v1.0.1
Asynchronous Port Scanner written in rust
https://github.com/lflxp/blackwater
";