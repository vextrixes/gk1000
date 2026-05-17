pkgname=gk1000ctl
pkgver=1.0.1
pkgrel=1
pkgdesc="A simple cli tool to control the CZC GK1000 RGB"
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url="https://github.com/vextrixes/gk1000"
license=('Unlicense')
depends=(hidapi)
makedepends=(cargo
            hidapi)
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('247de48a56a0f00ef3e4be5995174be91e8f421ce12c59be76379c75cbbb44c0')
options=('!debug')

prepare(){
  cd "gk1000-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --offline
}


build() {
  cd "gk1000-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  export RUSTFLAGS="-l hidapi-hidraw -l udev"
  cargo build --offline --release --package gk1000ctl

  target/release/gk1000ctl completions bash > bash_gk1000ctl
  target/release/gk1000ctl completions zsh > _gk1000ctl
  target/release/gk1000ctl completions fish > gk1000ctl.fish
}

package() {
  cd "gk1000-$pkgver"

  install -Dm755 target/release/gk1000ctl "$pkgdir"/usr/bin/gk1000ctl
  install -Dm644 LICENSE.md "$pkgdir"/usr/share/licenses/gk1000ctl/LICENSE
  install -Dm644 data/gk1000.rules "$pkgdir"/usr/lib/udev/rules.d/70-gk1000.rules

  install -Dm644 bash_gk1000ctl "${pkgdir}/usr/share/bash-completion/completions/gk1000ctl"
  install -Dm644 _gk1000ctl "${pkgdir}/usr/share/zsh/site-functions/_gk1000ctl"
  install -Dm644 gk1000ctl.fish "${pkgdir}/usr/share/fish/vendor_completions.d/gk1000ctl.fish"
}
