#!/bin/sh

CARGO="cargo +nightly"

publish() {
    pushd $1
    ${CARGO} publish
    popd
}


publish pqcrypto-traits-wasi
publish pqcrypto-internals-wasi
sleep 10
echo "Waiting a little bit for the pqcrypto-traits package to settle on crates.io"
publish pqcrypto-kyber-wasi
publish pqcrypto-frodo-wasi
publish pqcrypto-ntru-wasi
publish pqcrypto-ntruprime-wasi
publish pqcrypto-hqc-wasi
publish pqcrypto-sphincsplus-wasi
publish pqcrypto-saber-wasi
publish pqcrypto-dilithium-wasi
publish pqcrypto-falcon-wasi
publish pqcrypto-rainbow-wasi
publish pqcrypto-classicmceliece-wasi

echo "Waiting a little bit for the packages to settle on crates.io"

sleep 30
publish pqcrypto
