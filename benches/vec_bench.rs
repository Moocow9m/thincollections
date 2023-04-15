// Copyright 2018 Mohammad Rezaei.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
#![feature(test)]

extern crate rand;
extern crate test;
#[macro_use]
extern crate thincollections;

use std::rc::Rc;
use test::stats::Summary;

use criterion::{Bencher, black_box, Criterion, criterion_group, criterion_main};

use thincollections::thin_v64::V64;
use thincollections::thin_vec::ThinVec;

fn benchv_v64_1m_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut vec: V64<u64> = V64::with_capacity(16);
        for i in 0..1_000_000 {
            vec.push(i);
        }
        black_box(vec.len());
    });
}

pub fn benchv_vec_1m_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut vec: Vec<u64> = Vec::with_capacity(16);
        for i in 0..1_000_000 {
            vec.push(i);
        }
        black_box(vec.len());
    });
}

pub fn benchv_thinvec_1m_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut vec: ThinVec<u64> = ThinVec::with_capacity(16);
        for i in 0..1_000_000 {
            vec.push(i);
        }
        black_box(vec.len());
    });
}

pub fn benchv_thinthin(b: &mut Bencher) {
    b.iter(|| {
        let mut vv: ThinVec<ThinVec<u32>> = ThinVec::with_capacity(16);
        for i in 0..1_000_000 {
            let mut v: ThinVec<u32> = ThinVec::new();
            if i % 2 == 0 {
                v.push(1);
            }
            if i % 3 == 0 {
                v.push(1);
            }
            vv.push(v);
        }
        black_box(vv.len());
    });
}

pub fn benchv_thinv64(b: &mut Bencher) {
    b.iter(|| {
        let mut vv: ThinVec<V64<u32>> = ThinVec::with_capacity(16);
        for i in 0..1_000_000 {
            let mut v: V64<u32> = V64::new();
            if i % 2 == 0 {
                v.push(1);
            }
            if i % 3 == 0 {
                v.push(1);
            }
            vv.push(v);
        }
        black_box(vv.len());
    });
}

pub fn benchv_vecvec(b: &mut Bencher) {
    b.iter(|| {
        let mut vv: Vec<Vec<u32>> = Vec::with_capacity(16);
        for i in 0..1_000_000 {
            let mut v: Vec<u32> = Vec::new();
            if i % 2 == 0 {
                v.push(1);
            }
            if i % 3 == 0 {
                v.push(1);
            }
            vv.push(v);
        }
        black_box(vv.len());
    });
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}

fn powerset_thin<T>(s: &[T]) -> ThinVec<ThinVec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}

pub fn benchp_vec(b: &mut Bencher) {
    b.iter(|| {
        let v: Vec<i32> = vec![1; 20];
        let x = powerset(&v);
        black_box(x.len());
    })
}

pub fn benchp_thin(b: &mut Bencher) {
    b.iter(|| {
        let v: ThinVec<i32> = thinvec![1; 20];
        let x = powerset_thin(&v);
        black_box(x.len());
    })
}

pub fn benchprc_vec(b: &mut Bencher) {
    b.iter(|| {
        let v: Vec<_> = vec![Rc::new("b"); 20];
        let x = powerset(&v);
        black_box(x.len());
    })
}

pub fn benchprc_thin(b: &mut Bencher) {
    b.iter(|| {
        let v: ThinVec<_> = thinvec![Rc::new("b"); 20];
        let x = powerset_thin(&v);
        black_box(x.len());
    })
}

fn benchv_1m_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec 1M insert");
    group.bench_function("V64", benchv_v64_1m_insert);
    group.bench_function("Vec", benchv_vec_1m_insert);
    group.bench_function("ThinVec", benchv_thinvec_1m_insert);
    group.finish();
}

fn benchv_type_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec Type insert");
    group.bench_function("ThinThin", benchv_thinthin);
    group.bench_function("ThinV64", benchv_thinv64);
    group.bench_function("VecVec", benchv_vecvec);
    group.finish();
}

fn benchv_powerset_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec PowerSet insert");
    group.bench_function("Vec", benchp_vec);
    group.bench_function("Thin", benchp_thin);
    group.finish();
}

fn benchv_powerset_rc_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec PowerSet insert");
    group.bench_function("Vec", benchprc_vec);
    group.bench_function("Thin", benchprc_thin);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchv_1m_insert, benchv_type_insert, benchv_powerset_insert, benchv_powerset_rc_insert
}
criterion_main!(benches);