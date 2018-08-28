#[macro_use]
extern crate criterion;
//extern crate rand;
//extern crate test_work_price_list;

use criterion::Criterion;
//use test_work_price_list::{PriceList, Size, Meta, Inner};


//fn gen_size_meta(times: u32) -> Vec<(Size, Meta)> {
//    (0..times).map(|_| (rand::random(), rand::random::<u64>() as u128)).collect()
//}


fn bench_alloc(c: &mut Criterion) {

    c.bench_function("alloc", |b| {


        b.iter(|| {
            let mut a = Vec::<i32>::with_capacity(10);
//            let mut a = Vec::<i32>::new();
            a.push(1);
//            let y = 1 + 1;
        });


    });

}


//fn bench_split(c: &mut Criterion) {
//
//    c.bench_function("split", |b| {
//
//        let mut inner: Inner = vec![];
//        for _i in 0..1000 {
//            inner.push((rand::random(), gen_size_meta(100)));
//        }
//
//        let mut price_list = PriceList::new();
//
//        for i in &inner {
//            for j in &i.1 {
//                price_list.add(i.0, *j);
//            }
//        }
//
//        b.iter(|| {
//            price_list.split(rand::random(), rand::random::<u32>() as u128);
//        });
//
//    });
//
//}



criterion_group!(benches, bench_alloc);
//criterion_group!(benches, bench_price_list_add);
criterion_main!(benches);