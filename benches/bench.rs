#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
use bimap::BiHashMap;

#[bench]
fn bench_hash_map_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut map: BiHashMap<String, i64> = BiHashMap::new();

        for i in 1i64..100 {
            map.insert(format!("key {}", i), i);
        }

        black_box(map);
    });
}

#[bench]
fn bench_hash_map_get_left(b: &mut Bencher) {
    let mut map: BiHashMap<String, i64> = BiHashMap::new();

    for i in 1i64..100 {
        map.insert(format!("key {}", i), i);
    }

    b.iter(|| {
        for i in 1i64..100 {
            black_box(map.get_by_left(&format!("key {}", i)));
        }
    });
}

#[bench]
fn bench_hash_map_get_right(b: &mut Bencher) {
    let mut map: BiHashMap<String, i64> = BiHashMap::new();

    for i in 1i64..100 {
        map.insert(format!("key {}", i), i);
    }

    b.iter(|| {
        for i in 1i64..100 {
            black_box(map.get_by_right(&i));
        }
    });
}

#[bench]
fn bench_hash_map_iter(b: &mut Bencher) {
    let mut map: BiHashMap<String, i64> = BiHashMap::new();

    for i in 1i64..100 {
        map.insert(format!("key {}", i), i);
    }

    b.iter(|| {
        map.iter().for_each(|(l, r)| {
            black_box((l, r));
        });
    });
}