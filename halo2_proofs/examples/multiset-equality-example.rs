use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector},
    poly::Rotation,
};
use pasta_curves::Fp;

#[derive(Default)]
struct EqSetCircuit {}

struct EqSetChip {
    config: EqSetConfig,
}

#[derive(Clone, Debug)]
struct EqSetConfig {
    input: Column<Instance>,
    input_copy: Column<Advice>,
    permutation: Column<Fixed>,
}

impl Chip<Fp> for EqSetChip {
    type Config = EqSetConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl EqSetChip {
    fn new(config: EqSetConfig) -> Self {
        EqSetChip { config }
    }

    fn configure<F: FieldExt>(meta: &mut ConstraintSystem<F>) -> EqSetConfig {
        let input = meta.instance_column();
        let input_copy = meta.advice_column();
        let permutation = meta.fixed_column();

        meta.enable_equality(input);
        meta.enable_equality(input_copy);

        meta.multiset_equality(|region| {
            let input = region.query_advice(input_copy, Rotation::cur());
            let perm = region.query_fixed(permutation, Rotation::cur());
            vec![(input, perm)]
        });

        EqSetConfig {
            input,
            input_copy,
            permutation,
        }
    }
}

impl<F: FieldExt> Circuit<F> for EqSetCircuit {
    type Config = EqSetConfig;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        EqSetChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "input",
            |mut region| {
                for offset in 0..5 {
                    region.assign_advice_from_instance(
                        || "",
                        config.input,
                        offset,
                        config.input_copy,
                        offset,
                    )?;

                    region.assign_fixed(
                        || "",
                        config.permutation,
                        offset,
                        || Value::known(F::from(offset as u64)),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}

fn main() {
    let k = 4;

    let input = [4, 2, 0, 1, 3].map(Fp::from).to_vec();
    let prover = MockProver::run(k, &EqSetCircuit {}, vec![input]).unwrap();
    assert!(prover.verify().is_ok());

    let input = [10].map(Fp::from).to_vec();
    let prover = MockProver::run(k, &EqSetCircuit {}, vec![input]).unwrap();
    assert!(prover.verify().is_ok());
}
