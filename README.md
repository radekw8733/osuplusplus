# osu!reloaded
osu!reloaded is an unofficial client to osu! world. It's built on C++ and SDL2 to bring faster and better latency experience.

## Roadmap

- [ ] Basic osu experience required to play game
- [ ] Animations
- [ ] Uploading scores to server (maybe even that main)

## Installing

### Debian (and other debian-based distros like Ubuntu)

Grab latest build from Releases page or [Actions page](https://github.com/radekw8733/osu-reloaded/actions) (but with green checkmark) and then:
```
sudo apt install ./osureloaded.deb
```

### Other (compiling from source)

So far I only build debian packages so you must build osu reloaded from source.
Install mandatory dependencies:
```
sudo apt install cmake libsdl2-dev libsdl2-image-dev
```
Then clone repository:
```
git clone https://github.com/radekw8733/osu-reloaded.git
cd osu-reloaded
```
Finally compile it:
```
mkdir build
cd build
cmake ..
cmake --build .
./osureloaded
```
