use std::str::FromStr;
use vm::Vm;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn sum_of_primes(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("sum-of-primes")).unwrap();
    vm.input(100000);
    let f = || vm.clone().run_until().unwrap();
    assert!(f() == 454396537);
    c.bench_function("sum-of-primes", |b| b.iter(f));
}

pub fn ackerman(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("ackerman")).unwrap();
    vm.input(3);
    vm.input(6);
    let f = || vm.clone().run_until().unwrap();
    assert!(f() == 509);
    c.bench_function("ackerman", |b| b.iter(f));
}

pub fn isqrt(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("isqrt")).unwrap();
    vm.input(130);
    let f = || vm.clone().run_until().unwrap();
    assert!(f() == 11);
    c.bench_function("isqrt", |b| b.iter(f));
}

pub fn divmod(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("divmod")).unwrap();
    vm.input(1024);
    vm.input(3);
    let f = || {
        let mut vm = vm.clone();
        let a = vm.run_until().unwrap();
        let b = vm.run_until().unwrap();
        (a, b)
    };
    assert!(f() == (341, 1));
    c.bench_function("divmod", |b| b.iter(f));
}

pub fn factor_small_prime(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("factor")).unwrap();
    let i = 2147483647;
    vm.input(i);
    let f = || vm.clone().run_until().unwrap();
    assert!(f() == i);
    c.bench_function(&format!("factor {}", i), |b| b.iter(f));
}

pub fn factor_large_composite(c: &mut Criterion) {
    let mut vm = Vm::from_str(include_str!("factor")).unwrap();
    let i = 19201644899;
    vm.input(i);
    let f = || {
        let mut vm = vm.clone();
        let a = vm.run_until().unwrap();
        let b = vm.run_until().unwrap();
        (a, b)
    };
    assert!(f() == (138569, 138571));
    c.bench_function(&format!("factor {}", i), |b| b.iter(f));
}

criterion_group!{
    name = fast;
    config = Criterion::default();
    targets = isqrt,
              divmod,
}
criterion_group!{
    name = slow;
    config = Criterion::default().sample_size(10);
    targets = sum_of_primes,
              ackerman,
              factor_small_prime,
              factor_large_composite,
}
criterion_main!(fast, slow);
