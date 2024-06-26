use std::num::*;

use criterion::{criterion_group, criterion_main, Criterion};

use prime_bag::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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

    for x in u8_arr.iter_mut() {
        *x = rng.gen();
    }
    for x in u16_arr.iter_mut() {
        *x = rng.gen();
    }
    for x in u32_arr.iter_mut() {
        *x = rng.gen();
    }
    for x in u64_arr.iter_mut() {
        *x = rng.gen();
    }
    for x in u128_arr.iter_mut() {
        *x = rng.gen();
    }

    let u8_bags: [PrimeBag8<MyElement>; COUNT] = u8_arr.map(PrimeBag8::from_inner);
    let u16_bags: [PrimeBag16<MyElement>; COUNT] = u16_arr.map(PrimeBag16::from_inner);
    let u32_bags: [PrimeBag32<MyElement>; COUNT] = u32_arr.map(PrimeBag32::from_inner);
    let u64_bags: [PrimeBag64<MyElement>; COUNT] = u64_arr.map(PrimeBag64::from_inner);
    let u128_bags: [PrimeBag128<MyElement>; COUNT] = u128_arr.map(PrimeBag128::from_inner);

    c.bench_function("count_2_3s u8", |b| b.iter(|| count_2_3s_u8(&u8_bags)));
    c.bench_function("count_2_3s u16", |b| b.iter(|| count_2_3s_u16(&u16_bags)));
    c.bench_function("count_2_3s u32", |b| b.iter(|| count_2_3s_u32(&u32_bags)));
    c.bench_function("count_2_3s u64", |b| b.iter(|| count_2_3s_u64(&u64_bags)));
    c.bench_function("count_2_3s u128", |b| {
        b.iter(|| count_2_3s_u128(&u128_bags))
    });

    c.bench_function("Intersect u8", |b| b.iter(|| intersect_all_u8(&u8_bags)));
    c.bench_function("Intersect u16", |b| b.iter(|| intersect_all_u16(&u16_bags)));
    c.bench_function("Intersect u32", |b| b.iter(|| intersect_all_u32(&u32_bags)));
    c.bench_function("Intersect u64", |b| b.iter(|| intersect_all_u64(&u64_bags)));
    c.bench_function("Intersect u128", |b| {
        b.iter(|| intersect_all_u128(&u128_bags))
    });

    c.bench_function("Union u8", |b| b.iter(|| union_all_u8(&u8_bags)));
    c.bench_function("Union u16", |b| b.iter(|| union_all_u16(&u16_bags)));
    c.bench_function("Union u32", |b| b.iter(|| union_all_u32(&u32_bags)));
    c.bench_function("Union u64", |b| b.iter(|| union_all_u64(&u64_bags)));
    c.bench_function("Union u128", |b| b.iter(|| union_all_u128(&u128_bags)));
}

macro_rules! intersect_all {
    ($name: ident, $bag: ty, $inner: ty ) => {
        fn $name<T: PrimeBagElement>(bags: &[$bag]) -> $inner {
            let mut total: $inner = 0;
            for x in 0..(bags.len() - 1) {
                let left = &bags[x];
                let right = &bags[x + 1];

                let intersection = left.intersection(right);
                let inner = intersection.into_inner().get();
                total = total.wrapping_add(inner);
            }
            total
        }
    };
}

macro_rules! union_all {
    ($name: ident, $bag: ty, $inner: ty ) => {
        fn $name<T: PrimeBagElement>(bags: &[$bag]) -> $inner {
            let mut total: $inner = 0;
            for x in 0..(bags.len() - 1) {
                let left = &bags[x];
                let right = &bags[x + 1];

                let union1 = left.try_union(right).unwrap_or_default();
                let inner = union1.into_inner().get();
                total = total.wrapping_add(inner);
            }
            total
        }
    };
}

macro_rules! count_2_3s {
    ($name: ident, $bag: ty, $inner: ty ) => {
        fn $name(bags: &[$bag]) -> (usize, usize) {
            let mut t2: usize = 0;
            let mut t3: usize = 0;
            for bag in bags {
                let c2 = bag.count_instances(MyElement(0));
                let c3 = bag.count_instances(MyElement(1));

                t2 = t2.wrapping_add(c2);
                t3 = t3.wrapping_add(c3);
            }
            (t2, t3)
        }
    };
}

intersect_all!(intersect_all_u8, PrimeBag8<T>, u8);
intersect_all!(intersect_all_u16, PrimeBag16<T>, u16);
intersect_all!(intersect_all_u32, PrimeBag32<T>, u32);
intersect_all!(intersect_all_u64, PrimeBag64<T>, u64);
intersect_all!(intersect_all_u128, PrimeBag128<T>, u128);

union_all!(union_all_u8, PrimeBag8<T>, u8);
union_all!(union_all_u16, PrimeBag16<T>, u16);
union_all!(union_all_u32, PrimeBag32<T>, u32);
union_all!(union_all_u64, PrimeBag64<T>, u64);
union_all!(union_all_u128, PrimeBag128<T>, u128);

count_2_3s!(count_2_3s_u8, PrimeBag8<MyElement>, u8);
count_2_3s!(count_2_3s_u16, PrimeBag16<MyElement>, u16);
count_2_3s!(count_2_3s_u32, PrimeBag32<MyElement>, u32);
count_2_3s!(count_2_3s_u64, PrimeBag64<MyElement>, u64);
count_2_3s!(count_2_3s_u128, PrimeBag128<MyElement>, u128);

pub struct MyElement(usize);

impl PrimeBagElement for MyElement {
    fn to_prime_index(&self) -> usize {
        self.0
    }

    fn from_prime_index(value: usize) -> Self {
        Self(value)
    }
}
