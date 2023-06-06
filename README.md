# osu!++
WARNING! This project is under heavy development and it is far from playable.<br/>
osu!++ is an unofficial client to osu! world. It's built with Bevy engine to bring fast and low latency experience.

## Installing
### Windows
osu++ doesn't have installer on Windows yet. Grab newest release from [here](https://nightly.link/radekw8733/osuplusplus/workflows/client_build/master/osuplusplus-windows.zip)

### Linux
It is planned to build Debian and Arch Linux packages. For now grab release from [here](https://nightly.link/radekw8733/osuplusplus/workflows/client_build/master/osuplusplus-linux.zip)

## Playing
To load your maps, simply drag them over game window. Note that on Wayland this feature doesn't work due to lacking `winit` feature implementation. Run game with `WINIT_UNIX_BACKEND=x11` environment variable