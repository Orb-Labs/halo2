use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{
        create_proof, keygen_pk, keygen_vk, verify_proof, Advice, Circuit, Column,
        ConstraintSystem, Error, Fixed, Instance, Selector, SingleVerifier,
    },
    poly::{commitment::Params, Rotation},
    transcript::{Blake2bRead, Blake2bWrite},
};
use pasta_curves::{vesta, EqAffine, Fp};
use rand_core::OsRng;

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
                    // region.assign_advice_from_instance(
                    //     || "",
                    //     config.input,
                    //     offset,
                    //     config.input_copy,
                    //     offset,
                    // )?;
                    region.assign_advice(
                        || "",
                        config.input_copy,
                        offset,
                        || Value::known(F::from(offset as u64)),
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

    let params: Params<EqAffine> = Params::new(k);
    let vk = keygen_vk(&params, &EqSetCircuit::default()).unwrap();

    let pk = keygen_pk(&params, vk, &EqSetCircuit::default()).unwrap();
    let mut transcript = Blake2bWrite::<_, vesta::Affine, _>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[EqSetCircuit::default()],
        &[&[&[]]],
        &mut OsRng,
        &mut transcript,
    )
    .expect("Failed to create proof");
    let proof = transcript.finalize();

    let mut transcript = Blake2bRead::init(&proof[..]);

    let verifier = SingleVerifier::new(&params);
    verify_proof(&params, pk.get_vk(), verifier, &[&[&[]]], &mut transcript)
        .expect("could not verify_proof");
}
