//! Contains goldilocks field.

// FIXME: Remove. Not sure why this is erring.
#![allow(missing_docs)]

use ff::PrimeField;
use pasta_curves::arithmetic::{CurveAffine, CurveExt, FieldExt};

// FIXME: This should use only one limb
// Limitation in `ff_derive` results in use of 2 limbs.
// See https://github.com/zkcrypto/ff/blob/9b9a8d9c363ecbf7bb4c79998aaed32c1f8ce027/ff_derive/src/lib.rs#L142
#[derive(PrimeField)]
#[PrimeFieldModulus = "18446744069414584321"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
/// Goldilocks field.
/// p = 2^64 - 2^32 - 1. Discovered by Plonky2 authors.
pub struct Fp([u64; 2]);
