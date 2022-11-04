use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use ff::Field;
use pasta_curves::{EpAffine, Fp, Fq};
use rand_core::OsRng;
use std::vec;

use halo2_proofs::poly::{
    commitment::{Blind, Params},
    Coeff, EvaluationDomain, Polynomial,
};

fn msm_1(v: &Vec<Fp>) -> Fp {
    let mut t = Fp::one();
    for i in v {
        t = t * i;
    }

    t
}

// Run with RAYON_NUM_THREADS=1 cargo bench --bench msm

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut ref_group = c.benchmark_group("msm ref");
    ref_group.sample_size(500);

    let base: usize = 2;
    for exp in 20..27 {
        let capacity = base.pow(exp);
        let mut v = vec::Vec::with_capacity(capacity);

        for _i in 0..capacity {
            v.push(Fp::random(OsRng));
        }

        ref_group.bench_with_input(BenchmarkId::from_parameter(exp), &v, |b, v| {
            b.iter(|| msm_1(v));
        });
    }
    ref_group.finish();

    let mut pippinger_group = c.benchmark_group("msm Pippenger");
    pippinger_group.sample_size(500);

    for K in 12..19 {
        let params: Params<EpAffine> = Params::new(K);
        let domain = EvaluationDomain::new(1, K);
        let mut px: Polynomial<Fq, Coeff> = domain.empty_coeff();
        for (_i, a) in px.iter_mut().enumerate() {
            *a = Fq::random(OsRng);
        }

        let blind = Blind(Fq::random(OsRng));

        pippinger_group.bench_with_input(
            BenchmarkId::from_parameter(K),
            &(px, blind),
            |b, (px, blind)| {
                b.iter(|| params.commit(px, *blind));
            },
        );
    }
    pippinger_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
