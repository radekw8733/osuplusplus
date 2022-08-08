# osu!++ [Pre-alpha] - only use for testing
osu!++ is an unofficial client to osu! world. It's built using C++ and SFML to bring faster and better latency experience. It's graphics design is also modular so it should be easy to plug it in to other frameworks.

## Roadmap

- [ ] [WIP] Basic framework
- [ ] Windows support
- [ ] Loading osu files
- [ ] Implementation of osu logic
- [ ] Custom UI
- [ ] Animations
- [ ] Uploading scores to server (maybe even that main)

## Installing

## Windows
### [WIP]

## ~~Debian (and other debian-based distros like Ubuntu)~~ 

### ~~Grab latest build from Releases page or [Actions page](https://github.com/radekw8733/osu-reloaded/actions) (but with green checkmark) and then:~~
```
sudo apt install ./osureloaded.deb
```
### [DEBIAN BUILDS ARE CURRENTLY BROKEN]

## Arch linux
### [WIP]

## Other (compiling from source)

~~So far I only build debian packages~~ so you must build osu reloaded from source.
Install required dependencies:
```
sudo apt install cmake libsfml-dev
```
Then clone repository:
```
git clone --recursive https://github.com/radekw8733/osuplusplus.git
cd osuplusplus
```
Finally compile it:
```
mkdir build
cd build
cmake ..
cmake --build .
./osupp
```
