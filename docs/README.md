# Litecraft
[![Discord](https://img.shields.io/discord/371055566480605184.svg)](https://discord.gg/qKjuDxx)

<img src="https://i.imgur.com/JwRuNEl.gif" width="600">

[Open source](https://en.wikipedia.org/wiki/Free_and_open-source_software), [clean room](https://en.wikipedia.org/wiki/Clean_room_design) implementation of [Minecraft](https://minecraft.net) Client. Although this implementation does not use any Mojang source code, you must have a valid Minecraft account to play. We do not support piracy in any way.

# Features
- [X] Native perfomance and better memory management. (Thanks to D Programming Language).
- [X] Better rendering system.
- [X] Multi-thread and low Heap allocations.
- [X] Rendering using shaders. [Shaders are faster than old OpenGL](https://thebookofshaders.com/00/).
- [X] Safe by design.
- [X] Supported by an awesome community.
- [X] Modern design (Design patterns and use of VAOs, VBOs, EBOs, etc), use of new interfaces and libraries (like OpenGL >= 3.0).
- [ ] All vanilla client features.
- [ ] Launcher with assets downloader [(WIP!)](https://github.com/Litecrafty/Launcher)
- [ ] Server side plugin (mod) API (Send resources from server to client)
- [ ] More soon™ We have some crazy ideas... ;)

# Screenshots
<img src="https://cdn.discordapp.com/attachments/377277794595635210/377296799892766720/unknown.png" width="600">
<img src="https://cdn.discordapp.com/attachments/377277794595635210/377277937902419968/687474703a2f2f692e696d6775722e636f6d2f68465967334a752e706e67.png" width="600">

# Clone and Compile

 - Install and enable Git LFS for assets and libraries download.
 - Clone the repository.
 - Download D Programming Language: `dub`, `ldc2` or any D compiler, and `gcc` as linker.
 - Install libraries: `glfw3`.
 - Open any Minecraft jar and copy `assets/minecraft` to Litecraft `resources` folder.
 - Build with dub.
 - Run with our [Launcher](https://github.com/Litecrafty/Launcher) or manually with any client token:
```bash
dub build && ./litecraft TESTTOKEN
```
 - Profit!

# F.A.Q

### I want to help! Where can I learn D?
Here: https://tour.dlang.org

### How long will it take you to program a usable version?
Although we have thousands of hours of work, we probably have many thousands more.

### Litecraft is supposed to be faster, why does not it support old computers with OpenGL 2.1?
OpenGL 3.X has been launched for more than 10 years, besides facilitating the development, it allows us to do things in a more efficient way that otherwise would not be possible. If you really want to use OpenGL 2.1 or 1.1, we recommend using Minecraft Vanilla, sorry :(

# Contributing
[Bug reports](https://github.com/Litecrafty/Litecraft/issues) and [pull requests](https://github.com/Litecrafty/Litecraft/pulls) are welcome on our GitHub. This project is intended to be a safe, welcoming space for collaboration and discussion, and contributors are expected to adhere to the [Contributor Covenant code of conduct](https://github.com/Litecrafty/Litecraft/blob/master/CONTRIBUTING.md), you can read it on your Language [here](https://www.contributor-covenant.org/translations.html).

# License
[Apache License 2.0](https://github.com/Litecrafty/Litecraft/blob/master/LICENSE)