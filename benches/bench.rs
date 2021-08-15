use cpython::PyDict;
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion, Throughput};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{fmt::Write as _, iter, time::Duration};

#[derive(Serialize, Deserialize)]
struct DataOwned {
    name: String,
    value: u16,
}

#[derive(Serialize, Deserialize)]
struct DataBorrowed<'a> {
    name: &'a str,
    value: u16,
}

fn json_input(count: u64) -> String {
    let mut s = String::new();
    let mut rng = thread_rng();

    for _ in 0..count {
        let value: u16 = rng.gen();
        let name = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(7)
            .collect();

        let d = DataOwned { name, value };

        writeln!(&mut s, "{}", serde_json::to_string(&d).unwrap()).unwrap();
    }

    s
}

fn bench_py(input: &str, setup_code: &str, run_code: &str, b: &mut Bencher) {
    let gil = cpython::Python::acquire_gil();
    let python = gil.python();

    let locals = PyDict::new(python);
    locals.set_item(python, "INPUT", &input).unwrap();

    python.run(setup_code, None, None).unwrap();

    b.iter(|| black_box(python.run(run_code, None, Some(&locals)).unwrap()));
}

fn build_number_list(count: u64) -> String {
    let mut s = String::new();
    let mut rng = thread_rng();

    for _ in 0..count {
        let num: u16 = rng.gen();
        writeln!(&mut s, "{}", num).unwrap();
    }

    s
}

fn numbers_group(c: &mut Criterion) {
    let count = 1_000;

    let numbers = build_number_list(count);

    let mut group = c.benchmark_group("numbers");
    group.throughput(Throughput::Elements(count));

    group.bench_function("python", |b| {
        bench_py(
            &numbers,
            "",
            r#"
s = 0
for line in INPUT.splitlines():
    value = int(line.strip())
    s += value
           "#,
            b,
        )
    });

    group.bench_function("single allocation", |b| {
        b.iter(|| {
            let mut sum = 0;
            for l in numbers.lines() {
                let num: u64 = l.trim().to_string().parse().unwrap();
                sum += num;
            }

            black_box(sum);
        });
    });

    group.bench_function("double allocation", |b| {
        b.iter(|| {
            let mut sum = 0;
            for l in numbers.lines() {
                let num: u64 = l.to_string().trim().to_string().parse().unwrap();
                sum += num;
            }

            black_box(sum);
        });
    });

    group.bench_function("no allocation", |b| {
        b.iter(|| {
            let mut sum = 0;
            for l in numbers.lines() {
                let num: u64 = l.trim().parse().unwrap();
                sum += num;
            }

            black_box(sum);
        });
    });
}

fn json_group(c: &mut Criterion) {
    let count = 1_000;

    let input = json_input(count);

    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Elements(count));

    group.bench_function("owned", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for l in input.lines() {
                let data: DataOwned = serde_json::from_str(l).unwrap();
                sum += data.value as u64;
                sum += data.name.len() as u64;
            }

            black_box(sum);
        });
    });

    group.bench_function("python", |b| {
        bench_py(
            &input,
            r#"
import json
"#,
            r#"
s = 0
for line in INPUT.splitlines():
    value = json.loads(line)
    s += value['value']
    s += len(value['name'])
            "#,
            b,
        )
    });

    group.bench_function("borrowed", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for l in input.lines() {
                let data: DataBorrowed = serde_json::from_str(l).unwrap();
                sum += data.value as u64;
                sum += data.name.len() as u64;
            }

            black_box(sum);
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = json_group, numbers_group
);
criterion_main!(benches);
