//#![allow(warnings, unused)]
use camigo::prelude::*;
use core::iter;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::*;

#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type ID = usize;

    fn generate_item(rng: &mut Rng, total_length: &mut ID) -> String {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<char>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.char(..)).take(item_len));

        let mut string = String::with_capacity(4 * item_len);
        string.extend(item.into_iter());
        *total_length += string.len();
        string
    }

    fn id_postfix(total_length: &ID) -> String {
        format!("Sum len: {total_length}.")
    }

    let mut total_length: ID = 0; // NOT in chars, but in bytes.

    bench_vec_sort_bin_search(
        c,
        &mut rng,
        "strings",
        &mut total_length,
        id_postfix,
        generate_item,
    );
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
