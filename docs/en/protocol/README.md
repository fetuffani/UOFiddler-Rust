# Documentation of the Ultima Online network protocol

Reference for the packets of client 7.0.x and for flows that combine several packets.

## Document index

### Tables
- [packets.md](packets.md): server->client and client->server packet tables (id, size, name) and the layouts of `0xB4`, `0x84`, `0xF1`/`0xF2`.
- [gump-layout.md](gump-layout.md): gump layout commands (`{ cmd arg ... }`) of the `0xB0` and `0xDD` packets.

- [packets/README.md](packets/README.md): field-by-field format of each packet and of each `0xBF`/`0xD7` subcommand.

### Multi-packet flows
- [house-customization.md](house-customization.md): custom house editor (`0xBF/0x20`, `0xBF/0x1D`, `0xBF/0x1E`, `0xD8`, `0xD7` subcommands), plan structure, compression and review.
- [boats-smooth-sailing.md](boats-smooth-sailing.md): boat movement (`0xF6`, `0xF7`, `0xBF/0x33`, `0x99`), speed, turning, helm, comparison with legacy movement.
- [gump-open-response.md](gump-open-response.md): gump opening (`0xB0`/`0xDD`), response (`0xB1`) with switches and text fields, server-side closing (`0xBF/0x04`).
- [login-flow.md](login-flow.md): login flow (seed `0xEF`, `0x80`, `0xA8`, `0xA0`, `0x8C`, `0x91`, `0xA9`, server selection, character creation and deletion, `0xBD`), encryption and compression, entering the world (`0x1B` to `0x55`).

## How to read

- Integers are big-endian on the network, except where indicated. `u8`, `u16`, `u32` = unsigned; `i8`, `i16`, `i32` = signed.
- Offsets are counted from byte 0 of the packet, which is the id. The "Size" column of [packets.md](packets.md) is the total number of bytes, including the id; `var` indicates variable size, and in that case bytes 1 and 2 carry the total size (`u16`).
- `0xBF` and `0xD7` are envelopes: `0xBF` = `[id][u16 size][u16 subcommand][data]`; `0xD7` = `[id][u16 size][u32 serial][u16 subcommand][arguments][0x0A]`. In the documents, a subcommand is written as `0xBF/0x20` or `0xD7/0x06`.
- `S->C` is server to client and `C->S` is client to server.
- Each flow document follows the same order: overview and client versions, numbered sequence of messages, byte-by-byte layout of the packets, data structure, server rules, example with bytes, common pitfalls, differences between implementations, unspecified items and implementation reference (file paths).
- Hexadecimal examples are separated by spaces, one byte per pair.
- Code identifiers (class names, method names, flags) are kept in English, as in the reference repositories.
- When one implementation differs from another (ServUO, ModernUO, Sphere/Source-X, ClassicUO), the difference is described in the document's own section.
