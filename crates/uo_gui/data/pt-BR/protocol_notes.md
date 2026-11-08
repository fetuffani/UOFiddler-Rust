## Atualização 5 – client 7.0.10.3 (samples/client 7.0.10.3.exe)
- Tabela de tamanhos de pacote: .data 0x6ce150, 12 B/entrada (word0: len | 0x8000=variável), ids 0x00-0xF6.
- Dispatcher S->C: 0x4813a9 (lea eax,[id-0xb]; byte idx 0x4822c0; jmp [idx*4+0x482078]); 146 handlers, 92 ids -> default 0x481fa1.
- Net layer: 0x462c70 = "processa pacote recebido" (a assinatura `5f 5e 5d 5b 59 c3 e8` do EasyUO bate em 0x4631ed/0x463386); 0x45dbe0 bate a sig de send.
- Saída: out/packets.md (tools/dump_handlers.py, make_packets_md.py).

## Atualização 6 – 0xB4 Target Object List (UltimaLive)
- Server->client 0xB4 (len var, handler 0x481b86 -> 0x472cf0 no client 7.0.10.3): [cmd][ushort len][byte allowGround][uint serial][ushort x,y,z][ushort count] + count*{ushort itemId,hue,xOff,yOff,zOff}.
- Cliente stock já trata nativamente (lê count*5 words, seta globais 0xa71020/0xa73c38, callback 0xa73c40=0x472960, chama 0x53bd30 se 0xa176d0 existir).
- UltimaLive (praxiiz/modernuo) usa para RadialTarget: itemId 0xA12, hue 35, círculo por rasterCircle(raio). Definição do pacote creditada a "hash" (RunUO.com). refs/ tem os repos clonados.

## Atualização 7 – como o client 7.0.10.3 desenha o 0xB4 (ghost items no cursor)
- 0x472cf0 (handler) -> 0x53bd30(this=[0xa176d0], count, arr, x, y, z): recria objeto global [0xa176d4] (0x108 B; +0x100 -> array, +0x104 -> count) com entradas de 16 B:
  {int16 x=xOff-hdrX, int16 y=yOff-hdrY, int16 z=zOff-hdrZ, uint16 itemId @+6, uint16 hue @+8, dword 0 @+0xC}; seta flag this+0x14c=1.
  Mesma estrutura é preenchida por 0x53bab0 (carrega lista de um arquivo/recurso 0xd/0xe -> multi-target / preview de casa 0x99) => 0xB4 reaproveita o preview de multi.
- Por quadro: 0x45d080 (se [0xa176d4]!=0) pega mouse (0x5875a0) -> tile do mundo (0x4e48a0/0x588160) -> 0x53b780(this, tileX, tileY).
- 0x53b780: p/ cada entrada aloca objeto 0xb0 B e chama construtor 0x5a2a90(itemId, tileX+x, tileY+y, hue, flag) com z = zTerreno(cursor)+z(entrada)+this[0x144]; registra via 0x44c1f0 => itens "fantasma" seguem o cursor.
- Limpeza: 0x53bf30/0x53bf90 (libera lista quando flag==1 e alvo termina). Clique: callback 0x472960 -> envia 0x6C (Target Response).

## Atualização 8 – nomes completados + pacotes sem documentação
- Fontes cruzadas: ServUO (Server/Network/PacketHandlers.cs + classes Packet), POL (docs.polserver.com/packets), SphereServer Source-X (src/common/sphereproto.h), UltimaLive. Saídas: out/names_sources.json, out/pol_index.txt, out/sphere_xcmd.txt, out/servuo_out.txt.
- Ainda sem fonte (cliente 7.0.10.3): 0x6A, 0x92, 0x94, 0xCD, 0xCE (0xB4 e 0x40-0x44 resolvidos).
- Leitores de pacote do client: 0x480570=u8, 0x480590=u16, 0x4806a0=u32 (todos net->host), 0x4802c0=bytes(ptr,len), 0x4803d0=string, 0x480390=tamanho do pacote.
- 0x6A (0x46a6a0): lê 2 bytes e descarta. 0xCD (0x46fe70): `ret`. 
- 0x92 (0x469070): u32,u32,bytes -> 0x55de30(0xd,0xe,...) ; 0x94 (0x469130): u8,u32,bytes -> 0x55de30(0xf,0x10,...).
  0x55de30 = escreve num par idx/mul (tabela de nomes @0x6d34e4, stride 56: 0xd=multi.idx 0xe=multi.mul 0xf=skills.idx 0x10=skills.mul; fopen modo "r+b"; fseek/fread/fwrite/fclose). Só executa se [0xa73c20]!=0 (esse global = IPv4 numérico do servidor, parseado em 0x54cff2) - significado exato da condição NÃO confirmado.
- 0xCE (0x477930): u32,u32,16B,36B,u32,str,str -> 5 std::string (0x5364c0) -> 0x537690 (rotina grande, strings "default"/"noid", URLs my.uo.com/cgi-bin/gms_web.pl). Hipótese: relacionado a MyUO/web. Não confirmado.

## Atualização 9 – pacotes cliente->servidor sem registro no ServUO
- Envio: 0x45dbe0 = SendPacket(ptr) -> 0x462c00 (243 call sites). Cada tipo tem uma "construtora" que chama 0x47fed0(buf,id); escritores: 0x480050=u32, 0x4800c0=2 u32 (8B), 0x480160=str fixa, 0x47ff10=u8, 0x47ff70=u16(?). Mapa id->construtora: out/id_builders.json (heurístico). tools/id_builders.py, callers_of.py.
- 0x84 (0x46eb00, chamada em 0x53e40b): id+char[30]+char[30]+u32+IPv4 local (0x461b50 = gethostname+gethostbyname) = 69B = Change Password.
- 0xF1 (0x4707c0; chamadas 0x58a858/0x58a8cf/0x58c31a): id+8B (time sync request; resposta 0xF2 = 25B = id+3x8B). 0xF0 (0x46dbf0; chamada 0x58a2b3): lote da fila de movimento (stride 0x30 @0xa6f760, head/tail 0xa6f718/0xa6f720): count + entradas {2 qwords, u8,u8, u32(1|2), 3x s32}; só envia se [0xa17895] ou [0xa73c20]!=0.
- 0xD9 (0x5463b0, via 0x46ec80<-0x5465bd): pesquisa de hardware 268B; throttle por HKLM\SOFTWARE\Origin Worlds Online\LastHarvestTime (RegQuery/RegSetValue).
- [0xa73c20] = IPv4 do LoginServer (parse "%d.%d.%d.%d,%d" = ip,porta do login.cfg em 0x54cff2; [0xa73c24]=porta). Condição `!=0` aparece em 65 pontos (inclui 0x92/0x94 e envio do 0xF0) - significado exato NÃO confirmado.
- Construtoras sem chamador (código morto neste cliente): 0x4B, 0x6B, 0x52, 0x19.

## Atualização 10 – comandos de gump e VirtueGumpItem
- Ver out/gump_commands.md (27 comandos; tools/gump_cmds.py). gumppic aceita hue= e class=; único class: VirtueGumpItem (0x66e564) -> GumpPic estendido com duplo clique, tooltip por nível e envio de 0xB1 (gump 0x1CD).
- mastergump = memória de posição compartilhada entre gumps (ver out/gump_commands.md). noresize/picinpic/picinpichued/picinpicphued/gumppichued/gumppicphued existem no ClassicUO (refs/cuo ... PacketHandlers.cs ~7066-7100), não no parser do 7.0.10.3.

