# 0xCF - AccountLogin

Direction: client -> server. Size: 78 bytes (fixed). Available: before login.

Purpose: Account login in recent clients.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCF |
| 1 | 77 | u8[77] | data | Not specified |

Notes:

ServUO registers the id as variable-length and reads account[30] and password[30] from offset 3. ModernUO and Source-X do not handle this id.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`AccountLogin`)
