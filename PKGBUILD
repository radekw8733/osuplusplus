# Maintainer: Radosław Wolański <radekw8733@gmail.com>
pkgname="osupp"
pkgver="0.1.0"
pkgrel=1
pkgdesc="High performance osu! client written in C++"
arch=("x86_64")
makedepends=("sfml" "cmake")
source=("git+https://github.com/radekw8733/osuplusplus.git")
md5sums=("SKIP")

build() {
    cd "osuplusplus"
    git submodule update --init --recursive
    mkdir -p build
    cd build
    cmake ..
    cmake --build .
}

package() {
    cd "osuplusplus/build"
    make DESTDIR="$pkgdir/" install
}