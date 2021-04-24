#[cfg(features = "altalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value as JsonToken};
use std::time::Instant;
use tapa_trait_serde::IJsonSerializable;
use uuid::Uuid;

#[derive(Deserialize, IJsonSerializable, Serialize, Clone)]
pub struct PersonData {
    pub id: Uuid,
    pub name: String,
    pub birthdate: DateTime<Utc>,
}

impl PersonData {
    pub fn new(
        id: Option<Uuid>,
        name: &str,
        birthdate_utc: (i32, u32, u32),
        birthtime_utc: (u32, u32, u32),
    ) -> Self {
        let id = match id {
            None => Uuid::new_v4(),
            Some(id) => id,
        };
        let birthdate = Utc
            .ymd(birthdate_utc.0, birthdate_utc.1, birthdate_utc.2)
            .and_hms(birthtime_utc.0, birthtime_utc.1, birthtime_utc.2);

        Self {
            id,
            name: name.into(),
            birthdate,
        }
    }
}

fn main() {
    let mut people_100_000 = Vec::with_capacity(100_000);

    for _ in 0..100_000 {
        people_100_000.push(PersonData::new(None, "Kresna", (1984, 2, 9), (22, 15, 0)));
    }

    let mut people_100_000_json = Vec::with_capacity(100_000);

    while let Some(person) = people_100_000.pop() {
        people_100_000_json.push(person.to_json_string_pretty());
    }

    let mut people_100_000_indices_0 = Vec::with_capacity(100_000);
    let mut people_100_000_indices_1 = Vec::with_capacity(100_000);

    // Tokenization
    let tokenization_instant = Instant::now();

    for i in 0..100_000 {
        let people_json_borrow = people_100_000_json.get(i).unwrap();
        let people_json = from_str::<JsonToken>(people_json_borrow).unwrap();
        let people_id = people_json["id"].to_string();
        people_100_000_indices_0.push(people_id);
    }

    let tokenization_duration = tokenization_instant.elapsed();
    let tokenization_duration = tokenization_duration.as_nanos();
    let tokenization_per_item = tokenization_duration / 100_000;

    println!(
        "First ID in Tokenization: {}",
        people_100_000_indices_0.get(0).unwrap()
    );
    println!("Tokenization Duration: {} ns", tokenization_duration);
    println!("Tokenization per Item: {} ns", tokenization_per_item);

    // Deserialization
    let deseralization_instant = Instant::now();

    for i in 0..100_000 {
        let people_json_borrow = people_100_000_json.get(i).unwrap();
        let people = PersonData::from_json_string(people_json_borrow).unwrap();
        people_100_000_indices_1.push(people.id);
    }

    let deserialization_duration = deseralization_instant.elapsed();
    let deserialization_duration = deserialization_duration.as_nanos();
    let deserialization_per_item = deserialization_duration / 100_000;

    println!(
        "First ID in Deserialization: {}",
        people_100_000_indices_1.get(0).unwrap()
    );
    println!("Deserialization Duration: {} ns", deserialization_duration);
    println!("Deserialization per Item: {} ns", deserialization_per_item);
}
