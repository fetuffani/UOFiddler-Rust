# 0xD4 - Book Header (new)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Opens a book with a title and an author of variable length.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD4 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the book |
| 7 | 1 | u8 | flag | 1 |
| 8 | 1 | u8 | writable | 1 if editable |
| 9 | 2 | u16 | pages | Number of pages |
| 11 | 2 | u16 | titleLength | Length of the title in bytes, including the NUL |
| 13 | titleLength | utf8 | title | Title in UTF-8, terminated by NUL |
| ... | 2 | u16 | authorLength | Length of the author, including the NUL |
| ... | authorLength | utf8 | author | Author in UTF-8, terminated by NUL |

Notes: Total size in ModernUO: `17 + bytes(title) + bytes(author)`. In older data 0x93 uses fixed fields.

## Implementation reference
- ServUO: `Scripts/Items/Books/BaseBook.cs`
- ModernUO: `Projects/UOContent/Items/Books/BookPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketDisplayBookNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenBook`)
