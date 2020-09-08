use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_cbor::{self, Value, Deserializer, error};
use std::{collections::BTreeMap, iter::FromIterator};

fn lazy(c: &mut Criterion) {
    let v = test_value();
    c.bench_function("lazy", |b| b.iter(|| test_object(&v, true)));
}

fn not_lazy(c: &mut Criterion) {
    let v = test_value();
    c.bench_function("not lazy", |b| b.iter(|| test_object(&v, false)));
}

criterion_group!(benches, lazy, not_lazy);
criterion_main!(benches);

fn test_value() -> Vec<u8> {
    let mut object = BTreeMap::new();
    object.insert(Value::Text("a".to_owned()), Value::Text("A".to_owned()));
    object = BTreeMap::from_iter(vec![(Value::Text("a".to_owned()), Value::Map(object))]);
    object.insert(Value::Text("b".to_owned()), Value::Text("B".to_owned()));
    object.insert(Value::Text("c".to_owned()), Value::Text("C".to_owned()));
    object.insert(Value::Text("d".to_owned()), Value::Text("D".to_owned()));
    object.insert(Value::Text("e".to_owned()), Value::Text("E".to_owned()));

    serde_cbor::to_vec(&object).unwrap()
}

fn test_object(v: &[u8], lazy: bool) {

    let mut de = Deserializer::from_slice(&v);
    if lazy {
        de.whitelist = vec!["b"];
    }
    let value: error::Result<Value> = serde::de::Deserialize::deserialize(&mut de);

    match value.unwrap() {
        Value::Map(object) => {
            assert_eq!(object.len(), if lazy {
                1
            } else {
                5
            });
        }
        _ => unreachable!()
    }
}