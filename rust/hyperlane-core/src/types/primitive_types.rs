// Based on https://github.com/paritytech/parity-common/blob/a5ef7308d6986e62431e35d3156fed0a7a585d39/primitive-types/src/lib.rs

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::reversed_empty_ranges)]

use borsh::{BorshDeserialize, BorshSerialize};
use fixed_hash::impl_fixed_hash_conversions;
use uint::construct_uint;

use crate::types::serialize;

/// Error type for conversion.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Overflow encountered.
    Overflow,
}

construct_uint! {
    /// 128-bit unsigned integer.
    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct U128(2);
}
construct_uint! {
    /// 256-bit unsigned integer.
    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct U256(4);
}

construct_uint! {
    /// 512-bit unsigned integer.
    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct U512(8);
}

mod fixed_hashes {
    // we can't change how they made the macro, so ignore the lint
    #![allow(clippy::incorrect_clone_impl_on_copy_type)]

    use borsh::{BorshDeserialize, BorshSerialize};
    use fixed_hash::construct_fixed_hash;

    construct_fixed_hash! {
        /// 128-bit hash type.
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct H128(16);
    }

    construct_fixed_hash! {
        /// 160-bit hash type.
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct H160(20);
    }

    construct_fixed_hash! {
        /// 256-bit hash type.
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct H256(32);
    }

    construct_fixed_hash! {
        /// 512-bit hash type.
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct H512(64);
    }
}
pub use fixed_hashes::*;

#[cfg(feature = "ethers")]
type EthersH160 = ethers_core::types::H160;
#[cfg(feature = "ethers")]
type EthersH256 = ethers_core::types::H256;
#[cfg(feature = "ethers")]
type EthersH512 = ethers_core::types::H512;

#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(H256, EthersH160);
#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(EthersH256, H160);
#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(EthersH512, H160);
#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(EthersH512, H256);
#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(H512, EthersH160);
#[cfg(feature = "ethers")]
impl_fixed_hash_conversions!(H512, EthersH256);

impl_fixed_hash_conversions!(H256, H160);
impl_fixed_hash_conversions!(H512, H256);
impl_fixed_hash_conversions!(H512, H160);

macro_rules! impl_fixed_uint_conversions {
    ($larger:ty, $smaller:ty) => {
        impl From<$smaller> for $larger {
            impl_fixed_uint_conversions!(@from_smaller $larger, $smaller);
        }

        impl<'a> From<&'a $smaller> for $larger {
            impl_fixed_uint_conversions!(@from_smaller $larger, &'a $smaller);
        }

        impl TryFrom<$larger> for $smaller {
            type Error = Error;
            impl_fixed_uint_conversions!(@try_from_larger $larger, $smaller);
        }

        impl<'a> TryFrom<&'a $larger> for $smaller {
            type Error = Error;
            impl_fixed_uint_conversions!(@try_from_larger &'a $larger, $smaller);
        }
    };
    (@from_smaller $larger:ty, $smaller:ty) => {
        fn from(val: $smaller) -> $larger {
            let mut ret = <$larger>::zero();
            for i in 0..val.0.len() {
                ret.0[i] = val.0[i];
            }
            ret
        }
    };
    (@try_from_larger $larger:ty, $smaller:ty) => {
        fn try_from(val: $larger) -> Result<$smaller, Error> {
            let mut ret = <$smaller>::zero();
            for i in 0..ret.0.len() {
                ret.0[i] = val.0[i];
            }

            let mut ov = 0;
            for i in ret.0.len()..val.0.len() {
                ov |= val.0[i];
            }
            if ov == 0 {
                Ok(ret)
            } else {
                Err(Error::Overflow)
            }
        }
    };
}

impl_fixed_uint_conversions!(U256, U128);
impl_fixed_uint_conversions!(U512, U128);
impl_fixed_uint_conversions!(U512, U256);
#[cfg(feature = "ethers")]
impl_fixed_uint_conversions!(U256, ethers_core::types::U128);
#[cfg(feature = "ethers")]
impl_fixed_uint_conversions!(U512, ethers_core::types::U128);
#[cfg(feature = "ethers")]
impl_fixed_uint_conversions!(U512, ethers_core::types::U256);
#[cfg(feature = "ethers")]
impl_fixed_uint_conversions!(ethers_core::types::U512, U256);
#[cfg(feature = "ethers")]
impl_fixed_uint_conversions!(ethers_core::types::U512, U128);

macro_rules! impl_f64_conversions {
    ($ty:ty) => {
        impl $ty {
            /// Lossy saturating conversion from a `f64` to a `$ty`. Like for floating point to
            /// primitive integer type conversions, this truncates fractional parts.
            ///
            /// The conversion follows the same rules as converting `f64` to other
            /// primitive integer types. Namely, the conversion of `value: f64` behaves as
            /// follows:
            /// - `NaN` => `0`
            /// - `(-∞, 0]` => `0`
            /// - `(0, $ty::MAX]` => `value as $ty`
            /// - `($ty::MAX, +∞)` => `$ty::MAX`
            pub fn from_f64_lossy(val: f64) -> $ty {
                const TY_BITS: u64 = <$ty>::zero().0.len() as u64 * <$ty>::WORD_BITS as u64;
                if val >= 1.0 {
                    let bits = val.to_bits();
                    // NOTE: Don't consider the sign or check that the subtraction will
                    //   underflow since we already checked that the value is greater
                    //   than 1.0.
                    let exponent = ((bits >> 52) & 0x7ff) - 1023;
                    let mantissa = (bits & 0x0f_ffff_ffff_ffff) | 0x10_0000_0000_0000;

                    if exponent <= 52 {
                        <$ty>::from(mantissa >> (52 - exponent))
                    } else if exponent < TY_BITS {
                        <$ty>::from(mantissa) << <$ty>::from(exponent - 52)
                    } else {
                        <$ty>::MAX
                    }
                } else {
                    <$ty>::zero()
                }
            }

            /// Lossy conversion of `$ty` to `f64`.
            pub fn to_f64_lossy(self) -> f64 {
                let mut acc = 0.0;
                for i in (0..self.0.len()).rev() {
                    acc += self.0[i] as f64 * 2.0f64.powi((i * <$ty>::WORD_BITS) as i32);
                }
                acc
            }
        }
    };
}

impl_f64_conversions!(U128);
impl_f64_conversions!(U256);
impl_f64_conversions!(U512);

#[cfg(feature = "ethers")]
macro_rules! impl_inner_conversion {
    ($a:ty, $b:ty) => {
        impl From<$a> for $b {
            fn from(val: $a) -> Self {
                Self(val.0)
            }
        }

        impl<'a> From<&'a $a> for $b {
            fn from(val: &'a $a) -> Self {
                Self(val.0)
            }
        }

        impl From<$b> for $a {
            fn from(val: $b) -> Self {
                Self(val.0)
            }
        }

        impl<'a> From<&'a $b> for $a {
            fn from(val: &'a $b) -> Self {
                Self(val.0)
            }
        }
    };
}

#[cfg(feature = "ethers")]
impl_inner_conversion!(H128, ethers_core::types::H128);
#[cfg(feature = "ethers")]
impl_inner_conversion!(H160, ethers_core::types::H160);
#[cfg(feature = "ethers")]
impl_inner_conversion!(H256, ethers_core::types::H256);
#[cfg(feature = "ethers")]
impl_inner_conversion!(H512, ethers_core::types::H512);
#[cfg(feature = "ethers")]
impl_inner_conversion!(U128, ethers_core::types::U128);
#[cfg(feature = "ethers")]
impl_inner_conversion!(U256, ethers_core::types::U256);
#[cfg(feature = "ethers")]
impl_inner_conversion!(U512, ethers_core::types::U512);

/// Add Serde serialization support to an integer created by `construct_uint!`.
macro_rules! impl_uint_serde {
    ($name: ident, $len: expr) => {
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut slice = [0u8; 2 + 2 * $len * 8];
                let mut bytes = [0u8; $len * 8];
                self.to_big_endian(&mut bytes);
                serialize::serialize_uint(&mut slice, &bytes, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mut bytes = [0u8; $len * 8];
                let wrote = serialize::deserialize_check_len(
                    deserializer,
                    serialize::ExpectedLen::Between(0, &mut bytes),
                )?;
                Ok(bytes[0..wrote].into())
            }
        }
    };
}

/// Add Serde serialization support to a fixed-sized hash type created by `construct_fixed_hash!`.
macro_rules! impl_fixed_hash_serde {
    ($name: ident, $len: expr) => {
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut slice = [0u8; 2 + 2 * $len];
                serialize::serialize_raw(&mut slice, &self.0, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mut bytes = [0u8; $len];
                serialize::deserialize_check_len(
                    deserializer,
                    serialize::ExpectedLen::Exact(&mut bytes),
                )?;
                Ok($name(bytes))
            }
        }
    };
}

impl_uint_serde!(U128, 2);
impl_uint_serde!(U256, 4);
impl_uint_serde!(U512, 8);

impl_fixed_hash_serde!(H128, 16);
impl_fixed_hash_serde!(H160, 20);
impl_fixed_hash_serde!(H256, 32);
impl_fixed_hash_serde!(H512, 64);

#[cfg(feature = "solana")]
impl From<solana_sdk::hash::Hash> for H256 {
    fn from(hash: solana_sdk::hash::Hash) -> Self {
        H256(hash.to_bytes())
    }
}

// Solana uses the first 64 byte signature of a transaction to uniquely identify the
// transaction.
#[cfg(feature = "solana")]
impl From<solana_sdk::signature::Signature> for H512 {
    fn from(sig: solana_sdk::signature::Signature) -> Self {
        H512(sig.into())
    }
}
