use super::{PubInputsInstructions, PubInputsLookup};
use halo2_proofs::{
    circuit::{AssignedCell, Layouter},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};
use pasta_curves::{arithmetic::FieldExt, Fp, Fq};
use std::convert::TryInto;

fn lebs2ip<const K: usize>(bits: &[bool; K]) -> u64 {
    assert!(K <= 64);
    bits.iter()
        .enumerate()
        .fold(0u64, |acc, (i, b)| acc + if *b { 1 << i } else { 0 })
}

impl PubInputsLookup<10, 1024> for Fp {
    fn table() -> [([bool; 10], Self); 1024] {
        super::super::primitive::fp::TABLE
    }
}

impl PubInputsLookup<10, 1024> for Fq {
    fn table() -> [([bool; 10], Self); 1024] {
        super::super::primitive::fq::TABLE
    }
}

impl<const K: usize> From<&InnerWord<K>> for Assigned<Fp> {
    fn from(bits: &InnerWord<K>) -> Assigned<Fp> {
        assert!(K <= 64);
        Fp::from(lebs2ip(&bits.0)).into()
    }
}

impl<const K: usize> From<&InnerWord<K>> for Assigned<Fq> {
    fn from(bits: &InnerWord<K>) -> Assigned<Fq> {
        assert!(K <= 64);
        Fq::from(lebs2ip(&bits.0)).into()
    }
}

#[derive(Debug, Clone)]
pub struct InnerWord<const K: usize>([bool; K]);

/// An N-bit word in the public input.
#[derive(Debug, Clone)]
pub struct Word<F: FieldExt, const K: usize>(AssignedCell<InnerWord<K>, F>);

/// An endoscalar representing an N-bit word in the public input.
#[derive(Debug)]
pub struct Endoscalar<F: FieldExt>(AssignedCell<F, F>);

/// Columns used in processing public inputs.
#[derive(Copy, Clone, Debug)]
pub struct PubInputsConfig<F: FieldExt, const K: usize> {
    // Selector enabling the lookup.
    q_lookup: Selector,
    // Public inputs are provided as endoscalars. Each endoscalar corresponds
    // to an N-bit chunk.
    endoscalars: Column<Instance>,
    // An additional advice column where endoscalar values are copied and used
    // in the lookup argument.
    endoscalars_copy: Column<Advice>,
    // The N-bit chunk representations of the public inputs.
    // These are used in-circuit for scalar multiplication.
    bits: Column<Advice>,
    table: lookup::TableConfig<F, K>,
}

impl<F: FieldExt, const K: usize> PubInputsConfig<F, K> {
    #[allow(dead_code)]
    pub(crate) fn configure(
        meta: &mut ConstraintSystem<F>,
        endoscalars: Column<Instance>,
        endoscalars_copy: Column<Advice>,
        bits: Column<Advice>,
        table: lookup::TableConfig<F, K>,
    ) -> Self {
        let config = Self {
            q_lookup: meta.complex_selector(),
            endoscalars,
            endoscalars_copy,
            bits,
            table,
        };

        meta.enable_equality(config.endoscalars);
        meta.enable_equality(config.endoscalars_copy);

        meta.lookup(|meta| {
            use halo2_proofs::plonk::Expression;

            let q_lookup = meta.query_selector(config.q_lookup);
            let bits = meta.query_advice(config.bits, Rotation::cur());
            let endoscalars = meta.query_advice(config.endoscalars_copy, Rotation::cur());

            // Endoscalar that maps to the K-string of 0's.
            let default = {
                use super::super::primitive::endoscale;
                Expression::<F>::Constant(endoscale([false; K]))
            };
            let not_q_lookup = Expression::Constant(F::one()) - q_lookup.clone();

            vec![
                (q_lookup.clone() * bits, table.bits),
                (
                    q_lookup * endoscalars + not_q_lookup * default,
                    table.endoscalar,
                ),
            ]
        });

        config
    }
}

impl PubInputsInstructions<Fp, 10, 1024> for PubInputsConfig<Fp, 10> {
    type Word = Word<Fp, 10>;
    type Endoscalar = Endoscalar<Fp>;

    fn get_bitstring(
        &self,
        mut layouter: impl Layouter<Fp>,
        row: usize,
    ) -> Result<(Self::Word, Self::Endoscalar), Error> {
        layouter.assign_region(
            || "get_bitstring",
            |mut region| {
                // Copy endoscalar in from instance column
                let scalar = region
                    .assign_advice_from_instance(
                        || "copy endoscalar",
                        self.endoscalars,
                        row,
                        self.endoscalars_copy,
                        0,
                    )
                    .map(Endoscalar)?;

                let bitstring = scalar.0.value().map(|scalar| {
                    // Look up the bitstring corresponding to the endoscalar
                    Fp::table()
                        .iter()
                        .find(|(_, table_scalar)| scalar == table_scalar)
                        .expect("should have found scalar")
                        .0
                });

                // Enable lookup
                self.q_lookup.enable(&mut region, 0)?;

                // Witness bitstring
                let word = {
                    region
                        .assign_advice(
                            || format!("row {}", row),
                            self.bits,
                            0,
                            || bitstring.ok_or(Error::Synthesis),
                        )
                        .map(Word)?
                };

                Ok((word, scalar))
            },
        )
    }
}

impl PubInputsInstructions<Fq, 10, 1024> for PubInputsConfig<Fq, 10> {
    type Word = Word<Fq, 10>;
    type Endoscalar = Endoscalar<Fq>;

    fn get_bitstring(
        &self,
        mut layouter: impl Layouter<Fq>,
        row: usize,
    ) -> Result<(Self::Word, Self::Endoscalar), Error> {
        layouter.assign_region(
            || "get_bitstring",
            |mut region| {
                // Copy endoscalar in from instance column
                let scalar = region
                    .assign_advice_from_instance(
                        || "copy endoscalar",
                        self.endoscalars,
                        row,
                        self.endoscalars_copy,
                        0,
                    )
                    .map(Endoscalar)?;

                let bitstring = scalar.0.value().map(|scalar| {
                    // Look up the bitstring corresponding to the endoscalar
                    Fq::table()
                        .iter()
                        .find(|(_, table_scalar)| scalar == table_scalar)
                        .expect("should have found scalar")
                        .0
                });

                // Enable lookup
                self.q_lookup.enable(&mut region, 0)?;

                // Witness bitstring
                let word = {
                    region
                        .assign_advice(
                            || format!("row {}", row),
                            self.bits,
                            0,
                            || bitstring.ok_or(Error::Synthesis),
                        )
                        .map(Word)?
                };

                Ok((word, scalar))
            },
        )
    }
}

pub(crate) mod lookup {
    use crate::recursion::pub_inputs::primitive::{endoscale, i2lebsp};
    use halo2_proofs::{
        circuit::Layouter,
        plonk::{ConstraintSystem, Error, TableColumn},
    };
    use pasta_curves::arithmetic::FieldExt;
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Debug)]
    pub struct TableConfig<F: FieldExt, const K: usize> {
        pub(in crate::recursion) bits: TableColumn,
        pub(in crate::recursion) endoscalar: TableColumn,
        _marker: PhantomData<F>,
    }

    impl<F: FieldExt, const K: usize> TableConfig<F, K> {
        #[allow(dead_code)]
        pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
            TableConfig {
                bits: meta.lookup_table_column(),
                endoscalar: meta.lookup_table_column(),
                _marker: PhantomData,
            }
        }

        #[allow(dead_code)]
        pub fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
            layouter.assign_table(
                || "endoscalar_map",
                |mut table| {
                    for index in 0..(1 << K) {
                        table.assign_cell(
                            || "bits",
                            self.bits,
                            index,
                            || Ok(F::from(index as u64)),
                        )?;
                        table.assign_cell(
                            || "endoscalar",
                            self.endoscalar,
                            index,
                            || Ok(endoscale::<F, K>(i2lebsp(index as u64))),
                        )?;
                    }
                    Ok(())
                },
            )
        }
    }
}
