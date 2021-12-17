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

#[cfg(test)]
mod tests {
    use crate::{
        circuit::{Layouter, SimpleFloorPlanner},
        plonk::{Circuit, ConstraintSystem, Error},
    };
    use pasta_curves::{arithmetic::FieldExt, Fp, Fq};

    use super::{
        chip::{lookup::TableConfig, PubInputsConfig},
        PubInputsInstructions, PubInputsLookup,
    };

    use std::marker::PhantomData;

    #[derive(Default)]
    struct MyCircuit<F: FieldExt>(PhantomData<F>);

    impl Circuit<Fp> for MyCircuit<Fp> {
        type Config = PubInputsConfig<Fp, 10>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            let table_config = TableConfig::configure(meta);

            let _ = meta.instance_column();
            let endoscalars = meta.instance_column();
            let endoscalars_copy = meta.advice_column();
            let bits = meta.advice_column();

            PubInputsConfig::<Fp, 10>::configure(
                meta,
                endoscalars,
                endoscalars_copy,
                bits,
                table_config,
            )
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<Fp>,
        ) -> Result<(), Error> {
            config.table.load(&mut layouter)?;

            for row in 0..1024 {
                config.get_bitstring(layouter.namespace(|| format!("row {}", row)), row)?;
            }

            Ok(())
        }
    }

    impl Circuit<Fq> for MyCircuit<Fq> {
        type Config = PubInputsConfig<Fq, 10>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<Fq>) -> Self::Config {
            let table_config = TableConfig::configure(meta);

            let _ = meta.instance_column();
            let endoscalars = meta.instance_column();
            let endoscalars_copy = meta.advice_column();
            let bits = meta.advice_column();

            PubInputsConfig::<Fq, 10>::configure(
                meta,
                endoscalars,
                endoscalars_copy,
                bits,
                table_config,
            )
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<Fq>,
        ) -> Result<(), Error> {
            config.table.load(&mut layouter)?;

            // The max no. of 255-bit scalars that can fit into 1024 rows is 39.
            // Each of these scalars is encoded as 26 endoscalars.
            // We check 26 * 39 = 1014 endoscalars.
            for row in 0..1014 {
                config.get_bitstring(layouter.namespace(|| format!("row {}", row)), row)?;
            }

            Ok(())
        }
    }

    #[test]
    fn test_pub_inputs() {
        use super::super::primitive::endoscale_with_padding;
        use crate::dev::MockProver;
        use ff::{Field, PrimeField, PrimeFieldBits};

        let fp_circuit = MyCircuit::<Fp>(PhantomData);
        let fq_circuit = MyCircuit::<Fq>(PhantomData);

        let fp_pub_inputs = (0..39)
            .map(|_| Fp::random(rand::rngs::OsRng))
            .collect::<Vec<_>>();
        let fq_pub_inputs = (0..39)
            .map(|_| Fq::random(rand::rngs::OsRng))
            .collect::<Vec<_>>();

        let fp_prover = MockProver::run(
            11,
            &fp_circuit,
            vec![
                fp_pub_inputs.clone(),
                super::super::primitive::fp::TABLE
                    .iter()
                    .map(|(_, scalar)| *scalar)
                    .collect(),
            ],
        )
        .unwrap();
        assert_eq!(fp_prover.verify(), Ok(()));

        // Encode fp_pub_inputs as public inputs in Fq.
        let fp_pub_inputs_enc = fp_pub_inputs
            .iter()
            .map(|fp_elem| {
                endoscale_with_padding::<Fq, 10>(
                    &fp_elem
                        .to_le_bits()
                        .iter()
                        .by_val()
                        .take(Fp::NUM_BITS as usize)
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect::<Vec<_>>();
        println!("fp_pub_inputs_enc.len(): {:?}", fp_pub_inputs_enc.len());

        // Provide encoded Fp public inputs as endoscalars to the Fq circuit.
        let fq_prover =
            MockProver::run(11, &fq_circuit, vec![fq_pub_inputs, fp_pub_inputs_enc]).unwrap();
        assert_eq!(fq_prover.verify(), Ok(()));
    }
}
