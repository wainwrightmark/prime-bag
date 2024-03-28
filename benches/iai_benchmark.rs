use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;
use std::num::*;

use prime_bag::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct MyElement(usize);

impl PrimeBagElement for MyElement {
    fn to_prime_index(&self) -> usize {
        self.0
    }

    fn from_prime_index(value: usize) -> Self {
        Self(value)
    }
}

const COUNT: usize = 100;

macro_rules! get_random_bags {
    ($name: ident,  $inner: ty, $bag: ty) => {
        fn $name() -> [$bag; COUNT] {
            let mut rng = StdRng::seed_from_u64(12345);

            let mut arr = [<$inner>::MIN; COUNT];
            for x in arr.iter_mut() {
                *x = rng.gen();
            }

            black_box(arr.map(|x| <$bag>::from_inner(x)))
        }
    };
}

macro_rules! intersect_all {
    ($name: ident, $get_bags: ident, $bag: ty, $inner: ty ) => {
        #[library_benchmark]
        #[bench::go($get_bags())]
        fn $name(bags: [$bag; COUNT]) -> $inner {
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
    ($name: ident, $get_bags: ident, $bag: ty, $inner: ty ) => {
        #[library_benchmark]
        #[bench::go($get_bags())]
        fn $name(bags: [$bag; COUNT]) -> $inner {
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
    ($name: ident, $get_bags: ident, $bag: ty, $inner: ty ) => {
        #[library_benchmark]
        #[bench::go($get_bags())]
        fn $name(bags: [$bag; COUNT]) -> (usize, usize) {
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
#[library_benchmark]
fn count_is_at_least() -> usize {
    let mut t = 0usize;
    let bag =
        PrimeBag64::<MyElement>::try_from_iter([0, 0, 0, 1, 1, 8].map(|x| MyElement(x))).unwrap();

    loop {
        if !black_box(bag).is_count_at_least(black_box(t)) {
            return t;
        } else {
            t += 1;
        }
    }
}

get_random_bags!(get_bags_u8, NonZeroU8, PrimeBag8<MyElement>);
get_random_bags!(get_bags_u16, NonZeroU16, PrimeBag16<MyElement>);
get_random_bags!(get_bags_u32, NonZeroU32, PrimeBag32<MyElement>);
get_random_bags!(get_bags_u64, NonZeroU64, PrimeBag64<MyElement>);
get_random_bags!(get_bags_u128, NonZeroU128, PrimeBag128<MyElement>);

intersect_all!(intersect_all_u8, get_bags_u8, PrimeBag8<MyElement>, u8);
intersect_all!(intersect_all_u16, get_bags_u16, PrimeBag16<MyElement>, u16);
intersect_all!(intersect_all_u32, get_bags_u32, PrimeBag32<MyElement>, u32);
intersect_all!(intersect_all_u64, get_bags_u64, PrimeBag64<MyElement>, u64);
intersect_all!(
    intersect_all_u128,
    get_bags_u128,
    PrimeBag128<MyElement>,
    u128
);

union_all!(union_all_u8, get_bags_u8, PrimeBag8<MyElement>, u8);
union_all!(union_all_u16, get_bags_u16, PrimeBag16<MyElement>, u16);
union_all!(union_all_u32, get_bags_u32, PrimeBag32<MyElement>, u32);
union_all!(union_all_u64, get_bags_u64, PrimeBag64<MyElement>, u64);
union_all!(union_all_u128, get_bags_u128, PrimeBag128<MyElement>, u128);

count_2_3s!(count_2_3s_u8, get_bags_u8, PrimeBag8<MyElement>, u8);
count_2_3s!(count_2_3s_u16, get_bags_u16, PrimeBag16<MyElement>, u16);
count_2_3s!(count_2_3s_u32, get_bags_u32, PrimeBag32<MyElement>, u32);
count_2_3s!(count_2_3s_u64, get_bags_u64, PrimeBag64<MyElement>, u64);
count_2_3s!(count_2_3s_u128, get_bags_u128, PrimeBag128<MyElement>, u128);

library_benchmark_group!(
    name = counts;
    benchmarks = count_is_at_least, count_2_3s_u8, count_2_3s_u16, count_2_3s_u32, count_2_3s_u64, count_2_3s_u128
);

library_benchmark_group!(
    name = union_all;
    benchmarks = union_all_u8, union_all_u16, union_all_u32, union_all_u64, union_all_u128
);

library_benchmark_group!(
    name = intersect_all;
    benchmarks = intersect_all_u8, intersect_all_u16, intersect_all_u32, intersect_all_u64, intersect_all_u128
);

main!(library_benchmark_groups = counts, union_all, intersect_all);
