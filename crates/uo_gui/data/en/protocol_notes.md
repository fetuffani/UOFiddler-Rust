## Update 5 – client 7.0.10.3 (samples/client 7.0.10.3.exe)
- Packet size table: .data 0x6ce150, 12 B/entry (word0: len | 0x8000=variable), ids 0x00-0xF6.
- S->C dispatcher: 0x4813a9 (lea eax,[id-0xb]; byte idx 0x4822c0; jmp [idx*4+0x482078]); 146 handlers, 92 ids -> default 0x481fa1.
- Net layer: 0x462c70 = "process received packet" (EasyUO's signature `5f 5e 5d 5b 59 c3 e8` matches at 0x4631ed/0x463386); 0x45dbe0 matches the send signature.
- Output: out/packets.md (tools/dump_handlers.py, make_packets_md.py).

## Update 6 – 0xB4 Target Object List (UltimaLive)
- Server->client 0xB4 (len var, handler 0x481b86 -> 0x472cf0 in client 7.0.10.3): [cmd][ushort len][byte allowGround][uint serial][ushort x,y,z][ushort count] + count*{ushort itemId,hue,xOff,yOff,zOff}.
- The stock client already handles it natively (reads count*5 words, sets globals 0xa71020/0xa73c38, callback 0xa73c40=0x472960, calls 0x53bd30 if 0xa176d0 exists).
- UltimaLive (praxiiz/modernuo) uses it for RadialTarget: itemId 0xA12, hue 35, circle via rasterCircle(radius). Packet definition credited to "hash" (RunUO.com). refs/ has the cloned repos.

## Update 7 – how client 7.0.10.3 draws 0xB4 (ghost items on the cursor)
- 0x472cf0 (handler) -> 0x53bd30(this=[0xa176d0], count, arr, x, y, z): recreates the global object [0xa176d4] (0x108 B; +0x100 -> array, +0x104 -> count) with 16 B entries:
  {int16 x=xOff-hdrX, int16 y=yOff-hdrY, int16 z=zOff-hdrZ, uint16 itemId @+6, uint16 hue @+8, dword 0 @+0xC}; sets flag this+0x14c=1.
  The same structure is filled by 0x53bab0 (loads a list from a file/resource 0xd/0xe -> multi-target / house preview 0x99) => 0xB4 reuses the multi preview.
- Per frame: 0x45d080 (if [0xa176d4]!=0) gets the mouse (0x5875a0) -> world tile (0x4e48a0/0x588160) -> 0x53b780(this, tileX, tileY).
- 0x53b780: for each entry allocates a 0xb0 B object and calls constructor 0x5a2a90(itemId, tileX+x, tileY+y, hue, flag) with z = terrainZ(cursor)+z(entry)+this[0x144]; registers via 0x44c1f0 => "ghost" items follow the cursor.
- Cleanup: 0x53bf30/0x53bf90 (frees the list when flag==1 and the target ends). Click: callback 0x472960 -> sends 0x6C (Target Response).

## Update 8 – completed names + undocumented packets
- Cross-checked sources: ServUO (Server/Network/PacketHandlers.cs + Packet classes), POL (docs.polserver.com/packets), SphereServer Source-X (src/common/sphereproto.h), UltimaLive. Outputs: out/names_sources.json, out/pol_index.txt, out/sphere_xcmd.txt, out/servuo_out.txt.
- Still without a source (client 7.0.10.3): 0x6A, 0x92, 0x94, 0xCD, 0xCE (0xB4 and 0x40-0x44 resolved).
- Client packet readers: 0x480570=u8, 0x480590=u16, 0x4806a0=u32 (all net->host), 0x4802c0=bytes(ptr,len), 0x4803d0=string, 0x480390=packet size.
- 0x6A (0x46a6a0): reads 2 bytes and discards them. 0xCD (0x46fe70): `ret`.
- 0x92 (0x469070): u32,u32,bytes -> 0x55de30(0xd,0xe,...) ; 0x94 (0x469130): u8,u32,bytes -> 0x55de30(0xf,0x10,...).
  0x55de30 = writes into an idx/mul pair (name table @0x6d34e4, stride 56: 0xd=multi.idx 0xe=multi.mul 0xf=skills.idx 0x10=skills.mul; fopen mode "r+b"; fseek/fread/fwrite/fclose). Only runs if [0xa73c20]!=0 (this global = the server's numeric IPv4, parsed at 0x54cff2) - the exact meaning of the condition is NOT confirmed.
- 0xCE (0x477930): u32,u32,16B,36B,u32,str,str -> 5 std::string (0x5364c0) -> 0x537690 (large routine, strings "default"/"noid", URLs my.uo.com/cgi-bin/gms_web.pl). Hypothesis: related to MyUO/web. Not confirmed.

## Update 9 – client->server packets with no ServUO entry
- Send: 0x45dbe0 = SendPacket(ptr) -> 0x462c00 (243 call sites). Each type has a "constructor" that calls 0x47fed0(buf,id); writers: 0x480050=u32, 0x4800c0=2 u32 (8B), 0x480160=fixed str, 0x47ff10=u8, 0x47ff70=u16(?). id->constructor map: out/id_builders.json (heuristic). tools/id_builders.py, callers_of.py.
- 0x84 (0x46eb00, called at 0x53e40b): id+char[30]+char[30]+u32+local IPv4 (0x461b50 = gethostname+gethostbyname) = 69B = Change Password.
- 0xF1 (0x4707c0; calls 0x58a858/0x58a8cf/0x58c31a): id+8B (time sync request; reply 0xF2 = 25B = id+3x8B). 0xF0 (0x46dbf0; call 0x58a2b3): batch of the movement queue (stride 0x30 @0xa6f760, head/tail 0xa6f718/0xa6f720): count + entries {2 qwords, u8,u8, u32(1|2), 3x s32}; only sends if [0xa17895] or [0xa73c20]!=0.
- 0xD9 (0x5463b0, via 0x46ec80<-0x5465bd): hardware survey 268B; throttled by HKLM\SOFTWARE\Origin Worlds Online\LastHarvestTime (RegQuery/RegSetValue).
- [0xa73c20] = IPv4 of the LoginServer (parses "%d.%d.%d.%d,%d" = ip,port from login.cfg at 0x54cff2; [0xa73c24]=port). The `!=0` condition shows up in 65 places (including 0x92/0x94 and the 0xF0 send) - the exact meaning is NOT confirmed.
- Constructors with no caller (dead code in this client): 0x4B, 0x6B, 0x52, 0x19.

## Update 10 – gump commands and VirtueGumpItem
- See out/gump_commands.md (27 commands; tools/gump_cmds.py). gumppic accepts hue= and class=; the only class: VirtueGumpItem (0x66e564) -> GumpPic extended with double click, per-level tooltip and sending 0xB1 (gump 0x1CD).
- mastergump = position memory shared between gumps (see out/gump_commands.md). noresize/picinpic/picinpichued/picinpicphued/gumppichued/gumppicphued exist in ClassicUO (refs/cuo ... PacketHandlers.cs ~7066-7100), not in the 7.0.10.3 parser.
