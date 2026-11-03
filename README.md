# DualCraft

Experimental **dual-protocol** Minecraft server written in Rust.

Supports both:
- **Java Edition** (TCP, modern protocol)
- **Bedrock Edition** (UDP / RakNet)

> This is a **scaffold / research project**, not a finished production server.
> Full Java + Bedrock compatibility is a very large undertaking (see Status below).

## Goals

- Single server process that accepts both Java and Bedrock clients
- Shared world / player management where possible
- Clean Rust architecture that can grow over time
- Learn and document the differences between the two protocols

## Current Status

| Feature                    | Status      | Notes                                      |
|---------------------------|-------------|--------------------------------------------|
| Project structure         | Done        | Workspace + modules                        |
| Java TCP listener         | Stub        | Accepts connections, basic handshake skeleton |
| Bedrock (RakNet) listener | Stub        | UDP socket ready, RakNet not fully implemented |
| Shared player/world state | Placeholder | Common types only                          |
| Full login / play         | Not started | —                                          |
| World generation / saving | Not started | —                                          |
| Geyser-style translation  | Not started | Possible future approach                   |

## Architecture

```
dualcraft/
├── crates/
│   ├── dualcraft          # main binary
│   ├── protocol-java      # Java Edition protocol
│   ├── protocol-bedrock   # Bedrock / RakNet protocol
│   └── common             # shared types, config, world stubs
├── Cargo.toml             # workspace
└── README.md
```

## Building

```bash
cargo build
cargo run -p dualcraft
```

Default ports (configurable later):
- Java: `25565` (TCP)
- Bedrock: `19132` (UDP)

## Why this is hard

Java and Bedrock are very different:
- Different transports (TCP vs UDP/RakNet)
- Completely different packet formats and login sequences
- Different world formats, entity systems, inventory, etc.
- Mojang does not provide an official unified server API

Most existing "Java + Bedrock" solutions use a **proxy/translator** (like Geyser) in front of a normal Java server instead of implementing both protocols natively.

This project experiments with a native dual approach.

## Licence

MIT

## Credits

Inspired by the wider Minecraft server ecosystem (Paper, Velocity, Geyser, etc.) and written as a learning / experimental project.
