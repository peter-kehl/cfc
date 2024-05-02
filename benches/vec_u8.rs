#![feature(anonymous_lifetime_in_impl_trait)]
//#![feature(type_alias_impl_trait)]

//#![allow(warnings, unused)]
use camigo::prelude::*;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::*;

#[path = "shared/lib_benches.rs"]
mod lib_benches;

/*struct S<'a> {
    i: u8,
    rf: Option<&'a u8>,
}
fn f() -> S<'static> {
    let mut s = S { i: 0, rf: None };
    s.rf = Some(&s.i);
    s
}*/

pub fn bench_target(c: &mut Criterion) {
    /*let mut rng = Rng::new();

    type IdState = ();

    fn generate_item(rng: &mut Rng, _: &mut IdState) -> u8 {
        rng.u8(..)
    }

    fn id_postfix(_: &IdState) -> String {
        String::new()
    }

    let mut id_state: IdState = ();

    bench_vec_sort_bin_search::<u8, u8, VecToVecMovedOuterHolder, Rng, ()>(
        c,
        &mut rng,
        "u8",
        &mut id_state,
        id_postfix,
        generate_item,
    );*/
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_target
}
// Based on expansion of `criterion_main!(benches);`
fn main() {
    benches();

    Criterion::default().configure_from_args().final_summary();
}
