use std::num::*;

use criterion::{criterion_group, criterion_main, Criterion};
use prime_bag::prime_bag_element::*;
use prime_bag::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    const COUNT: usize = 100;
    let mut rng = StdRng::seed_from_u64(12345);
    let mut u8_arr = [NonZeroU8::MIN; COUNT];
    let mut u16_arr = [NonZeroU16::MIN; COUNT];
    let mut u32_arr = [NonZeroU32::MIN; COUNT];
    let mut u64_arr = [NonZeroU64::MIN; COUNT];
    let mut u128_arr = [NonZeroU128::MIN; COUNT];

    for x in u8_arr.iter_mut(){
        *x = rng.gen();
    }for x in u16_arr.iter_mut(){
        *x = rng.gen();
    }for x in u32_arr.iter_mut(){
        *x = rng.gen();
    }for x in u64_arr.iter_mut(){
        *x = rng.gen();
    }for x in u128_arr.iter_mut(){
        *x = rng.gen();
    }

    let u8_bags: [PrimeBag8<MyElement>; COUNT] = u8_arr.map(|x|PrimeBag8::from_inner(x));
    let u16_bags: [PrimeBag16<MyElement>; COUNT] = u16_arr.map(|x|PrimeBag16::from_inner(x));
    let u32_bags: [PrimeBag32<MyElement>; COUNT] = u32_arr.map(|x|PrimeBag32::from_inner(x));
    let u64_bags: [PrimeBag64<MyElement>; COUNT] = u64_arr.map(|x|PrimeBag64::from_inner(x));
    let u128_bags: [PrimeBag128<MyElement>; COUNT] = u128_arr.map(|x|PrimeBag128::from_inner(x));



    c.bench_function("Intersect u8", |b| b.iter(|| intersect_all_u8(&u8_bags)));
    c.bench_function("Intersect u16", |b| b.iter(|| intersect_all_u16(&u16_bags)));
    c.bench_function("Intersect u32", |b| b.iter(|| intersect_all_u32(&u32_bags)));
    c.bench_function("Intersect u64", |b| b.iter(|| intersect_all_u64(&u64_bags)));
    c.bench_function("Intersect u128", |b| b.iter(|| intersect_all_u128(&u128_bags)));
}

fn intersect_all_u8<T: PrimeBagElement>(bags: &[PrimeBag8<T>])-> u8{
    let mut total = 0u8;
    for x in 0..(bags.len() -1){
        let left =  &bags[x];
        let right = &bags[x + 1];

        let intersection = left.intersection(right);
        let inner = intersection.into_inner().get();
        total =  total.wrapping_add(inner);
    }
    total
}

fn intersect_all_u16<T: PrimeBagElement>(bags: &[PrimeBag16<T>])-> u16{
    let mut total = 0u16;
    for x in 0..(bags.len() -1){
        let left =  &bags[x];
        let right = &bags[x + 1];

        let intersection = left.intersection(right);
        let inner = intersection.into_inner().get();
        total =  total.wrapping_add(inner);
    }
    total
}

fn intersect_all_u32<T: PrimeBagElement>(bags: &[PrimeBag32<T>])-> u32{
    let mut total = 0u32;
    for x in 0..(bags.len() -1){
        let left =  &bags[x];
        let right = &bags[x + 1];

        let intersection = left.intersection(right);
        let inner = intersection.into_inner().get();
        total =  total.wrapping_add(inner);
    }
    total
}


fn intersect_all_u64<T: PrimeBagElement>(bags: &[PrimeBag64<T>])-> u64{
    let mut total = 0u64;
    for x in 0..(bags.len() -1){
        let left =  &bags[x];
        let right = &bags[x + 1];

        let intersection = left.intersection(right);
        let inner = intersection.into_inner().get();
        total =  total.wrapping_add(inner);
    }
    total
}

fn intersect_all_u128<T: PrimeBagElement>(bags: &[PrimeBag128<T>])-> u128{
    let mut total = 0u128;
    for x in 0..(bags.len() -1){
        let left =  &bags[x];
        let right = &bags[x + 1];

        let intersection = left.intersection(right);
        let inner = intersection.into_inner().get();
        total =  total.wrapping_add(inner);
    }
    total
}

pub struct MyElement(usize);

impl PrimeBagElement for MyElement {
    fn into_prime_index(&self) -> usize {
        self.0
    }

    fn from_prime_index(value: usize) -> Self {
        Self(value)
    }
}