# Litecraft
[![Discord](https://img.shields.io/discord/371055566480605184.svg)](https://discord.gg/qKjuDxx)

<img src="https://i.imgur.com/jVISi6u.png" width="600">

[Open source](https://en.wikipedia.org/wiki/Free_and_open-source_software), [clean room](https://en.wikipedia.org/wiki/Clean_room_design) implementation of [Minecraft](https://minecraft.net) Client. Although this implementation does not use any Mojang source code, you must have a valid Minecraft account to play. We do not support piracy in any way.

# Features
- [X] Better memory management.
- [X] Better rendering system.
- [ ] Rendering using shaders.
- [ ] All vanilla client features.
- [ ] Launcher with assets downloader [(WIP!)](https://github.com/Litecrafty/Launcher)
- [ ] Server side plugin (mod) API (Send resources from server to client)
- [x] More soon™ We have some crazy ideas... ;)

# Screenshots
<img src="https://i.imgur.com/7u4Zyy1.png" width="600">
<img src="https://cdn.discordapp.com/attachments/377277794595635210/377296799892766720/unknown.png" width="600">
<img src="https://cdn.discordapp.com/attachments/377277794595635210/377277937902419968/687474703a2f2f692e696d6775722e636f6d2f68465967334a752e706e67.png" width="600">

# Compile

 - Download Rust Nightly with rustup.
 - Install allegro with your package manager or build it (Also see the next section)
 - Build with cargo.
 - Run with our [Launcher](https://github.com/Litecrafty/Launcher) or manually with any client token:
```bash
cargo run -- client_token
```
 - Profit!

# Allegro on Windows
Litecraft works well with the official Allegro pre-compiled binaries. First, download the official binaries from http://liballeg.org. You'll want to match the ABI of your Rust installation. GNU ABI on 32 bit can load Allegro 32 bit MSVC binaries, but otherwise you'll want to match the platform and ABI exactly. Let's say you extract the binaries to C:/allegro. That directory will contain the include, bin and lib directories. To compile and run, do the following from the Litecraft source directory:

If you're using MSYS:
```bash
export ALLEGRO_INCLUDE_DIR=C:/allegro/include
export RUST_ALLEGRO_EXAMPLE_LINK_PATH=C:/allegro/lib
cargo build
```

If you're using cmd directly:
```cmd
set ALLEGRO_INCLUDE_DIR=C:/allegro/include
set RUST_ALLEGRO_EXAMPLE_LINK_PATH=C:/allegro/lib
cargo build
```

# Contributing
[Bug reports](https://github.com/Litecrafty/Litecraft/issues) and [pull requests](https://github.com/Litecrafty/Litecraft/pulls) are welcome on our GitHub. This project is intended to be a safe, welcoming space for collaboration and discussion, and contributors are expected to adhere to the [Contributor Covenant code of conduct](https://github.com/Litecrafty/Litecraft/blob/master/CONTRIBUTING.md), you can read it on your Language [here](https://www.contributor-covenant.org/translations.html).

# License
[Apache License 2.0](https://github.com/Litecrafty/Litecraft/blob/master/LICENSE)