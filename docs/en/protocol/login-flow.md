# Login flow (connection, account, game server, character and entering the world)

Multi-packet flow that takes the client from the initial TCP connection to the character inside the world. There are two connections: the first with the login server (account and server list) and the second with the game server (character list, selection or creation, and entering the world). The field-by-field layout of each packet is in the files under [packets/](packets/README.md); this document describes the order, the dependencies between packets, and the differences between implementations. Only what has no file of its own (the 4-byte seed) is detailed here.

Conventions: big-endian integers; offsets counted from byte 0 (id). `S->C` = server to client, `C->S` = client to server. General ids and sizes are in [packets.md](packets.md).

## 1. Overview and client versions

| Step | Connection | Packets |
|---|---|---|
| Seed | login | 4-byte seed or `0xEF` |
| Account | login | `0x80` (C->S), `0xA8` or `0x82` (S->C) |
| Server selection | login | `0xA0` (C->S), `0x8C` (S->C) |
| Login to the game server | game | seed = key from `0x8C`, `0x91` (C->S), `0xB9` and `0xA9` (S->C) |
| Character | game | `0x5D`, `0x00`, `0xF8`, `0x8D`, `0x83` (C->S); `0x85`, `0x86`, `0x53` (S->C) |
| Client version | game | `0xBD` (S->C request, C->S reply) |
| Entering the world | game | `0x1B` and following, ending with `0x55` |

Differences by client version, confirmed in the implementations:

| Version | Effect |
|---|---|
| Before 6.0.5.0 | The seed is sent alone (4 bytes), without `0xEF` and without a version. The server only learns the version after `0xBD` |
| 6.0.5.0 or later | `0xEF` with seed and version (`major`, `minor`, `revision`, `prototype`) |
| Before 6.0.14.2 | `0xB9` with 3 bytes (flags `u16`) |
| 6.0.14.2 or later | `0xB9` with 5 bytes (flags `u32`) |
| 7.0.0.0 or later | `genderRace` values with gargoyle (see [c2s-0x00](packets/c2s-0x00.md)) |
| 7.0.13.0 or later | `0xA9` with 89-byte cities and the `lastSlot` field |
| 7.0.16.0 or later | Character creation through `0xF8` (4 skills) |
| Enhanced Client | Creation through `0x8D`; major version `>= 67` is treated as Enhanced by ServUO |

Detection in ServUO: the `NetState.Version` setter selects the set of `ProtocolChanges` (`NewCharacterList` from 7.0.13.0, `NewCharacterCreation` from 7.0.16.0, `ExtendedSupportedFeatures` from 6.0.14.2, `StygianAbyss` from 7.0.0.0).

## 2. Sequence

### 2.1 Login server

1. The client opens the connection and sends the seed: [`0xEF`](packets/c2s-0xef.md) (6.0.5.0+) or the 4-byte seed (section 3.1). A zero seed is invalid and the connection is closed.
2. C->S [`0x80`](packets/c2s-0x80.md) with account and password. ClassicUO sends the seed and the `0x80` in sequence, without waiting for a reply.
3. The server validates the account. On success, S->C [`0xA8`](packets/s2c-0xa8.md) (server list). On failure, S->C [`0x82`](packets/s2c-0x82.md) with the reason, and the connection is closed (ServUO, ModernUO).
4. C->S [`0xA0`](packets/c2s-0xa0.md) with the index of the chosen server.
5. S->C [`0x8C`](packets/s2c-0x8c.md) with IPv4 address, port and authentication key (`authId`).

### 2.2 Game server

1. The client closes the login connection and opens a new one to the address and port from `0x8C`. On the new connection it enables game compression and encryption, and first sends the 4 bytes of the key (the connection seed, big-endian, without id), then [`0x91`](packets/c2s-0x91.md) with the key repeated at offset 1, account and password (ClassicUO, `HandleRelayServerPacket`).
2. The server validates the key and the account. On refusal, ServUO and ModernUO close the connection without sending `0x82`.
3. On success, S->C [`0xB9`](packets/s2c-0xb9.md) (enabled features) and [`0xA9`](packets/s2c-0xa9.md) (characters, starting cities and flags). ServUO and ModernUO turn on outgoing compression at this point.
4. The client chooses one of the actions in section 2.3.

### 2.3 Character list

| Action | C->S | Server response |
|---|---|---|
| Enter with an existing character | [`0x5D`](packets/c2s-0x5d.md) | Section 2.4 |
| Create character | [`0x00`](packets/c2s-0x00.md), [`0xF8`](packets/c2s-0xf8.md) or [`0x8D`](packets/c2s-0x8d.md) | Section 2.4 on success; on refusal, [`0x53`](packets/s2c-0x53.md) or the connection is closed |
| Delete character | [`0x83`](packets/c2s-0x83.md) | [`0x86`](packets/s2c-0x86.md) with the updated list; on failure, [`0x85`](packets/s2c-0x85.md) with the reason (ServUO: followed by `0x86`) |

ClassicUO chooses the creation packet as follows: `0xF8` when the version is `>= CV_70160`, `0x00` otherwise (`Send_CreateCharacter`). ClassicUO does not send `0x8D`.

If the slot in `0x5D` is invalid or empty, ServUO and ModernUO close the connection. If another character on the account is already in the world, ServUO and ModernUO reply [`0x53`](packets/s2c-0x53.md) with `CharInWorld` and do not close the connection. In Source-X the equivalent (`CharIdle`) is returned as `0x82` with the reason `InUse` and closes the connection's read loop (`addLoginErr`); Source-X also sends a warning (`addIdleWarning`).

### 2.4 Client version and entering the world

1. If the server does not yet know the version (client without `0xEF`), S->C [`0xBD`](packets/s2c-0xbd.md) (`BD 00 03`) and C->S [`0xBD`](packets/c2s-0xbd.md) with the version as text. ServUO blocks processing of the client's other packets (`BlockAllPackets`), and a timer of 1 s (ServUO) or 64 ms (ModernUO) resumes the login when the version arrives. ModernUO always asks for the version, even when it already knows it.
2. S->C [`0x1B`](packets/s2c-0x1b.md) Login Confirm: serial, body, position, direction and map size.
3. World state, in the order of each server (section 2.5), including the map ([`0xBF/0x08`](packets/bf-0x08.md)), map patches ([`0xBF/0x18`](packets/bf-0x18.md)), features ([`0xB9`](packets/s2c-0xb9.md)), the player's own mobile ([`0x78`](packets/s2c-0x78.md) and [`0x20`](packets/s2c-0x20.md)), light ([`0x4F`](packets/s2c-0x4f.md) and [`0x4E`](packets/s2c-0x4e.md)), status ([`0x11`](packets/s2c-0x11.md)) and war mode ([`0x72`](packets/s2c-0x72.md)).
4. S->C [`0x55`](packets/s2c-0x55.md) Login Complete. ClassicUO only switches to the game scene when it receives `0x55`, and only if it has already created the player with `0x1B`.

Messages that ClassicUO sends on entering (client reference, not required by the reference servers):

| When | Packet |
|---|---|
| On receiving `0x1B` | [`0xBF/0x05`](packets/bf-0x05.md) (window size), [`0xBF/0x0B`](packets/bf-0x0b.md) (language), `0xBD` (version), [`0x09`](packets/c2s-0x09.md) (single click on the player's own character), [`0x34`](packets/c2s-0x34.md) (skills request) |
| On receiving `0x55` | [`0x34`](packets/c2s-0x34.md) (status and skills), [`0x06`](packets/c2s-0x06.md) (double click on the player's own character), [`0xBF/0x0F`](packets/bf-0x0f.md) (client flags), [`0xC8`](packets/c2s-0xc8.md) (view range) |

Some of these messages depend on the client version (`CV_200`, `CV_305D`, `CV_306E`, `CV_70796` in `EnterWorld` and `LoginComplete`). The packet called `Send_ClientType` in ClassicUO's code is `0xBF/0x0F`; ClassicUO does not send `0xE1`.

### 2.5 Send order on entering (by implementation)

ServUO (`PacketHandlers.DoLogin`):

1. `0x1B`
2. `0xBF/0x08` (if there is a map) and `0xBF/0x18`
3. `0xB9`
4. `0x78` of the player
5. `0x20` (`MobileUpdate` or `MobileUpdateOld` format depending on the version)
6. `SendEverything` (nearby items and mobiles)
7. `0x4F` and `0x4E` (`CheckLightLevels(true)`)
8. `0x55`
9. `0x78`, `0x11`, `0x72`, `0xBC`, `0x5B`, `0xBF/0x08`

ModernUO (`IncomingAccountPackets.DoLogin`):

1. `0x1B`
2. `0xBF/0x08` and `0xBF/0x18`
3. `0xBC` and `0xB9`
4. `0x20` (twice)
5. light (`0x4F`, `0x4E`) and `0x20`
6. `0x78`, `0x11` and `0x72`
7. `SendEverything`
8. `0xB9`, `0x20`, `0x11`, `0x72` and `0x78` again
9. `0x55`, `0x5B`, `0xBC`, `0xBF/0x08`
10. `0x6D` (region music)

Source-X (`CClient::addPlayerStart`):

1. `0x1B`
2. `0xBF/0x18` (always sent, even when empty)
3. `MoveToChar` and `Update` (packets for the player's own mobile and the surroundings; the exact ids were not traced)
4. `0x72`
5. `0x55`
6. `0x5B`
7. `0xBC`

It is not confirmed for Source-X where `0x4F`/`0x4E` fall in this sequence (`addLight` is called in `addPlayerView`).

## 3. Packets

### 3.1 4-byte seed (C->S, no id)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u32 | seed | Big-endian; zero is invalid |

Used by clients before 6.0.5.0 on the login connection. On the game connection ClassicUO sends the seed this way, with the `authId` from `0x8C` as the value. ModernUO treats the first bytes of any connection that does not start with `0xEF` as a 4-byte seed. ServUO distinguishes `0xEF` from a 4-byte seed by the first byte (`0xEF` = 239; the code comment says a normal seed does not start with this value). See `MessagePump.HandleSeed`.

In ClassicUO the seed value on the login connection is the local IPv4 of the socket (`NetClient.Socket.LocalIP`).

### 3.2 Packets with their own file

| Packet | Direction | Size | Role | File |
|---|---|---|---|---|
| `0xEF` Login Seed | C->S | 21 | seed and client version | [c2s-0xef](packets/c2s-0xef.md) |
| `0x80` Login Request | C->S | 62 | account, password and `nextLoginKey` | [c2s-0x80](packets/c2s-0x80.md) |
| `0xA8` Game Server List | S->C | `6 + 40 * count` | available servers | [s2c-0xa8](packets/s2c-0xa8.md) |
| `0x82` Login Denied | S->C | 2 | refusal reason | [s2c-0x82](packets/s2c-0x82.md) |
| `0xA0` Select Server | C->S | 3 | index in the `0xA8` list | [c2s-0xa0](packets/c2s-0xa0.md) |
| `0x8C` Connect To Game Server | S->C | 11 | IP, port and `authId` | [s2c-0x8c](packets/s2c-0x8c.md) |
| `0x91` Game Server Login | C->S | 65 | `authId`, account and password | [c2s-0x91](packets/c2s-0x91.md) |
| `0xB9` Enable Locked Client Features | S->C | 5 (3 in old clients) | feature flags | [s2c-0xb9](packets/s2c-0xb9.md) |
| `0xA9` Characters + Starting Locations | S->C | var | slots, cities and flags | [s2c-0xa9](packets/s2c-0xa9.md) |
| `0x5D` Play Character | C->S | 73 | character slot | [c2s-0x5d](packets/c2s-0x5d.md) |
| `0x00` Create Character | C->S | 104 | classic creation | [c2s-0x00](packets/c2s-0x00.md) |
| `0xF8` CreateCharacter70160 | C->S | 106 | creation 7.0.16.0+ | [c2s-0xf8](packets/c2s-0xf8.md) |
| `0x8D` ECCreateCharacter | C->S | var (146) | creation through the Enhanced Client | [c2s-0x8d](packets/c2s-0x8d.md) |
| `0x83` DeleteCharacter | C->S | 39 | deletion by slot | [c2s-0x83](packets/c2s-0x83.md) |
| `0x85` Character Delete Result | S->C | 2 | failure reason | [s2c-0x85](packets/s2c-0x85.md) |
| `0x86` Character List Update | S->C | var | list after creating or deleting | [s2c-0x86](packets/s2c-0x86.md) |
| `0x53` Reject Character Logon | S->C | 2 | refusal when entering with a character | [s2c-0x53](packets/s2c-0x53.md) |
| `0x81` Change Character | S->C | var | account's character list for switching | [s2c-0x81](packets/s2c-0x81.md) |
| `0xBD` Client Version | S->C / C->S | 3 / var | version request and reply | [s2c-0xbd](packets/s2c-0xbd.md), [c2s-0xbd](packets/c2s-0xbd.md) |
| `0xBE` Assist Version | S->C / C->S | 7 / var | assistant version | [s2c-0xbe](packets/s2c-0xbe.md), [c2s-0xbe](packets/c2s-0xbe.md) |
| `0xE1` ClientType | C->S | var | client type (KR, Enhanced) | [c2s-0xe1](packets/c2s-0xe1.md) |
| `0xA4` System Info | C->S | 149 | hardware (old) | [c2s-0xa4](packets/c2s-0xa4.md) |
| `0xD9` Spy on Client | C->S | 268 | hardware | [c2s-0xd9](packets/c2s-0xd9.md) |
| `0x1B` Login Confirm | S->C | 37 | entry confirmation | [s2c-0x1b](packets/s2c-0x1b.md) |
| `0x55` Login Complete | S->C | 1 | end of loading | [s2c-0x55](packets/s2c-0x55.md) |
| `0x78` Mobile Incoming | S->C | var | the player's own character with equipment | [s2c-0x78](packets/s2c-0x78.md) |
| `0x20` Draw Game Player | S->C | 19 | player's body, position and direction | [s2c-0x20](packets/s2c-0x20.md) |
| `0x4F` Overall Light Level | S->C | 2 | global light | [s2c-0x4f](packets/s2c-0x4f.md) |
| `0x4E` Personal Light Level | S->C | 6 | personal light | [s2c-0x4e](packets/s2c-0x4e.md) |
| `0x72` War Mode | S->C / C->S | 5 | war mode | [s2c-0x72](packets/s2c-0x72.md), [c2s-0x72](packets/c2s-0x72.md) |
| `0xBF/0x08` | S->C | 6 | current map | [bf-0x08](packets/bf-0x08.md) |
| `0xBF/0x18` | S->C | `9 + 8 * n` | map patches | [bf-0x18](packets/bf-0x18.md) |
| `0xBC` Seasonal Info | S->C | 3 | season | [s2c-0xbc](packets/s2c-0xbc.md) |
| `0x5B` Time | S->C | 4 | game time | [s2c-0x5b](packets/s2c-0x5b.md) |

`0xB8` (character profile) is a game packet and is not part of the login: see [c2s-0xb8](packets/c2s-0xb8.md) and [s2c-0xb8](packets/s2c-0xb8.md).

## 4. Data structures

### 4.1 Authentication key (`authId`)

A `u32` value created by the server when it receives `0xA0`, returned in `0x8C` and verified in `0x91`. The client also uses it as the seed of the game connection.

| Implementation | Generation | Verification in `0x91` |
|---|---|---|
| ServUO | Random in `[1, uint.MaxValue - 1]`, with bit 31 set 50% of the time; window of pending ids with fixed size (`m_AuthIDWindowSize`), the oldest is removed when the window is full | `authId` must be in the window (if `m_ClientVerification`), and must equal the connection seed. The id used is removed and the client version stored with it is restored |
| ModernUO | Random; zero and any value whose first byte is `0xEF` are excluded; valid for 2 minutes; bound to the account and the IP address of the one who received it | `authId` must equal the seed and belong to the same user and address. Within the time limit, it replaces the password check; expired, it falls back to the password check. Unknown id or one from another user/address: connection closed |
| Source-X | With `UseAuthID` on: CRC32 of `serverName + accountName`; off: fixed `0x7F000001` | Must match the value stored in the account (`customerid`); different: `BadAuthID` |

### 4.2 Character slots

Each slot takes 60 bytes in `0xA9` and `0x86`: 30 bytes of ASCII name and 30 zeroed bytes; a free slot = 60 zeros. ClassicUO only uses the first 30 bytes and ignores the rest. The slot index sent in `0x5D`, `0x00`/`0xF8`/`0x8D` and `0x83` is the index in this list. ClassicUO picks the first slot with an empty name to create a character.

### 4.3 Starting cities

The server sends the cities in `0xA9`; the chosen index comes back as `cityIndex` in the creation packets. ServUO and ModernUO reject (close the connection on) a `cityIndex` outside the connection's list (`state.CityInfo`). ServUO chooses the list in `AccountHandler.EventSink_GameLogin` (`SiegeStartingCities`, `StartingCitiesT2A`, `StartingCities` or `StartingCitiesSA`, depending on the shard and the expansion).

### 4.4 Server list

Each entry of `0xA8` is 40 bytes, and the index sent in `0xA0` is the index of the entry (`u16`). ServUO numbers from 0; Source-X numbers from 1 and limits the list to 32 entries (code comment: many servers can crash the client). ClassicUO sends the index as `u8` (`Send_SelectServer`: `u8 0x00` followed by the `u8` index).

## 5. Encryption and compression

### 5.1 Login (login connection)

- The seed is sent without encryption (ClassicUO: `skipEncryption`).
- The encryption of `0x80` and of the following packets on the login connection depends on the seed and on two keys derived from the client version. It only applies in the client-to-server direction; the login server's replies are not encrypted (ModernUO, `LoginEncryption.ServerEncrypt` does nothing).
- ModernUO (`LoginKeys.ComputeKeys` and `LoginEncryption`): `key1` and `key2` are computed from `major`, `minor` and `revision`; the state tables are initialized from the seed (`table1 = ((~seed ^ 0x00001357) << 16) | ((seed ^ 0xFFFFAAAA) & 0xFFFF)`, `table2 = ((seed ^ 0x43210000) >> 16) | ((~seed ^ 0xABCDFFFF) & 0xFFFF0000)`), and each byte is XORed with the low byte of `table1`, which then advances. The validation of an encrypted login is: byte 0 = `0x80` and bytes 30 and 60 = `0x00` after decryption.
- ClassicUO chooses the encryption type by version (`EncryptionHelper.CalculateEncryption`): `OLD_BFISH`, `BLOWFISH__1_25_36`, `BLOWFISH` and `BLOWFISH__2_0_3` for old versions and `TWOFISH_MD5` for versions above 2.0.3.
- Clients before 6.0.5.0 do not send the version in the seed: ModernUO tries all known keys (`LoginKeys.LegacyKeys`), and Source-X tries all versions on the first message (`m_Crypt.Init`, `xProcessClientSetup`).

### 5.2 Game (game connection)

- The key comes from the connection seed (the `authId` from `0x8C`). ModernUO (`GameEncryption`): 16-byte Twofish key with the seed repeated 4 times (big-endian); a 256-byte table (values 0 to 255) is encrypted with Twofish and renewed every 256 bytes received; client to server = XOR with the table; server to client = XOR with the 16 bytes of `MD5(table)`, in rotation.
- ModernUO: the encryption of `0x91` is detected from the first bytes of the message after decryption (id `0x91` and `authId` equal to the seed).
- ClassicUO: on the game connection, `Encryption.Initialize(false, seed)` and `EnableCompression()` are called right after connecting; the client only decrypts incoming data when compression is on and the type is Twofish/MD5.
- Source-X: when sending `0x8C`, it already switches the connection's encryption to game mode (`PacketServerRelay::onSent`, `InitFast(customerId, CONNECT_GAME)`), in case the client stays on the same connection.

### 5.3 Compression

- S->C, after game login is accepted: Huffman compression of all server traffic (ServUO and ModernUO: `CompressionEnabled = true` in `GameLogin`; Source-X compresses every packet when `GetConnectType() == CONNECT_GAME`).
- Output order (ModernUO): compress first, then encrypt.

### 5.4 Encrypted clients

- ServUO does not support encrypted clients: if the first packet is not one of the ids `0xF0`, `0xF1`, `0xCF`, `0x80`, `0x91`, `0xA4`, `0xEF`, `0xE4`, `0xFF`, the connection is closed (`MessagePump.CheckEncrypted`). The ids `0x80` and `0x91` are the login and game login ones in plain form.
- ModernUO accepts encrypted and unencrypted clients by default (`network.encryptionMode = Both`, modes `Unencrypted`, `Encrypted`, `Both`, `None`).
- Source-X accepts according to `USECRYPT` and `USENOCRYPT` in the configuration file.

## 6. Server rules

1. The first packet of the connection is the seed. Zero seed: close the connection.
2. Accept a single `0x80` per login connection and a single `0x91` per game connection (ServUO: `SentFirstPacket`; ModernUO also refuses a second `0xA0` and a second seed on the same connection).
3. Register the `0x8C` key associated with the account (and, in ModernUO, with the IP) before sending the packet; `0x91` is only accepted if the key matches the connection seed.
4. The address of each server in the list is the server's decision. ServUO returns the local socket address and replaces it with the configured public address when the client is neither loopback nor on a private network (`Scripts/Misc/ServerList.cs`).
5. In `0xA8` and `0x8C` respect the byte order of the IP according to the client version (see the packet files).
6. Send `0xB9` before `0xA9` in game login.
7. Choose the format of `0xA9` (cities of 63 or 89 bytes) and the size of `0xB9` (3 or 5 bytes) by client version; if the version is unknown (client before 6.0.5.0), request `0xBD` before continuing the login.
8. Refuse entry if another character on the account is in the world (`0x53` with `CharInWorld`). If the chosen character already has a connection, ServUO and ModernUO close the previous connection.
9. On deletion: refuse a character in use, a character created too recently (`RestrictDeletion` and `DeleteDelay` in ServUO; `m_iMinCharDeleteTime` in Source-X), and an invalid index, with `0x85`. ServUO sends `0x86` after each `0x85` and after each successful deletion; Source-X only sends `0x85`.
10. `0x55` marks the end of the initial loading and comes after `0x1B` and the world state.

## 7. Example

Client 7.0.10.3 (`major = 7`, `minor = 0`, `revision = 10`, `prototype = 3`), account `user`, password `pass`. Illustrative example of the layouts. Repeated zeros are abbreviated as `00 x N`.

Login server, C->S:

```
EF                                  id
7F 00 00 01                         seed (example)
00 00 00 07  00 00 00 00            major = 7, minor = 0
00 00 00 0A  00 00 00 03            revision = 10, prototype = 3

80                                  id
75 73 65 72  00 x 26                "user" padded to 30 bytes
70 61 73 73  00 x 26                "pass" padded to 30 bytes
FF                                  nextLoginKey
```

S->C, one server in the list (IP in natural octet order, as Source-X and ClassicUO interpret it):

```
A8 00 2E 5D 00 01                   id, size 46, flags, count = 1
00 00                               index = 0
54 65 73 74 00 x 28                 "Test" padded to 32 bytes
00 00                               percentFull = 0, timezone = 0
7F 00 00 01                         IP
```

Selection and redirect:

```
A0 00 00                            C->S: server with index 0
8C 7F 00 00 01 0A 21 12 34 56 78    S->C: IP 127.0.0.1, port 2593 (0x0A21), authId 0x12345678
```

Game server (new connection), C->S:

```
12 34 56 78                         seed = authId (4 bytes, no id)
91 12 34 56 78                      id and authId
75 73 65 72  00 x 26                account
70 61 73 73  00 x 26                password
```

Login refusal (login server), S->C, wrong password in ServUO (`BadPass = 0x03`):

```
82 03
```

Entering the world, S->C (`0x1B`): player `0x00000007`, body `0x0190`, position (1500, 1600, 0), direction 0, map 6144 x 4096:

```
1B  00 00 00 07  00 00 00 00  01 90  05 DC  06 40  00 00  00 00
    FF FF FF FF  00 00 00 00  18 00  10 00  00 00 00 00 00 00
```

End of loading: `55`.

## 8. Pitfalls

- The game connection starts with 4 bytes of seed (the `authId`), without an id, and only after that comes `0x91`. In ModernUO, a connection that does not start with `0xEF` has its first 4 bytes read as the seed; in ServUO and ModernUO the `authId` of `0x91` must equal that seed.
- The `authId` must not start with the byte `0xEF` (ModernUO): the server would read the first byte as the `0xEF` packet and wait for bytes that never arrive.
- The index sent in `0xA0` is the one that came in `0xA8` (base 0 in ServUO, base 1 in Source-X). Whoever builds the list decides the base.
- The IP in `0xA8` is not used to connect (the real connection comes from `0x8C`). The IP in `0x8C` is in natural octet order.
- A client before 6.0.5.0 does not report its version in the seed. ServUO only requests `0xBD` after `0x5D` or after character creation, and uses the packet format for the version it knows (`NetState.Version`) when it sends `0xB9` and `0xA9`.
- ServUO blocks the client's other packets while it waits for `0xBD` (`BlockAllPackets`); late client replies delay the player's entry.
- `0xA9` does not have the `lastSlot` field in clients before 7.0.13.0 and uses 63-byte cities.
- In `0x5D` the slot comes from the client: validate it against the size of the account's list (ServUO, ModernUO and Source-X reject an invalid index in different ways).
- ServUO writes the IP in `0xA8` in reversed order; clients from 4.0.0 onward expect natural order (see [s2c-0xa8](packets/s2c-0xa8.md)).
- Some packets are sent more than once on entering: ModernUO sends `0x20` four times and `0xB9`, `0x11`, `0x72` and `0x78` twice; ServUO sends the player's `0x78` twice.

## 9. Differences between implementations

| Aspect | ServUO | ModernUO | Source-X | ClassicUO (client) |
|---|---|---|---|---|
| Seed | `0xEF` by the first byte; otherwise 4 bytes | `0xEF` or 4 bytes; zero seed banned | `0xEF` or 4 bytes; KR uses seed `0xFFFFFFFF` and expects the server to send `0xE3` | Seed = local IPv4; `0xEF` from `CV_6040` (the [c2s-0xef](packets/c2s-0xef.md) file cites 6.0.5.0 as the limit) |
| `0x82` reasons | `Invalid` 0, `InUse` 1, `Blocked` 2, `BadPass` 3, `Idle` 0xFE, `BadComm` 0xFF | Same values (`ALRReason`) | `Invalid` 0, `InUse` 1, `Blocked` 2, `BadPass` 3, `Other` 4; the other internal reasons are translated to these | Maps the code to a message by position (0 to 8, values >= 9 use the last item) |
| Refusal in `0x91` | closes the connection | closes the connection | `addLoginErr` sends `0x82` with the translated reason and closes | n/a |
| Encryption | not supported | login: XOR; game: Twofish/MD5 | tries all versions; configurable | Blowfish and Twofish/MD5 by version |
| Compression S->C | Huffman after `0x91` | Huffman after `0x91` | Huffman on game connections | decompresses after `0x8C` |
| `0xB9` format | 3 or 5 bytes by version (`ExtendedSupportedFeatures`) | `SendSupportedFeature` | 4 bytes if the version is `>= MINCLIVER_EXTRAFEATURES` | reads 4 bytes from `CV_60142` |
| Creation `0x8D` | registered | not registered | registered (146 bytes) | does not send |
| `0xBD` request | only if the version is unknown | always when selecting or creating | on login (if the reported version is 0) and on game login when needed | answers the request |
| Entry order | section 2.5 | section 2.5 | section 2.5 | waits for `0x1B` and `0x55` |

Particularities:

- ClassicUO handles `0x82`, `0x85` and `0x53` in the same handler and chooses the message by the packet id and the code (`ServerErrorMessages.GetError`).
- In ModernUO, after `0xA0` the login server disconnects the client (`ProtocolState.LoginServer_AwaitingServerSelect`). If the client keeps sending on the same connection, the state goes back to `AwaitingSeed`, except when compiled with `STRICT_UO_PROTOCOL` (code comment: "CUO/Orion do not reconnect").
- Source-X subtracts `0x80` from the `0xA0` index when it is `>= 0x80` (code comment: "Sometimes we get an extra 0x80") and subtracts 1 from the base-1 index (`Login_Relay`).
- ClassicUO has a handler for `0xFD` (`LoginDelay`: `u8 delay`, shows the interval `(delay - 1) * 10` to `delay * 10` in the `0x53` message with code 13 or 14). No reference server implementation checked sends `0xFD`.

## 10. Not specified

- Layout and meaning of `0xFD` (`LoginDelay`): only ClassicUO handles it; no reference server sends it.
- Content of `0xE3` (KR encryption) beyond the sending of a fixed 77-byte block by Source-X (`PacketKREncryption`) and the `KREncryptionResponse` handler in ClassicUO.
- Position of `0x4F` and `0x4E` in the Source-X entry sequence.
- Exact ids of the mobile and surrounding packets sent by `MoveToChar`/`Update` in Source-X on entering.
- Full key derivation algorithm for Blowfish and for encrypted login in old versions in ClassicUO (`LoginCryptBehaviour`, `BlowfishBehaviour`) and in Source-X (`CCrypto*`).
- Meaning of the `nextLoginKey` field of `0x80` (value from `uo.cfg`; ignored by the servers; ClassicUO sends `0xFF`).
- Purpose of `0xBE`, `0xA4` and `0xD9`: the reference servers discard the content.
- Meaning of the `Unk1`, `Unk2`, `Unk3` and `Unk4` bits of the `0xA9` flags.

## 11. Implementation reference

| Part | File / symbol |
|---|---|
| ServUO: seed and input queue | `Server/Network/MessagePump.cs` (`HandleSeed`, `CheckEncrypted`, `HandleReceive`) |
| ServUO: handlers | `Server/Network/PacketHandlers.cs` (`LoginServerSeed`, `AccountLogin`, `AccountLogin_ReplyAck`, `AccountLogin_ReplyRej`, `PlayServer`, `GenerateAuthID`, `GameLogin`, `PlayCharacter`, `CreateCharacter`, `CreateCharacter70160`, `ECCreateCharacter`, `DeleteCharacter`, `ClientVersion`, `ClientType`, `LoginTimer`, `DoLogin`) |
| ServUO: packets | `Server/Network/Packets.cs` (`AccountLoginAck`, `AccountLoginRej`, `ALRReason`, `PlayServerAck`, `SupportedFeatures`, `CharacterList`, `CharacterListOld`, `CharacterListUpdate`, `DeleteResult`, `PopupMessage`, `ClientVersionReq`, `LoginConfirm`, `LoginComplete`, `MapChange`, `MapPatches`, `SetWarMode`, `CurrentTime`) |
| ServUO: client state | `Server/Network/NetState.cs` (`ProtocolChanges`, `Version`, `IsEnhancedClient`, `CompressionEnabled`), `Server/Network/Compression.cs` |
| ServUO: accounts and server list | `Scripts/Accounting/AccountHandler.cs` (`EventSink_AccountLogin`, `EventSink_GameLogin`, `EventSink_DeleteRequest`), `Scripts/Misc/ServerList.cs` (`EventSink_ServerList`), `Scripts/Mobiles/PlayerMobile.cs` (`CheckLightLevels`) |
| ModernUO: handlers | `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`LoginServerSeed`, `AccountLogin`, `CompleteAccountLogin`, `PlayServer`, `RegisterAuthId`, `ConsumeAuthId`, `GameLogin`, `PlayCharacter`, `CreateCharacter`, `DeleteCharacter`, `ClientVersion`, `ClientType`, `LoginTimer`, `DoLogin`) |
| ModernUO: packets | `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendPlayServerAck`, `SendLoginConfirmation`, `SendLoginComplete`, `SendClientVersionRequest`, `SendCharacterDeleteResult`, `SendCharacterListUpdate`) |
| ModernUO: state machine and encryption | `Projects/Server/Network/NetState/NetState.cs` (`ProtocolState`, seed handling, sending with compression and encryption), `Projects/Server/Network/Encryption/` (`EncryptionManager`, `LoginEncryption`, `LoginKeys`, `GameEncryption`, `TwofishEngine`), `Projects/Server/Network/NetworkCompression.cs` |
| Source-X | `src/network/CNetworkInput.cpp` (seed and first message), `src/network/CNetworkOutput.cpp` (output compression and encryption), `src/network/receive.cpp` (`PacketServersReq`, `PacketServerSelect`, `PacketCharListReq`, `PacketCharPlay`, `PacketCreate`, `PacketCreateHS`, `PacketCreateNew`, `PacketCharDelete`, `PacketClientVersion`), `src/network/send.cpp` (`PacketServerList`, `PacketServerRelay`, `PacketLoginError`, `PacketDeleteError`, `PacketCharacterListUpdate`, `PacketEnableFeatures`, `PacketPlayerStart`, `PacketLoginComplete`, `PacketWarMode`, `PacketGameTime`, `PacketEnableMapDiffs`, `PacketKREncryption`), `src/game/clients/CClientLog.cpp` (`Login_ServerList`, `Login_Relay`, `addRelay`, `addLoginErr`, `xProcessClientSetup`, `xCanEncLogin`), `src/game/clients/CClientMsg.cpp` (`Setup_ListReq`, `Setup_Play`, `Setup_Start`, `Setup_Delete`, `LogIn`, `addPlayerStart`, `addDeleteErr`), `src/common/crypto/` (`CCrypto*`, `CHuffman`) |
| ClassicUO | `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`OnNetClientConnected`, `ServerListReceived`, `SelectServer`, `HandleRelayServerPacket`, `ParseCharacterList`, `ParseCities`, `ReceiveCharacterList`, `UpdateCharacterList`, `HandleErrorCode`, `HandleLoginDelayPacket`), `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_Seed`, `Send_Seed_Old`, `Send_FirstLogin`, `Send_SelectServer`, `Send_SecondLogin`, `Send_CreateCharacter`, `Send_DeleteCharacter`, `Send_SelectCharacter`, `Send_ClientVersion`, `Send_ClientType`, `Send_GameWindowSize`, `Send_Language`, `Send_ClientViewRange`), `src/ClassicUO.Client/Network/PacketHandlers.cs` (`EnterWorld`, `LoginComplete`, `ServerListReceived`, `ReceiveServerRelay`, `ReceiveCharacterList`, `UpdateCharacterList`, `ReceiveLoginRejection`, `LoginDelay`, `EnableLockedFeatures`, `ClientVersion`), `src/ClassicUO.Client/Network/NetClient.cs` (`Send`, `ProcessEncryption`), `src/ClassicUO.Client/Network/Encryption/Encryption.cs` (`EncryptionHelper`), `src/ClassicUO.Client/Game/Data/ServerErrorMessages.cs` |
