# 0xB9 - Enable Locked Client Features
Direction: server -> client. Size: 5 bytes (clients 6.0.14.2+) | 3 bytes (earlier).
Purpose: Informs the client of the feature flags that are unlocked.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB9 |
| 1 | 4 | u32 | flags | Feature flags (5 bytes). Clients earlier than 6.0.14.2: u16 at offset 1 (3 bytes) |

Notes: Bits (`FeatureFlags`):

| Value | Name | Value | Name |
|---|---|---|---|
| 0x00000001 | T2A | 0x00001000 | SeventhCharacterSlot |
| 0x00000002 | UOR | 0x00002000 | RoleplayFaces |
| 0x00000004 | UOTD | 0x00004000 | TrialAccount |
| 0x00000008 | LBR | 0x00008000 | LiveAccount |
| 0x00000010 | AOS | 0x00010000 | SA |
| 0x00000020 | SixthCharacterSlot | 0x00020000 | HS |
| 0x00000040 | SE | 0x00040000 | Gothic |
| 0x00000080 | ML | 0x00080000 | Rustic |
| 0x00000100 | EighthAge | 0x00100000 | Jungle |
| 0x00000200 | NinthAge | 0x00200000 | Shadowguard |
| 0x00000400 | TenthAge | 0x00400000 | TOL |
| 0x00000800 | IncreasedStorage | 0x00800000 | EJ |

The packet table fixes 5 bytes; the 3-byte variant only applies to older clients.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`SupportedFeatures`); `Server/ExpansionInfo.cs` (`FeatureFlags`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendSupportedFeature`)
- Source-X: `src/network/send.cpp` (`PacketEnableFeatures`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`EnableLockedFeatures`); `src/ClassicUO.Client/Game/Data/LockedFeatures.cs`
