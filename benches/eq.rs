#![feature(test)]

extern crate test;
extern crate seckey;

use test::Bencher;
use seckey::{ Bytes, TempKey };


#[bench]
fn key_eq_bench(b: &mut Bencher) {
    let x = TempKey::from([9i32; 4096]);
    let y = [9i32; 4096];

    b.iter(|| x == y);
}

#[bench]
fn key_ne_bench(b: &mut Bencher) {
    let x = TempKey::from([8i32; 4096]);
    let z = [33i32; 4096];

    b.iter(|| x == z);
}

#[bench]
fn bytes_eq_bench(b: &mut Bencher) {
    let x = Bytes::new(&[9u8; 4096]);
    let y = vec![9u8; 4096];

    b.iter(|| x == y);
}

#[bench]
fn bytes_ne_bench(b: &mut Bencher) {
    let x = Bytes::new(&[8u8; 4096]);
    let z = vec![33u8; 4096];

    b.iter(|| x == z);
}
