use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    pasta::Fp,
    plonk::{
        create_proof, keygen_pk, keygen_vk, verify_proof, Advice, Circuit, Column,
        ConstraintSystem, DynamicTable, DynamicTableMap, Error, Expression, Selector,
        SingleVerifier,
    },
    poly::{commitment::Params, Rotation},
    transcript::{Blake2bRead, Blake2bWrite},
};
use pasta_curves::{vesta, EqAffine};
use rand_core::OsRng;

#[derive(Clone)]
struct EvenOddCircuitConfig {
    is_even: Selector,
    a: Column<Advice>,
    // starts at zero to use as default
    table_vals: Column<Advice>,
    even: DynamicTable,
    odd: DynamicTable,
}

#[test]
fn even_odd_dyn_tables() {
    struct DynLookupCircuit {}
    impl Circuit<Fp> for DynLookupCircuit {
        type Config = EvenOddCircuitConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
            let a = meta.advice_column();
            let table_vals = meta.advice_column();
            let is_even = meta.complex_selector();
            let even = meta.create_dynamic_table(&[], &[table_vals]);
            let odd = even.clone();

            meta.lookup_dynamic(&even, |cells, table_ref| {
                let a = cells.query_advice(a, Rotation::cur());
                let is_even = cells.query_selector(is_even);

                DynamicTableMap {
                    selector: is_even,
                    table_map: vec![(
                        a.clone(),
                        table_ref.table_column(table_vals.into()).unwrap(),
                    )],
                }
            });

            meta.lookup_dynamic(&odd, |cells, table_ref| {
                let a = cells.query_advice(a, Rotation::cur());
                let is_even = cells.query_selector(is_even);
                let is_odd = Expression::Constant(Fp::one()) - is_even.clone();

                DynamicTableMap {
                    selector: is_odd,
                    table_map: vec![(
                        a.clone(),
                        table_ref.table_column(table_vals.into()).unwrap(),
                    )],
                }
            });

            EvenOddCircuitConfig {
                a,
                table_vals,
                is_even,
                even,
                odd,
            }
        }

        fn without_witnesses(&self) -> Self {
            Self {}
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<Fp>,
        ) -> Result<(), Error> {
            for i in 0..=5 {
                layouter.assign_region(
                    || "lookup",
                    |mut region| {
                        // Enable the lookup on rows
                        if i % 2 == 0 {
                            config.is_even.enable(&mut region, i)?;
                        };

                        region.assign_advice(
                            || "",
                            config.a,
                            i,
                            || Value::known(Fp::from(i as u64)),
                        )
                    },
                )?;
            }
            layouter.assign_region(
                || "table",
                |mut region| {
                    for i in 0..=5 {
                        region.assign_advice(
                            || "",
                            config.table_vals,
                            i,
                            || Value::known(Fp::from(i as u64)),
                        )?;

                        let table = if i % 2 == 0 {
                            &config.even
                        } else {
                            &config.odd
                        };
                        table.include_row(|| "", &mut region, i)?;
                    }
                    Ok(())
                },
            )?;
            Ok(())
        }
    }
}

struct DynLookupCircuit {}
impl Circuit<Fp> for DynLookupCircuit {
    type Config = EvenOddCircuitConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let table_vals = meta.advice_column();
        let is_even = meta.complex_selector();
        let even = meta.create_dynamic_table(&[], &[table_vals]);
        let odd = even.clone();

        meta.lookup_dynamic(&even, |cells, table_ref| {
            let a = cells.query_advice(a, Rotation::cur());
            let is_even = cells.query_selector(is_even);

            DynamicTableMap {
                selector: is_even,
                table_map: vec![(
                    a.clone(),
                    table_ref.table_column(table_vals.into()).unwrap(),
                )],
            }
        });

        meta.lookup_dynamic(&odd, |cells, table_ref| {
            let a = cells.query_advice(a, Rotation::cur());
            let is_even = cells.query_selector(is_even);
            let is_odd = Expression::Constant(Fp::one()) - is_even.clone();

            DynamicTableMap {
                selector: is_odd,
                table_map: vec![(
                    a.clone(),
                    table_ref.table_column(table_vals.into()).unwrap(),
                )],
            }
        });

        EvenOddCircuitConfig {
            a,
            table_vals,
            is_even,
            even,
            odd,
        }
    }

    fn without_witnesses(&self) -> Self {
        Self {}
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        for i in 0..=5 {
            layouter.assign_region(
                || "lookup",
                |mut region| {
                    // Enable the lookup on rows
                    if i % 2 == 0 {
                        config.is_even.enable(&mut region, i)?;
                    };

                    region.assign_advice(|| "", config.a, i, || Value::known(Fp::from(i as u64)))
                },
            )?;
        }
        layouter.assign_region(
            || "table",
            |mut region| {
                for i in 0..=5 {
                    region.assign_advice(
                        || "",
                        config.table_vals,
                        i,
                        || Value::known(Fp::from(i as u64)),
                    )?;

                    let table = if i % 2 == 0 {
                        &config.even
                    } else {
                        &config.odd
                    };
                    table.include_row(|| "", &mut region, i)?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}

fn main() {
    let k = 5;

    MockProver::run(k, &DynLookupCircuit {}, vec![])
        .unwrap()
        .verify()
        .unwrap();

    let params: Params<EqAffine> = Params::new(k);
    let verifier = SingleVerifier::new(&params);
    let vk = keygen_vk(&params, &DynLookupCircuit {}).unwrap();
    let pk = keygen_pk(&params, vk.clone(), &DynLookupCircuit {}).unwrap();
    let mut transcript = Blake2bWrite::<_, vesta::Affine, _>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[DynLookupCircuit {}],
        &[],
        &mut OsRng,
        &mut transcript,
    )
    .expect("Failed to create proof");

    let proof: Vec<u8> = transcript.finalize();

    let mut transcript = Blake2bRead::init(&proof[..]);
    verify_proof(&params, pk.get_vk(), verifier, &[], &mut transcript)
        .expect("could not verify_proof");
}
