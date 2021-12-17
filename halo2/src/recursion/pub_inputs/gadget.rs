//! Instructions for endoscaling public inputs.
use halo2_proofs::{circuit::Layouter, plonk::Error};
use pasta_curves::arithmetic::FieldExt;

mod chip;

/// Instructions to map bitstring public inputs to and from endoscalars.
pub trait PubInputsInstructions<F: FieldExt + PubInputsLookup<K, N>, const K: usize, const N: usize>
{
    /// An N-bit word.
    type Word;

    /// An endoscalar corresponding to an N-bit word.
    type Endoscalar;

    /// Gets a bitstring and its endoscalar representation.
    ///
    /// These endoscalars are provided as the cells in the public input column.
    fn get_bitstring(
        &self,
        layouter: impl Layouter<F>,
        row: usize,
    ) -> Result<(Self::Word, Self::Endoscalar), Error>;

    /// Compute commitment to N-bit words using the endoscaling algorithm.
    ///
    /// The points in Params.g_lagrange are used as fixed bases, and each scaled
    /// base is added to an accumulator.
    fn commit(
        &self,
        words: Vec<Self::Word>,
        layouter: impl Layouter<F>,
    ) -> Result<(Self::Word, Self::Endoscalar), Error>;
}

/// A trait providing the lookup table for decoding public inputs.
pub trait PubInputsLookup<const K: usize, const N: usize>
where
    Self: std::marker::Sized,
{
    /// A lookup table mapping `K`-bit values to endoscalars.
    fn table() -> [([bool; K], Self); N];
}
