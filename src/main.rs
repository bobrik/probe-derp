use std::collections::HashMap;

use probe::probe_lazy;

macro_rules! probe_key {
    ($name:ident, $key:ident) => {
        let key_cstr; // https://github.com/cuviper/probe-rs/issues/10
        probe_lazy!(cache, $name, {
            key_cstr = std::ffi::CString::new(format!("{:?}", $key)).unwrap();
            key_cstr.as_ptr()
        })
    };
}

#[tokio::main]
async fn main() {
    let mut cache_one = Cache::default();

    let key = Key::new("beep".to_string());

    let value = cache_one.get(&key).await;
    eprintln!("first value (miss) : {value}");

    // This does not produce a probe with a hit
    let value = cache_one.try_get(&key).unwrap();
    eprintln!("first value (hit)  : {value}");
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    inner: String,
}

impl Key {
    fn new(inner: String) -> Self {
        Self { inner }
    }
}

#[derive(Default)]
struct Cache {
    inner: HashMap<Key, String>,
}

impl Cache {
    fn try_get(&mut self, key: &Key) -> Option<String> {
        self.inner.get(key).map(|value| {
            probe_key!(hit, key);

            value.clone()
        })
    }

    async fn get(&mut self, key: &Key) -> String {
        if let Some(value) = self.inner.get(key) {
            probe_key!(hit, key);

            return value.clone();
        }

        probe_key!(miss, key);

        let value = format!("whoa: {key:?}");

        self.inner.insert(key.clone(), value.clone());

        value
    }
}
