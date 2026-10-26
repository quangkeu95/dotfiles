//! Offline evidence integrity tools. No network client, signing, or subprocess execution.
pub mod fixture;
pub mod reconcile;
pub mod render;
pub mod validate;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::{
    fmt,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub type Result<T> = std::result::Result<T, String>;
pub const MAX_FILE_BYTES: u64 = 32 * 1024 * 1024;

#[macro_export]
macro_rules! ensure {
    ($condition:expr, $($message:tt)+) => {
        if !$condition { return Err(format!($($message)+)); }
    };
}

pub fn string(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "Expected a string".into())
}
pub fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "Expected an array".into())
}
pub fn integer(v: &Value) -> Result<u64> {
    v.as_u64()
        .ok_or_else(|| "Expected an unsigned integer".into())
}
pub fn text(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        !s.trim().is_empty()
            && !["todo", "tbd", "placeholder", "unknown", "n/a", "..."]
                .contains(&s.trim().to_lowercase().as_str())
    })
}
pub fn address(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 42 && s.starts_with("0x") && s[2..].bytes().all(|b| b.is_ascii_hexdigit())
    })
}
pub fn hash32(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 66
            && s.starts_with("0x")
            && s[2..].bytes().all(|b| b.is_ascii_hexdigit())
            && s[2..]
                .bytes()
                .any(|b| !b.eq_ignore_ascii_case(&s.as_bytes()[2]))
    })
}
pub fn identity(v: &Value) -> Result<(u64, String)> {
    let chain = integer(&v["chain_id"])?;
    ensure!(
        chain > 0 && address(&v["address"]),
        "Malformed chain/address identity"
    );
    Ok((chain, string(&v["address"])?.to_ascii_lowercase()))
}
pub fn quantity(v: &Value) -> Result<u64> {
    let s = string(v)?;
    let raw = s.strip_prefix("0x").ok_or("Malformed RPC hex quantity")?;
    ensure!(
        !raw.is_empty()
            && (raw == "0" || !raw.starts_with('0'))
            && raw.bytes().all(|b| b.is_ascii_hexdigit()),
        "Malformed RPC hex quantity"
    );
    u64::from_str_radix(raw, 16).map_err(|_| "RPC quantity exceeds supported u64 range".into())
}
pub fn hex_bytes(v: &Value) -> Result<Vec<u8>> {
    let s = string(v)?.strip_prefix("0x").ok_or("Expected hex bytes")?;
    ensure!(
        s.len() % 2 == 0 && s.bytes().all(|b| b.is_ascii_hexdigit()),
        "Malformed hex bytes"
    );
    s.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).map_err(|_| "Invalid hex")?, 16)
                .map_err(|_| "Invalid hex".into())
        })
        .collect()
}
pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn read_bounded(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    ensure!(
        file.metadata().map_err(|e| e.to_string())?.is_file(),
        "Evidence must be a regular file"
    );
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    ensure!(
        bytes.len() as u64 <= MAX_FILE_BYTES,
        "File exceeds 32 MiB limit"
    );
    Ok(bytes)
}
pub fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("Cannot create {}: {e}", path.display()))?;
    file.write_all(bytes).map_err(|e| e.to_string())
}
pub fn pretty(v: &Value) -> Result<String> {
    serde_json::to_string_pretty(v).map_err(|e| e.to_string())
}

// Value's default deserializer accepts duplicate object keys. Explicitly reject
// them, including in nested objects, so conflicting identities cannot be hidden.
struct Strict(Value);
impl<'de> Deserialize<'de> for Strict {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct StrictVisitor;
        impl<'de> Visitor<'de> for StrictVisitor {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON without duplicate keys")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Strict, E> {
                Number::from_f64(v)
                    .map(|n| Strict(n.into()))
                    .ok_or_else(|| E::custom("Non-finite number"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_none<E: de::Error>(self) -> std::result::Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut items = Vec::new();
                while let Some(Strict(item)) = a.next_element()? {
                    items.push(item);
                }
                Ok(Strict(items.into()))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut map = Map::new();
                while let Some(key) = a.next_key::<String>()? {
                    if map.contains_key(&key) {
                        return Err(de::Error::custom(format!("Duplicate JSON key: {key}")));
                    }
                    let Strict(value) = a.next_value()?;
                    map.insert(key, value);
                }
                Ok(Strict(map.into()))
            }
        }
        d.deserialize_any(StrictVisitor)
    }
}
pub fn parse_json(bytes: &[u8]) -> Result<Value> {
    ensure!(
        bytes.len() as u64 <= MAX_FILE_BYTES,
        "JSON exceeds 32 MiB limit"
    );
    serde_json::from_slice::<Strict>(bytes)
        .map(|v| v.0)
        .map_err(|e| format!("Invalid JSON: {e}"))
}
pub fn read_json(path: &Path) -> Result<Value> {
    parse_json(&read_bounded(path)?)
}
