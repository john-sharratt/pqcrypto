extern crate cc;
extern crate glob;

use std::env;
use std::path::PathBuf;

fn main() {
    let common_dir: PathBuf = ["pqclean", "common"].iter().collect();
    let common_files = vec![
        common_dir.join("fips202.c"),
        common_dir.join("aes.c"),
        common_dir.join("sha2.c"),
        common_dir.join("randombytes.c"),
        common_dir.join("nistseedexpander.c"),
        common_dir.join("sp800-185.c"),
    ];

    cc::Build::new()
        .include(&common_dir)
        .files(common_files.into_iter())
        .compile("pqclean_common");

    let avx2_enabled = env::var("CARGO_FEATURE_AVX2").is_ok();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let is_windows = target_os == "windows";
    let is_macos = target_os == "macos";

    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-128f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128f-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-128f-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-128f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128f-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-128f-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-128s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128s-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-128s-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-128s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-128s-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-128s-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-192f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192f-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-192f-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-192f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192f-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-192f-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-192s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192s-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-192s-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-192s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-192s-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-192s-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-256f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256f-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-256f-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-256f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256f-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-256f-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-256s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256s-robust",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-256s-robust_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-haraka-256s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-haraka-256s-simple",
            "aesni",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-haraka-256s-simple_aesni");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-128f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-128f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-128f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-128f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-128s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-128s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-128s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-128s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-128s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-192f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-192f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-192f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-192f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-192s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-192s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-192s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-192s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-192s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-256f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-256f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-256f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-256f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-256s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-256s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-shake256-256s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-shake256-256s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-shake256-256s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-128f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-128f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-128f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-128f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-128s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-128s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-128s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-128s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-128s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-192f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-192f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-192f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-192f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-192s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-192s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-192s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-192s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-192s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256f-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-256f-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256f-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-256f-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256f-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-256f-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256f-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-256f-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256s-robust",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-256s-robust_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256s-robust",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-256s-robust_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }
    {
        let mut builder = cc::Build::new();
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256s-simple",
            "clean",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder.include(&common_dir).include(target_dir).files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile("sphincs-sha256-256s-simple_clean");
    }

    if avx2_enabled && target_arch == "x86_64" {
        let target_dir: PathBuf = [
            "pqclean",
            "crypto_sign",
            "sphincs-sha256-256s-simple",
            "avx2",
        ]
        .iter()
        .collect();
        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        let mut builder = cc::Build::new();

        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul")
                .flag("-msse2");
        }
        builder
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            )
            .compile("sphincs-sha256-256s-simple_avx2");

        let mut builder = cc::Build::new();
        if is_windows {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-mavx2");
        };
        builder
            .file(
                &common_dir
                    .join("keccak4x")
                    .join("KeccakP-1600-times4-SIMD256.c"),
            )
            .compile("keccak4x");
    }

    // Print enableing flag for AVX2 implementation
    if avx2_enabled && target_arch == "x86_64" {
        println!("cargo:rustc-cfg=enable_avx2");
    }
}
