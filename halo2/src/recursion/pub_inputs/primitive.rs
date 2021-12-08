//! Primitives used in packing and unpacking public inputs.

use pasta_curves::arithmetic::FieldExt;

use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryInto;

/// Lookup table mapping endoscalars to their corresponding NUM_BITS-bit representations.
#[allow(dead_code)]
#[allow(clippy::type_complexity)]
fn lookup_table<F: FieldExt, const NUM_BITS: usize>(
) -> (Vec<([bool; NUM_BITS], F)>, BTreeMap<F, [bool; NUM_BITS]>) {
    let mut endoscalars = BTreeSet::new();
    let mut table = Vec::new();
    let mut inv_table = BTreeMap::new();
    let num_rows = 1 << NUM_BITS;

    for row in 0..num_rows {
        let bits = i2lebsp(row as u64);
        let scalar = endoscale(bits);

        assert!(endoscalars.insert(scalar));

        table.push((bits, scalar));
        inv_table.insert(scalar, bits);
    }

    (table, inv_table)
}

/// Maps a K-bit bitstring to a scalar.
///
/// This corresponds to Algorithm 1 from [BGH2019], where `F` corresponds to $F_q$, the
/// scalar field of $P$. Where Algorithm 1 computes $Acc = [scalar] P$, this function
/// computes `scalar`.
///
/// [BGH2019]: https://eprint.iacr.org/2019/1021.pdf
#[allow(dead_code)]
pub(crate) fn endoscale<F: FieldExt, const K: usize>(bits: [bool; K]) -> F {
    assert_eq!(K % 2, 0);

    /// Maps a pair of bits to a scalar using endoscaling.
    fn endoscale_pair<F: FieldExt>(bits: (bool, bool)) -> F {
        // [2 * bits.0 - 1]
        let mut scalar = F::from(bits.0).double() - F::one();

        if bits.1 {
            scalar *= F::ZETA;
        }

        scalar
    }

    let mut scalar = (F::ZETA + F::one()).double();

    for j in 0..(K / 2) {
        let pair = (bits[2 * j], bits[2 * j + 1]);
        scalar = endoscale_pair::<F>(pair) + scalar.double();
    }

    scalar
}

/// Maps an arbitrary-length bitstring to a list of endoscalars, where each endoscalar
/// corresponds to an N-bit chunk. Uses zero padding if necessary.
#[allow(dead_code)]
pub(crate) fn endoscale_with_padding<F: FieldExt, const N: usize>(bits: &[bool]) -> Vec<F> {
    let padding_len = bits.len() % N;
    let mut padded_bits = bits.to_vec();
    if padding_len != 0 {
        padded_bits.extend_from_slice(&vec![false; padding_len]);
    }

    let mut endoscalars = Vec::new();
    for chunk_idx in 0..(padded_bits.len() / N) {
        let idx = chunk_idx * 10;
        let endoscalar = endoscale::<F, N>(padded_bits[idx..(idx + 10)].try_into().unwrap());
        endoscalars.push(endoscalar);
    }

    endoscalars
}

#[allow(dead_code)]
pub(crate) fn i2lebsp<const NUM_BITS: usize>(int: u64) -> [bool; NUM_BITS] {
    assert!(NUM_BITS <= 64);

    fn gen_const_array<Output: Copy + Default, const LEN: usize>(
        closure: impl FnMut(usize) -> Output,
    ) -> [Output; LEN] {
        fn gen_const_array_with_default<Output: Copy, const LEN: usize>(
            default_value: Output,
            mut closure: impl FnMut(usize) -> Output,
        ) -> [Output; LEN] {
            let mut ret: [Output; LEN] = [default_value; LEN];
            for (bit, val) in ret.iter_mut().zip((0..LEN).map(|idx| closure(idx))) {
                *bit = val;
            }
            ret
        }
        gen_const_array_with_default(Default::default(), closure)
    }

    gen_const_array(|mask: usize| (int & (1 << mask)) != 0)
}

#[test]
fn test_lookup() {
    use pasta_curves::pallas;

    lookup_table::<pallas::Scalar, 10>();
}
