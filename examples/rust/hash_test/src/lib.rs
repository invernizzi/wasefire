// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests that cryptographic hashes are working properly.

#![no_std]
wasefire::applet!();

#[cfg(not(feature = "rust-crypto"))]
use alloc::vec;

#[cfg(feature = "rust-crypto")]
use digest::{Digest, Mac};
use wasefire::crypto::hash::Algorithm;
#[cfg(not(feature = "rust-crypto"))]
use wasefire::crypto::hash::{hkdf_expand, hkdf_extract};
#[cfg(not(feature = "rust-crypto"))]
use wasefire::crypto::hash::{Digest, Hmac};
#[cfg(feature = "rust-crypto")]
use wasefire::crypto::hash::{HmacSha256, Sha256};

fn main() {
    debug!("Use RustCrypto API: {}", cfg!(feature = "rust-crypto"));
    test("sha256", Algorithm::Sha256, SHA256_VECTORS);
    test_hmac("sha256", Algorithm::Sha256, HMAC_SHA256_VECTORS);
    #[cfg(not(feature = "rust-crypto"))]
    test_hkdf("sha256", Algorithm::Sha256, HKDF_SHA256_VECTORS);
    debug::exit(true);
}

fn test(name: &str, algorithm: Algorithm, vectors: &[Vector]) {
    debug!("test_{name}(): Compute the digest of test vectors.");
    if !crypto::hash::is_supported(algorithm) {
        debug!("- not supported");
        return;
    }
    for &Vector { message, digest } in vectors {
        debug!("- {} bytes", message.len());
        #[cfg(feature = "rust-crypto")]
        let digest_ = Sha256::digest(message);
        #[cfg(not(feature = "rust-crypto"))]
        let digest_ = {
            let mut digest = vec![0; algorithm.digest_len()];
            Digest::digest(algorithm, message, &mut digest).unwrap();
            digest
        };
        debug::assert_eq(&digest_[..], digest);
    }
}

fn test_hmac(name: &str, algorithm: Algorithm, vectors: &[HmacVector]) {
    debug!("test_hmac_{name}(): Compute the hmac of test vectors.");
    if !crypto::hash::is_hmac_supported(algorithm) {
        debug!("- not supported");
        return;
    }
    for &HmacVector { count, key, msg, mac } in vectors {
        debug!("- {count}");
        #[cfg(feature = "rust-crypto")]
        let mac_ =
            HmacSha256::new_from_slice(key).unwrap().chain_update(msg).finalize().into_bytes();
        #[cfg(not(feature = "rust-crypto"))]
        let mac_ = {
            let mut mac = vec![0; algorithm.digest_len()];
            Hmac::hmac(algorithm, key, msg, &mut mac).unwrap();
            mac
        };
        debug::assert_eq(&mac_[..], mac);
    }
}

#[cfg(not(feature = "rust-crypto"))]
fn test_hkdf(name: &str, algorithm: Algorithm, vectors: &[HkdfVector]) {
    debug!("test_hkdf_{name}(): Compute the hkdf of test vectors.");
    if !crypto::hash::is_hkdf_supported(algorithm) {
        debug!("- not supported");
        return;
    }
    for &HkdfVector { test_case, salt, ikm, info, prk, okm } in vectors {
        debug!("- {test_case}");
        let mut prk_ = vec![0; algorithm.digest_len()];
        hkdf_extract(algorithm, salt, ikm, &mut prk_).unwrap();
        debug::assert_eq(&prk_[..], prk);
        let mut okm_ = vec![0; okm.len()];
        hkdf_expand(algorithm, prk, info, &mut okm_).unwrap();
        debug::assert_eq(&okm_[..], okm);
    }
}

struct Vector {
    message: &'static [u8],
    digest: &'static [u8],
}

// Those test vectors are taken from:
// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip
const SHA256_VECTORS: &[Vector] = &[
    Vector {
        message: &[],
        digest: &[
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ],
    },
    Vector {
        message: &[0xd3],
        digest: &[
            0x28, 0x96, 0x9c, 0xdf, 0xa7, 0x4a, 0x12, 0xc8, 0x2f, 0x3b, 0xad, 0x96, 0x0b, 0x0b,
            0x00, 0x0a, 0xca, 0x2a, 0xc3, 0x29, 0xde, 0xea, 0x5c, 0x23, 0x28, 0xeb, 0xc6, 0xf2,
            0xba, 0x98, 0x02, 0xc1,
        ],
    },
    Vector {
        message: &[0x11, 0xaf],
        digest: &[
            0x5c, 0xa7, 0x13, 0x3f, 0xa7, 0x35, 0x32, 0x60, 0x81, 0x55, 0x8a, 0xc3, 0x12, 0xc6,
            0x20, 0xee, 0xca, 0x99, 0x70, 0xd1, 0xe7, 0x0a, 0x4b, 0x95, 0x53, 0x3d, 0x95, 0x6f,
            0x07, 0x2d, 0x1f, 0x98,
        ],
    },
    Vector {
        message: &[0x74, 0xcb, 0x93, 0x81, 0xd8, 0x9f, 0x5a, 0xa7, 0x33, 0x68],
        digest: &[
            0x73, 0xd6, 0xfa, 0xd1, 0xca, 0xaa, 0x75, 0xb4, 0x3b, 0x21, 0x73, 0x35, 0x61, 0xfd,
            0x39, 0x58, 0xbd, 0xc5, 0x55, 0x19, 0x4a, 0x03, 0x7c, 0x2a, 0xdd, 0xec, 0x19, 0xdc,
            0x2d, 0x7a, 0x52, 0xbd,
        ],
    },
    Vector {
        message: &[
            0x0a, 0x27, 0x84, 0x7c, 0xdc, 0x98, 0xbd, 0x6f, 0x62, 0x22, 0x0b, 0x04, 0x6e, 0xdd,
            0x76, 0x2b,
        ],
        digest: &[
            0x80, 0xc2, 0x5e, 0xc1, 0x60, 0x05, 0x87, 0xe7, 0xf2, 0x8b, 0x18, 0xb1, 0xb1, 0x8e,
            0x3c, 0xdc, 0x89, 0x92, 0x8e, 0x39, 0xca, 0xb3, 0xbc, 0x25, 0xe4, 0xd4, 0xa4, 0xc1,
            0x39, 0xbc, 0xed, 0xc4,
        ],
    },
    Vector {
        message: &[
            0x07, 0x77, 0xfc, 0x1e, 0x1c, 0xa4, 0x73, 0x04, 0xc2, 0xe2, 0x65, 0x69, 0x28, 0x38,
            0x10, 0x9e, 0x26, 0xaa, 0xb9, 0xe5, 0xc4, 0xae, 0x4e, 0x86, 0x00, 0xdf, 0x4b, 0x1f,
        ],
        digest: &[
            0xff, 0xb4, 0xfc, 0x03, 0xe0, 0x54, 0xf8, 0xec, 0xbc, 0x31, 0x47, 0x0f, 0xc0, 0x23,
            0xbe, 0xdc, 0xd4, 0xa4, 0x06, 0xb9, 0xdd, 0x56, 0xc7, 0x1d, 0xa1, 0xb6, 0x60, 0xdc,
            0xc4, 0x84, 0x2c, 0x65,
        ],
    },
    Vector {
        message: &[
            0x9d, 0x64, 0xde, 0x71, 0x61, 0x89, 0x58, 0x84, 0xe7, 0xfa, 0x3d, 0x6e, 0x9e, 0xb9,
            0x96, 0xe7, 0xeb, 0xe5, 0x11, 0xb0, 0x1f, 0xe1, 0x9c, 0xd4, 0xa6, 0xb3, 0x32, 0x2e,
            0x80, 0xaa, 0xf5, 0x2b, 0xf6, 0x44, 0x7e, 0xd1, 0x85, 0x4e, 0x71, 0x00, 0x1f, 0x4d,
            0x54, 0xf8, 0x93, 0x1d,
        ],
        digest: &[
            0xd0, 0x48, 0xee, 0x15, 0x24, 0x01, 0x4a, 0xdf, 0x9a, 0x56, 0xe6, 0x0a, 0x38, 0x82,
            0x77, 0xde, 0x19, 0x4c, 0x69, 0x4c, 0xc7, 0x87, 0xfc, 0x5a, 0x1b, 0x55, 0x4e, 0xa9,
            0xf0, 0x7a, 0xbf, 0xdf,
        ],
    },
];

struct HmacVector {
    count: usize,
    key: &'static [u8],
    msg: &'static [u8],
    mac: &'static [u8],
}

// Those test vectors are taken from (with L=32 and Tlen=32):
// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/mac/hmactestvectors.zip
const HMAC_SHA256_VECTORS: &[HmacVector] = &[
    HmacVector {
        count: 30,
        key: &[
            0x97, 0x79, 0xd9, 0x12, 0x06, 0x42, 0x79, 0x7f, 0x17, 0x47, 0x02, 0x5d, 0x5b, 0x22,
            0xb7, 0xac, 0x60, 0x7c, 0xab, 0x08, 0xe1, 0x75, 0x8f, 0x2f, 0x3a, 0x46, 0xc8, 0xbe,
            0x1e, 0x25, 0xc5, 0x3b, 0x8c, 0x6a, 0x8f, 0x58, 0xff, 0xef, 0xa1, 0x76,
        ],
        msg: &[
            0xb1, 0x68, 0x9c, 0x25, 0x91, 0xea, 0xf3, 0xc9, 0xe6, 0x60, 0x70, 0xf8, 0xa7, 0x79,
            0x54, 0xff, 0xb8, 0x17, 0x49, 0xf1, 0xb0, 0x03, 0x46, 0xf9, 0xdf, 0xe0, 0xb2, 0xee,
            0x90, 0x5d, 0xcc, 0x28, 0x8b, 0xaf, 0x4a, 0x92, 0xde, 0x3f, 0x40, 0x01, 0xdd, 0x9f,
            0x44, 0xc4, 0x68, 0xc3, 0xd0, 0x7d, 0x6c, 0x6e, 0xe8, 0x2f, 0xac, 0xea, 0xfc, 0x97,
            0xc2, 0xfc, 0x0f, 0xc0, 0x60, 0x17, 0x19, 0xd2, 0xdc, 0xd0, 0xaa, 0x2a, 0xec, 0x92,
            0xd1, 0xb0, 0xae, 0x93, 0x3c, 0x65, 0xeb, 0x06, 0xa0, 0x3c, 0x9c, 0x93, 0x5c, 0x2b,
            0xad, 0x04, 0x59, 0x81, 0x02, 0x41, 0x34, 0x7a, 0xb8, 0x7e, 0x9f, 0x11, 0xad, 0xb3,
            0x04, 0x15, 0x42, 0x4c, 0x6c, 0x7f, 0x5f, 0x22, 0xa0, 0x03, 0xb8, 0xab, 0x8d, 0xe5,
            0x4f, 0x6d, 0xed, 0x0e, 0x3a, 0xb9, 0x24, 0x5f, 0xa7, 0x95, 0x68, 0x45, 0x1d, 0xfa,
            0x25, 0x8e,
        ],
        mac: &[
            0x76, 0x9f, 0x00, 0xd3, 0xe6, 0xa6, 0xcc, 0x1f, 0xb4, 0x26, 0xa1, 0x4a, 0x4f, 0x76,
            0xc6, 0x46, 0x2e, 0x61, 0x49, 0x72, 0x6e, 0x0d, 0xee, 0x0e, 0xc0, 0xcf, 0x97, 0xa1,
            0x66, 0x05, 0xac, 0x8b,
        ],
    },
    HmacVector {
        count: 31,
        key: &[
            0x09, 0x67, 0x5f, 0x2d, 0xcc, 0x47, 0x83, 0xb5, 0x99, 0xf1, 0x8f, 0xb7, 0x65, 0x58,
            0x36, 0x68, 0xa0, 0xfd, 0x8a, 0xe4, 0x09, 0x6f, 0x6f, 0xcd, 0xc6, 0x0d, 0x4f, 0x35,
            0xb4, 0x13, 0x0f, 0xbe, 0xfc, 0xd5, 0x42, 0xff, 0xe7, 0x45, 0x9d, 0x2a,
        ],
        msg: &[
            0x0c, 0xf2, 0x19, 0x8c, 0x31, 0x37, 0x6f, 0x5c, 0x89, 0x15, 0x66, 0x01, 0x37, 0x72,
            0x5f, 0x2b, 0xbc, 0x18, 0x0a, 0x98, 0x6e, 0x5a, 0x7b, 0xda, 0x27, 0xfa, 0x81, 0x59,
            0x3a, 0x4a, 0x33, 0x9b, 0xab, 0x92, 0xcb, 0xc3, 0x9f, 0xb2, 0xb8, 0x58, 0x11, 0x08,
            0xee, 0x48, 0xc7, 0x94, 0x81, 0x2d, 0x84, 0x5a, 0x72, 0xce, 0x80, 0x08, 0xc9, 0xe9,
            0x15, 0xd9, 0xe3, 0x30, 0xbb, 0xb9, 0x0e, 0x91, 0x36, 0xaa, 0x53, 0xba, 0x0e, 0x66,
            0x93, 0xdd, 0x40, 0x46, 0xd6, 0xb0, 0x33, 0x62, 0xdf, 0xb9, 0xed, 0xfa, 0x04, 0xc8,
            0x87, 0x15, 0x3c, 0xc5, 0xde, 0x67, 0x7a, 0xab, 0x8c, 0x78, 0x39, 0xd5, 0x17, 0x03,
            0x58, 0x79, 0x67, 0x9c, 0x29, 0x72, 0x7e, 0x96, 0xc5, 0x42, 0x63, 0x24, 0xa2, 0x57,
            0x5f, 0xbe, 0x67, 0x8d, 0x6c, 0xc7, 0xfe, 0xf5, 0xeb, 0x6c, 0xeb, 0xd5, 0x95, 0xcf,
            0xdd, 0xef,
        ],
        mac: &[
            0x6b, 0x14, 0x2d, 0x4d, 0xfe, 0x21, 0x7f, 0x18, 0x81, 0xaa, 0x0e, 0x64, 0x83, 0xb2,
            0x71, 0xdd, 0x5d, 0x43, 0xf7, 0x0b, 0x85, 0x60, 0x59, 0x53, 0xa0, 0xfe, 0xf2, 0x72,
            0xdd, 0xde, 0x46, 0xca,
        ],
    },
    HmacVector {
        count: 32,
        key: &[
            0xcf, 0xd4, 0xa4, 0x49, 0x10, 0xc9, 0xe5, 0x67, 0x50, 0x7a, 0xbb, 0x6c, 0xed, 0xe4,
            0xfe, 0x60, 0x1a, 0x7a, 0x27, 0x65, 0xc9, 0x75, 0x5a, 0xa2, 0xcf, 0x6b, 0xa4, 0x81,
            0x42, 0x23, 0x81, 0x1a, 0x26, 0xa8, 0xa1, 0xef, 0x49, 0x9c, 0xeb, 0xd9,
        ],
        msg: &[
            0x3f, 0xb3, 0x01, 0xcb, 0x40, 0x92, 0xf9, 0x62, 0x3a, 0xa5, 0xff, 0xd6, 0x90, 0xd2,
            0x2d, 0x65, 0xd5, 0x6e, 0x5a, 0x1c, 0x33, 0x0b, 0x9c, 0x4a, 0x0d, 0x91, 0x0c, 0x34,
            0xe3, 0x91, 0xc9, 0x0a, 0x76, 0xd5, 0x40, 0x1a, 0x2d, 0x3c, 0xaa, 0x44, 0xb8, 0xc5,
            0xd5, 0xae, 0xf3, 0xe9, 0x28, 0xb9, 0x0d, 0x2e, 0xe2, 0x33, 0xe9, 0xf9, 0xa2, 0xce,
            0xc4, 0xa3, 0x2c, 0xd0, 0x19, 0xd0, 0x6a, 0x0d, 0xc1, 0xfc, 0xb1, 0x12, 0x5f, 0x57,
            0x46, 0xa4, 0xfb, 0xd3, 0x21, 0x69, 0xed, 0x7b, 0xf0, 0xe4, 0xfd, 0x06, 0x5f, 0xa7,
            0xc8, 0xac, 0x97, 0xc3, 0x66, 0x38, 0x04, 0x84, 0x49, 0x5f, 0x5c, 0x5b, 0x68, 0x50,
            0xdd, 0x1c, 0x9d, 0x8c, 0xd6, 0x69, 0x4c, 0xf8, 0x68, 0x6e, 0x46, 0x30, 0x8e, 0xd0,
            0xed, 0x1f, 0x5b, 0xdf, 0x98, 0xcd, 0x83, 0x13, 0x39, 0x77, 0x1d, 0xb6, 0x3d, 0xe5,
            0xa7, 0xde,
        ],
        mac: &[
            0x20, 0x15, 0x3b, 0xf8, 0xea, 0x29, 0x53, 0xc4, 0x82, 0x51, 0xeb, 0xcc, 0x41, 0x61,
            0xf8, 0xb6, 0xe2, 0x84, 0x99, 0xe5, 0xc7, 0x6c, 0x24, 0x01, 0x4c, 0xff, 0x4a, 0x9e,
            0x2f, 0x62, 0xd2, 0x5c,
        ],
    },
    HmacVector {
        count: 33,
        key: &[
            0x54, 0x48, 0x99, 0x8f, 0x9d, 0x8f, 0x98, 0x53, 0x4a, 0xdd, 0xf0, 0xc8, 0xba, 0x63,
            0x1c, 0x49, 0x6b, 0xf8, 0xa8, 0x00, 0x6c, 0xbb, 0x46, 0xad, 0x15, 0xfa, 0x1f, 0xa2,
            0xf5, 0x53, 0x67, 0x12, 0x0c, 0x19, 0x34, 0x8c, 0x3a, 0xfa, 0x90, 0xc3,
        ],
        msg: &[
            0x1c, 0x43, 0x96, 0xf7, 0xb7, 0xf9, 0x22, 0x8e, 0x83, 0x2a, 0x13, 0x69, 0x20, 0x02,
            0xba, 0x2a, 0xff, 0x43, 0x9d, 0xcb, 0x7f, 0xdd, 0xbf, 0xd4, 0x56, 0xc0, 0x22, 0xd1,
            0x33, 0xee, 0x89, 0x03, 0xa2, 0xd4, 0x82, 0x56, 0x2f, 0xda, 0xa4, 0x93, 0xce, 0x39,
            0x16, 0xd7, 0x7a, 0x0c, 0x51, 0x44, 0x1d, 0xab, 0x26, 0xf6, 0xb0, 0x34, 0x02, 0x38,
            0xa3, 0x6a, 0x71, 0xf8, 0x7f, 0xc3, 0xe1, 0x79, 0xca, 0xbc, 0xa9, 0x48, 0x2b, 0x70,
            0x49, 0x71, 0xce, 0x69, 0xf3, 0xf2, 0x0a, 0xb6, 0x4b, 0x70, 0x41, 0x3d, 0x6c, 0x29,
            0x08, 0x53, 0x2b, 0x2a, 0x88, 0x8a, 0x9f, 0xc2, 0x24, 0xca, 0xe1, 0x36, 0x5d, 0xa4,
            0x10, 0xb6, 0xf2, 0xe2, 0x98, 0x90, 0x4b, 0x63, 0xb4, 0xa4, 0x17, 0x26, 0x32, 0x18,
            0x35, 0xa4, 0x77, 0x4d, 0xd0, 0x63, 0xc2, 0x11, 0xcf, 0xc8, 0xb5, 0x16, 0x6c, 0x2d,
            0x11, 0xa2,
        ],
        mac: &[
            0x7e, 0x8c, 0xba, 0x9d, 0xd9, 0xf0, 0x6e, 0xbd, 0xd7, 0xf9, 0x2e, 0x0f, 0x1a, 0x67,
            0xc7, 0xf4, 0xdf, 0x52, 0x69, 0x3c, 0x21, 0x2b, 0xdd, 0x84, 0xf6, 0x73, 0x70, 0xb3,
            0x51, 0x53, 0x3c, 0x6c,
        ],
    },
    HmacVector {
        count: 34,
        key: &[
            0x9d, 0xa0, 0xc1, 0x14, 0x68, 0x2f, 0x82, 0xc1, 0xd1, 0xe9, 0xb5, 0x44, 0x30, 0x58,
            0x0b, 0x9c, 0x56, 0x94, 0x89, 0xca, 0x16, 0xb9, 0x2e, 0xe1, 0x04, 0x98, 0xd5, 0x5d,
            0x7c, 0xad, 0x5d, 0xb5, 0xe6, 0x52, 0x06, 0x34, 0x39, 0x31, 0x1e, 0x04,
        ],
        msg: &[
            0x49, 0x53, 0x40, 0x8b, 0xe3, 0xdd, 0xde, 0x42, 0x52, 0x1e, 0xb6, 0x25, 0xa3, 0x7a,
            0xf0, 0xd2, 0xcf, 0x9e, 0xd1, 0x84, 0xf5, 0xb6, 0x27, 0xe5, 0xe7, 0xe0, 0xe8, 0x24,
            0xe8, 0xe1, 0x16, 0x48, 0xb4, 0x18, 0xe5, 0xc4, 0xc1, 0xb0, 0x20, 0x4b, 0xc5, 0x19,
            0xc9, 0xe5, 0x78, 0xb8, 0x00, 0x43, 0x9b, 0xdd, 0x25, 0x4f, 0x39, 0xf6, 0x41, 0x08,
            0x2d, 0x03, 0xa2, 0x8d, 0xe4, 0x4a, 0xc6, 0x77, 0x64, 0x4c, 0x7b, 0x6c, 0x8d, 0xf7,
            0x43, 0xf2, 0x9f, 0x1d, 0xfd, 0x80, 0xfd, 0x25, 0xc2, 0xdb, 0x31, 0x01, 0x0e, 0xa0,
            0x2f, 0x60, 0x20, 0x1c, 0xde, 0x24, 0xa3, 0x64, 0xd4, 0x16, 0x8d, 0xa2, 0x61, 0xd8,
            0x48, 0xae, 0xd0, 0x1c, 0x10, 0xde, 0xe9, 0x14, 0x9c, 0x1e, 0xbb, 0x29, 0x00, 0x43,
            0x98, 0xf0, 0xd2, 0x9c, 0x60, 0x5a, 0x8b, 0xca, 0x03, 0x2b, 0x31, 0xd2, 0x41, 0xad,
            0x33, 0x71,
        ],
        mac: &[
            0xcd, 0xea, 0xcf, 0xce, 0xbf, 0x46, 0xcc, 0x9d, 0x7e, 0x4d, 0x41, 0x75, 0xe5, 0xd8,
            0xd2, 0x67, 0xc2, 0x3a, 0x64, 0xcd, 0xe8, 0x3e, 0x86, 0x7e, 0x50, 0x01, 0xec, 0xf2,
            0x6f, 0xbd, 0x30, 0xd2,
        ],
    },
];

#[cfg(not(feature = "rust-crypto"))]
struct HkdfVector {
    test_case: usize,
    salt: Option<&'static [u8]>,
    ikm: &'static [u8],
    info: &'static [u8],
    prk: &'static [u8],
    okm: &'static [u8],
}

// Those test vectors are taken from: https://datatracker.ietf.org/doc/html/rfc5869.
#[cfg(not(feature = "rust-crypto"))]
const HKDF_SHA256_VECTORS: &[HkdfVector] = &[
    HkdfVector {
        test_case: 1,
        salt: Some(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]),
        ikm: &[
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
        ],
        info: &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
        prk: &[
            0x07, 0x77, 0x09, 0x36, 0x2c, 0x2e, 0x32, 0xdf, 0x0d, 0xdc, 0x3f, 0x0d, 0xc4, 0x7b,
            0xba, 0x63, 0x90, 0xb6, 0xc7, 0x3b, 0xb5, 0x0f, 0x9c, 0x31, 0x22, 0xec, 0x84, 0x4a,
            0xd7, 0xc2, 0xb3, 0xe5,
        ],
        okm: &[
            0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a, 0x90, 0x43, 0x4f, 0x64, 0xd0, 0x36,
            0x2f, 0x2a, 0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c, 0x5d, 0xb0, 0x2d, 0x56,
            0xec, 0xc4, 0xc5, 0xbf, 0x34, 0x00, 0x72, 0x08, 0xd5, 0xb8, 0x87, 0x18, 0x58, 0x65,
        ],
    },
    HkdfVector {
        test_case: 2,
        salt: Some(&[
            0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
            0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b,
            0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
            0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
            0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5,
            0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
        ]),
        ikm: &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
            0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45,
            0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
        ],
        info: &[
            0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd,
            0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb,
            0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9,
            0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
            0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5,
            0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
        ],
        prk: &[
            0x06, 0xa6, 0xb8, 0x8c, 0x58, 0x53, 0x36, 0x1a, 0x06, 0x10, 0x4c, 0x9c, 0xeb, 0x35,
            0xb4, 0x5c, 0xef, 0x76, 0x00, 0x14, 0x90, 0x46, 0x71, 0x01, 0x4a, 0x19, 0x3f, 0x40,
            0xc1, 0x5f, 0xc2, 0x44,
        ],
        okm: &[
            0xb1, 0x1e, 0x39, 0x8d, 0xc8, 0x03, 0x27, 0xa1, 0xc8, 0xe7, 0xf7, 0x8c, 0x59, 0x6a,
            0x49, 0x34, 0x4f, 0x01, 0x2e, 0xda, 0x2d, 0x4e, 0xfa, 0xd8, 0xa0, 0x50, 0xcc, 0x4c,
            0x19, 0xaf, 0xa9, 0x7c, 0x59, 0x04, 0x5a, 0x99, 0xca, 0xc7, 0x82, 0x72, 0x71, 0xcb,
            0x41, 0xc6, 0x5e, 0x59, 0x0e, 0x09, 0xda, 0x32, 0x75, 0x60, 0x0c, 0x2f, 0x09, 0xb8,
            0x36, 0x77, 0x93, 0xa9, 0xac, 0xa3, 0xdb, 0x71, 0xcc, 0x30, 0xc5, 0x81, 0x79, 0xec,
            0x3e, 0x87, 0xc1, 0x4c, 0x01, 0xd5, 0xc1, 0xf3, 0x43, 0x4f, 0x1d, 0x87,
        ],
    },
    HkdfVector {
        test_case: 3,
        salt: Some(&[]),
        ikm: &[
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
        ],
        info: &[],
        prk: &[
            0x19, 0xef, 0x24, 0xa3, 0x2c, 0x71, 0x7b, 0x16, 0x7f, 0x33, 0xa9, 0x1d, 0x6f, 0x64,
            0x8b, 0xdf, 0x96, 0x59, 0x67, 0x76, 0xaf, 0xdb, 0x63, 0x77, 0xac, 0x43, 0x4c, 0x1c,
            0x29, 0x3c, 0xcb, 0x04,
        ],
        okm: &[
            0x8d, 0xa4, 0xe7, 0x75, 0xa5, 0x63, 0xc1, 0x8f, 0x71, 0x5f, 0x80, 0x2a, 0x06, 0x3c,
            0x5a, 0x31, 0xb8, 0xa1, 0x1f, 0x5c, 0x5e, 0xe1, 0x87, 0x9e, 0xc3, 0x45, 0x4e, 0x5f,
            0x3c, 0x73, 0x8d, 0x2d, 0x9d, 0x20, 0x13, 0x95, 0xfa, 0xa4, 0xb6, 0x1a, 0x96, 0xc8,
        ],
    },
    // This is test case 1 but without salt. The prk and okm come from running RustCrypto.
    HkdfVector {
        test_case: 4,
        salt: None,
        ikm: &[
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
            0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
        ],
        info: &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
        prk: &[
            0x19, 0xef, 0x24, 0xa3, 0x2c, 0x71, 0x7b, 0x16, 0x7f, 0x33, 0xa9, 0x1d, 0x6f, 0x64,
            0x8b, 0xdf, 0x96, 0x59, 0x67, 0x76, 0xaf, 0xdb, 0x63, 0x77, 0xac, 0x43, 0x4c, 0x1c,
            0x29, 0x3c, 0xcb, 0x4,
        ],
        okm: &[
            0xab, 0xba, 0xfb, 0x13, 0xf5, 0xc1, 0xbc, 0x48, 0x9d, 0x42, 0x3, 0x13, 0x58, 0x17,
            0x95, 0x6d, 0xd5, 0x21, 0xb3, 0x9e, 0x3b, 0xd6, 0x1d, 0x1c, 0xc8, 0x5c, 0xef, 0x88,
            0x4d, 0x1f, 0x8e, 0x2e, 0x2c, 0xa9, 0xc1, 0x9f, 0x23, 0xdf, 0x62, 0xd, 0xd3, 0x94,
        ],
    },
];

#[cfg(test)]
mod tests {
    use wasefire_stub as _;

    use super::*;

    #[test]
    fn test_sha256() {
        test("sha256", Algorithm::Sha256, SHA256_VECTORS);
    }

    #[test]
    fn test_hmac_sha256() {
        test_hmac("sha256", Algorithm::Sha256, HMAC_SHA256_VECTORS);
    }

    #[cfg(not(feature = "rust-crypto"))]
    #[test]
    fn test_hkdf_sha256() {
        test_hkdf("sha256", Algorithm::Sha256, HKDF_SHA256_VECTORS);
    }
}
