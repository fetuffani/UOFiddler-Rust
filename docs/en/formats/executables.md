# Executables, libraries and auxiliary files

Files in the client directory that do not contain game data. None of them is interpreted by Fiddler; the Files tab classifies them as "Executable" or "Binary" and shows only hex.

## Clients

| File | Version | Role |
|---|---|---|
| `client.exe` | 7.0.23.1 | Classic client 7.0.23.1. `client.exe.bak` and the `client.exe` inside `client.zip` are identical copies. |
| `client 7.0.10.3.exe` | 7.0.10.3 | Client 7.0.10.3 (reference for packet and gump command tables in `docs/protocol`) |
| `client7.0.6.4.exe` | 7.0.6.4 | Client 7.0.6.4 |
| `client7.0.3.exe` | 7.0.3.0 | Client 7.0.3 |
| `client7.0.2.2.exe` | 7.0.2.2 | Client 7.0.2.2 |
| `Decrypted_client.exe` | 7.0.16.0 | Client 7.0.16 (decrypted copy) |
| `client6.0.1.10.exe`, `client601.exe` | 6.0.1.10 | Client 6.0.1.10 (identical files) |
| `LUO-Client.exe` | 6.0.1.10 | Client 6.0.1.10 (LUO variant) |
| `client_5.0.9.1.exe` | 5.0.9.1 | Client 5.0.9.1 |
| `Client 502.exe`, `Client502.exe` | n/a | Client 5.0.2 by name (identical files) |

## Launcher, patcher and utilities

| File | Role |
|---|---|
| `uo.exe` | Launcher: checks for updates and starts the client; manifest in `uo.exe.manifest` (requires administrator) |
| `UOPatch.exe` | Automatic patcher (`UO AUTOPATCH`) |
| `ClientPicker.exe` | Client selector for choosing which client to run |
| `Patchw32.dll` | RTPatch library (Pocket Soft) used to apply `.rtp` patches |
| `patch/` | Patch download folder (`.rtp`) |
| `patchlog` | Patcher log |
| `SetUOFolder.exe` | .NET utility that sets the Ultima Online installation folder |
| `uninstUOClassic.exe` | Uninstaller (NSIS) |
| `uotd.exe` | Auxiliary client executable |
| `Owo.exe` | Auxiliary executable with HTTP client |
| `Transerv.exe` | Speech translation server (uses `sjis2uni.mul`; configuration in `Uoupg.cfg`) |

## Libraries

| File | Role |
|---|---|
| `Binkw32.dll` | Bink video codec (playback of `music/Intro.bik`) |
| `Granny.dll` | Granny 3D (Rad Game Tools): animation and 3D model rendering |
| `zlib1.dll` | zlib compression |
| `dbghelp.dll` | Microsoft Debug Help Library |
| `Igrping.dll` | Ping library (`CIgr_pinger`) loaded by the client |
| `Igrping.dll.1`, `Igrping.dll.themida`, `Igrping.uog.dll`, `Igrping_UL.dll` | Variants of `Igrping.dll` |
| `GDF/UOClassicGDF.dll` | Game Definition File for the Windows Game Explorer; accompanies `UO-MCE.lnk`, `UO-MCE.mcl` and `UO-MCE.png` |

## Dependency tool

| File | Role |
|---|---|
| `depends.exe` | Dependency Walker 2.2 (64-bit) |
| `depends.dll` | Dependency Walker library |
| `depends.chm` | Dependency Walker help (Microsoft Compiled HTML Help) |

## Other auxiliary files

| File | Role |
|---|---|
| `uo.exe.manifest`, `client.manifest` | Windows application manifests |
| `client.zip` | ZIP archive containing `client.exe` |
| `regfile.reg` | Installation registry keys |
| `Logs/` | Client debug logs (`DebugLog.*`) |
| `music/Intro.bik` | Intro video (Bink) |
| `music/Digital/*.mp3` | MP3 soundtrack |
| `*.cfg`, `*.lst` | Configuration and list files; see `misc-binaries.md` |
