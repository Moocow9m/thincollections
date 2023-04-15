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
extern crate rand_xoshiro;
extern crate test;
extern crate thincollections;

use std::collections::HashMap;

use criterion::{BenchmarkGroup, BenchmarkId, black_box, Criterion, criterion_group, criterion_main};
use criterion::measurement::WallTime;
use rand::*;
use rand::prelude::SliceRandom;
use rand_xoshiro::Xoshiro512StarStar;

use thincollections::thin_hasher::TrivialOneFieldHasherBuilder;
use thincollections::thin_map::ThinMap;

fn create_rand_vec(size: i64) -> Vec<i64> {
    let mut rng1 = Xoshiro512StarStar::seed_from_u64(0x1234_5678_9ABC_DEF1);
    let mut vec = Vec::with_capacity(size as usize);
    for _i in 0..size {
        vec.push(rng1.next_u64() as i64);
    }
    vec
}

fn benchmdr_thin64_get(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    let src = create_rand_vec(*points.last().unwrap() as i64);
    let mut rng1 = Xoshiro512StarStar::seed_from_u64(0x1234_5678_9ABC_DEF1);
    for p in points.iter() {
        let map = create_thin64_from_vec(*p, &src);
        let mut get_src = src.clone();
        get_src.truncate(*p as usize);
        get_src.shuffle(&mut rng1);
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Thin", *p), |b| b.iter(|| get_thin64_from_vec(&map, &get_src)));
    }
}

fn benchmdr_std64_get(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    let src = create_rand_vec(*points.last().unwrap() as i64);
    let mut rng1 = Xoshiro512StarStar::seed_from_u64(0x1234_5678_9ABC_DEF1);
    for p in points.iter() {
        let map = create_std64_from_vec(*p, &src);
        let mut get_src = src.clone();
        get_src.truncate(*p as usize);
        get_src.shuffle(&mut rng1);
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Std", *p), |b| b.iter(|| get_std64_from_vec(&map, &get_src)));
    }
}

fn benchmdr_thin64_insert(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    let src = create_rand_vec(*points.last().unwrap() as i64);
    for p in points.iter() {
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Thin", *p), |b| b.iter(|| create_thin64_from_vec(*p, &src)));
    }
}

fn benchmdr_std64_insert(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    let src = create_rand_vec(*points.last().unwrap() as i64);
    for p in points.iter() {
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Std", *p), |b| b.iter(|| create_std64_from_vec(*p, &src)));
    }
}

fn benchmds_thin64_insert(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    for p in points.iter() {
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Thin", *p), |b| b.iter(|| create_thin(*p, 0)));
    }
}

fn benchmds_std64_insert(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    for p in points.iter() {
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Std", *p), |b| b.iter(|| create_std(*p, 0)));
    }
}

fn benchmds_thin64_get(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    for p in points.iter() {
        let map = create_thin(*p, 0);
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Thin", *p), |b| b.iter(|| get_seq_thin64_var(&map, *p, 0)));
        black_box(map.len());
    }
}

fn benchmds_std64_get(group: &mut BenchmarkGroup<WallTime>) {
    let points = determine_points(4_000_000);
    for p in points.iter() {
        let map = create_std(*p, 0);
        group.throughput(criterion::Throughput::Elements(*p));
        group.bench_function(BenchmarkId::new("Std", *p), |b| b.iter(|| get_seq_std64_var(&map, *p, 0)));
        black_box(map.len());
    }
}

fn benchmpsa_thin_insert(c: &mut Criterion) {
    let size = 1_500_000;
    let mut group = c.benchmark_group("ThinMap seq 64 shifted insert");
    for p in 0..20 {
        group.throughput(criterion::Throughput::Elements(p));
        group.bench_function(BenchmarkId::new("Thin", p), |b| b.iter(|| create_thin_triv(size, p)));
    }
    group.finish();
}

fn get_thin64_from_vec(map: &ThinMap<i64, u64>, keys: &[i64]) {
    let mut sum = 0;
    for x in keys.iter() {
        sum += map.get(x).unwrap();
    }
    black_box(sum);
}

fn get_std64_from_vec(map: &HashMap<i64, u64>, keys: &[i64]) {
    let mut sum = 0;
    for x in keys.iter() {
        let option = map.get(x);
        if option.is_none() {
            println!("looking for {} in {:?} out of {:?}", *x, map, keys);
        }
        sum += option.unwrap();
    }
    black_box(sum);
}

fn create_thin(size: u64, shift: u64) -> ThinMap<i64, u64> {
    let mut thin_map = ThinMap::new();
    let mut c = 0;
    let x = size as i64;
    while c < x {
        thin_map.insert(c << shift, c as u64);
        c = c + 1;
    }
    thin_map
}

fn create_thin_triv(size: u64, shift: u64) -> ThinMap<i64, u64, TrivialOneFieldHasherBuilder> {
    let mut thin_map = ThinMap::with_hasher(TrivialOneFieldHasherBuilder::new());
    let mut c = 0;
    let x = size as i64;
    while c < x {
        thin_map.insert(c << shift, c as u64);
        c = c + 1;
    }
    thin_map
}

fn create_thin64_from_vec(size: u64, v: &[i64]) -> ThinMap<i64, u64> {
    let mut thin_map = ThinMap::new();
    let mut c = 0;
    let x = size as i64;
    let mut it = v.iter();
    while c < x {
        thin_map.insert(*it.next().unwrap(), c as u64);
        c = c + 1;
    }
    thin_map
}

fn create_std64_from_vec(size: u64, v: &[i64]) -> HashMap<i64, u64> {
    let mut std_map = HashMap::new();
    let mut c = 0;
    let x = size as i64;
    let mut it = v.iter();
    while c < x {
        std_map.insert(*it.next().unwrap(), c as u64);
        c = c + 1;
    }
    std_map
}

fn create_std(size: u64, shift: u64) -> HashMap<i64, u64> {
    let mut hash_map = HashMap::new();
    let mut c = 0;
    let x = size as i64;
    while c < x {
        hash_map.insert(c << shift, c as u64);
        c = c + 1;
    }
    hash_map
}

fn get_seq_thin64_var(map: &ThinMap<i64, u64>, size: u64, _shift: u64) {
    let mut c = 1;
    let mut x = 0;
    let y = size as i64;
    while c < y {
        x += map.get(&c).unwrap();
        c = c + 1;
    }
}

fn get_seq_std64_var(map: &HashMap<i64, u64>, size: u64, _shift: u64) {
    let mut c = 1;
    let mut x = 0;
    let y = size as i64;
    while c < y {
        x += map.get(&c).unwrap();
        c = c + 1;
    }
}

fn determine_points(max: u64) -> Vec<u64> {
    let mut thin_map: ThinMap<i64, u64> = ThinMap::with_capacity(10);
    let mut thin_points: Vec<u64> = Vec::new();
    let mut cur_cap = thin_map.capacity();

    thin_points.push(cur_cap as u64);

    while cur_cap < max as usize {
        while thin_map.capacity() <= cur_cap {
            let x = thin_map.len();
            thin_map.insert(x as i64, 1);
        }
        cur_cap = thin_map.capacity();
        thin_points.push(cur_cap as u64);
    }

    let mut hash_map: HashMap<i64, u64> = HashMap::with_capacity(10);
    let mut hash_points: Vec<u64> = Vec::new();
    let mut cur_cap = hash_map.capacity();

    hash_points.push(cur_cap as u64);

    while cur_cap < max as usize {
        while hash_map.capacity() <= cur_cap {
            let x = hash_map.len();
            hash_map.insert(x as i64, 1);
        }
        cur_cap = hash_map.capacity();
        hash_points.push(cur_cap as u64);
    }

    let mut result = Vec::new();
    calc_points(&thin_points, &mut result);
    calc_points(&hash_points, &mut result);

    result.sort();
    result
}

fn calc_points(thin_points: &[u64], result: &mut Vec<u64>) {
//    let mut prev = 0;
    for i in thin_points.iter() {
        result.push(i * 95 / 100);
        result.push(i * 105 / 100);
//        if prev != 0 {
//            result.push((i + prev) / 2);
//        }
//        prev = *i;
    }
}

fn benchmdr_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("64x64 Rnd Get");
    let group_ptr = &mut group;
    benchmdr_thin64_get(group_ptr);
    benchmdr_std64_get(group_ptr);
    group.finish();
}

fn benchmdr_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("64x64 Rnd Insert");
    let group_ptr = &mut group;
    benchmdr_thin64_insert(group_ptr);
    benchmdr_std64_insert(group_ptr);
    group.finish();
}

fn benchmds_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("64x64 Seq Get");
    let group_ptr = &mut group;
    benchmds_thin64_get(group_ptr);
    benchmds_std64_get(group_ptr);
    group.finish();
}

fn benchmds_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("64x64 Seq Insert");
    let group_ptr = &mut group;
    benchmds_thin64_insert(group_ptr);
    benchmds_std64_insert(group_ptr);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmdr_get, benchmdr_insert, benchmds_get, benchmds_insert, benchmpsa_thin_insert
}
criterion_main!(benches);