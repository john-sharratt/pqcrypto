/// Post-Quantum cryptographic primitives
///
/// Packages the [PQClean][pqclean] project as Rust crates
///
/// [pqclean]: https://github.com/PQClean/PQClean/
pub use pqcrypto_traits_wasi as traits;

pub mod prelude {
    pub use pqcrypto_traits_wasi::kem::{
        Ciphertext as _, PublicKey as _, SecretKey as _, SharedSecret as _,
    };
    pub use pqcrypto_traits_wasi::sign::{
        DetachedSignature as _, PublicKey as _, SecretKey as _, SignedMessage as _,
    };
}

pub mod kem {
    #[cfg(feature = "pqcrypto-classicmceliece-wasi")]
    pub use pqcrypto_classicmceliece_wasi::{
        mceliece348864, mceliece348864f, mceliece460896, mceliece460896f, mceliece6688128,
        mceliece6688128f, mceliece6960119, mceliece6960119f, mceliece8192128, mceliece8192128f,
    };
    #[cfg(feature = "pqcrypto-frodo-wasi")]
    pub use pqcrypto_frodo_wasi::{
        frodokem1344aes, frodokem1344shake, frodokem640aes, frodokem640shake, frodokem976aes,
        frodokem976shake,
    };
    #[cfg(feature = "pqcrypto-hqc-wasi")]
    pub use pqcrypto_hqc_wasi::{hqcrmrs128, hqcrmrs192, hqcrmrs256};
    #[cfg(feature = "pqcrypto-kyber-wasi")]
    pub use pqcrypto_kyber_wasi::{
        kyber1024, kyber102490s, kyber512, kyber51290s, kyber768, kyber76890s,
    };
    #[cfg(feature = "pqcrypto-ntru-wasi")]
    pub use pqcrypto_ntru_wasi::{ntruhps2048509, ntruhps2048677, ntruhps4096821, ntruhrss701};
    #[cfg(feature = "pqcrypto-ntruprime-wasi")]
    pub use pqcrypto_ntruprime_wasi::{
        ntrulpr1013, ntrulpr1277, ntrulpr653, ntrulpr761, ntrulpr857, ntrulpr953, sntrup1013,
        sntrup1277, sntrup653, sntrup761, sntrup857, sntrup953,
    };
    #[cfg(feature = "pqcrypto-saber-wasi")]
    pub use pqcrypto_saber_wasi::{firesaber, lightsaber, saber};
}

pub mod sign {
    #[cfg(feature = "pqcrypto-dilithium-wasi")]
    pub use pqcrypto_dilithium_wasi::{dilithium2, dilithium3, dilithium5};
    #[cfg(feature = "pqcrypto-falcon-wasi")]
    pub use pqcrypto_falcon_wasi::{falcon1024, falcon512};
    #[cfg(feature = "pqcrypto-rainbow-wasi")]
    pub use pqcrypto_rainbow_wasi::{
        rainbowicircumzenithal, rainbowiclassic, rainbowicompressed, rainbowiiicircumzenithal,
        rainbowiiiclassic, rainbowiiicompressed, rainbowvcircumzenithal, rainbowvclassic,
        rainbowvcompressed,
    };
    #[cfg(feature = "pqcrypto-sphincsplus-wasi")]
    pub use pqcrypto_sphincsplus_wasi::{
        sphincsharaka128frobust, sphincsharaka128fsimple, sphincsharaka128srobust,
        sphincsharaka128ssimple, sphincsharaka192frobust, sphincsharaka192fsimple,
        sphincsharaka192srobust, sphincsharaka192ssimple, sphincsharaka256frobust,
        sphincsharaka256fsimple, sphincsharaka256srobust, sphincsharaka256ssimple,
        sphincssha256128frobust, sphincssha256128fsimple, sphincssha256128srobust,
        sphincssha256128ssimple, sphincssha256192frobust, sphincssha256192fsimple,
        sphincssha256192srobust, sphincssha256192ssimple, sphincssha256256frobust,
        sphincssha256256fsimple, sphincssha256256srobust, sphincssha256256ssimple,
        sphincsshake256128frobust, sphincsshake256128fsimple, sphincsshake256128srobust,
        sphincsshake256128ssimple, sphincsshake256192frobust, sphincsshake256192fsimple,
        sphincsshake256192srobust, sphincsshake256192ssimple, sphincsshake256256frobust,
        sphincsshake256256fsimple, sphincsshake256256srobust, sphincsshake256256ssimple,
    };
}
