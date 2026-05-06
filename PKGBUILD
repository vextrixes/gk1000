pkgname=gk1000ctl
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple cli tool to control the CZC GK1000 RGB"
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url="https://github.com/vextrixes/gk1000"
license=('Unlicense')
depends=(hidapi)
makedepends=(cargo
            hidapi)
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('b0136c52e7b6356c64366330cf629f53a1a386fbff400a43c445ad106941134f')
options=('!debug')

prepare(){
  cd "gk1000-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked
}


build() {
  cd "gk1000-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  export RUSTFLAGS="-l hidapi-hidraw -l udev"
  cargo build --locked --release --package gk1000ctl
}

package() {
  cd "gk1000-$pkgver"

  install -Dm755 target/release/gk1000ctl "$pkgdir"/usr/bin/gk1000ctl
  install -Dm644 LICENSE.md "$pkgdir"/usr/share/licenses/gk1000ctl/LICENSE
  install -Dm644 data/gk1000.rules "$pkgdir"/usr/lib/udev/rules.d/70-gk1000.rules
}
