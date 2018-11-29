#![feature(test)]
extern crate test;
extern crate unorm;

use test::{Bencher, black_box};
use unorm::Unorm;

#[bench]
fn empty(b: &mut Bencher){
    b.iter(|| 1)
}

#[bench]
fn multiplaction_unorm_unorm(b: &mut Bencher){
    let x = black_box(Unorm::from_rational(2,3));
    let y = black_box(Unorm::from_rational(1,4));

    b.iter(||x * y)
}

#[bench]
fn multiplaction_f64_f64(b: &mut Bencher){
    let x = black_box(2f64/3.);
    let y = black_box(1f64/4.);

    b.iter(||x * y)
}

#[bench]
fn multiplaction_one(b: &mut Bencher){
    let x = black_box(unorm::ONE);
    let y = black_box(unorm::ONE);

    b.iter(||x * y)
}

#[bench]
fn multiplaction_zero(b: &mut Bencher){
    let x = black_box(unorm::ZERO);
    let y = black_box(unorm::ZERO);

    b.iter(||x * y)
}