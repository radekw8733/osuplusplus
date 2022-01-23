# Maintainer: Radosław Wolański <radekw8733@gmail.com>
pkgname="osureloaded"
pkgver="0.1.0"
pkgrel=1
pkgdesc="High performance osu! client written in C++"
arch=("x86_64")
makedepends=("sdl2" "sdl2_image" "cmake")
source=("git+https://github.com/radekw8733/osu-reloaded.git")
md5sums=("SKIP")

build() {
    cd "osu-reloaded"
    mkdir -p build
    cd build
    cmake ..
    cmake --build .
}

package() {
    cd "osu-reloaded/build"
    make DESTDIR="$pkgdir/" install
}