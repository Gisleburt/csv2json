use std::collections::HashMap;
use std::collections::hash_map::Entry;
use serde_json::Value as JsonValue;
use serde_json::map::Entry as JsonEntry;

pub fn row_to_object(
    headers: &Vec<String>,
    row: Vec<String>,
    ds: Option<&str>,
) -> HashMap<String, JsonValue> {
    let mut items = HashMap::new();
    let data_iter = headers.iter().cloned().zip(row.iter().cloned());
    for (key, value) in data_iter {
        let (key, value) = dimensional_converter(key, value, &ds);
        let prepared_value = prepare_upsert(items.entry(key.clone()), value);
        items.insert(key, prepared_value);
    }
    items
}

fn dimensional_converter(key: String, value: String, ds: &Option<&str>) -> (String, JsonValue) {
    if let &Some(separator) = ds {
        if key.contains(separator) {
            let mut parts = key.split(separator);
            let this_key = parts.next().unwrap().to_owned();
            let next_key = parts.collect::<Vec<&str>>().join(".").to_owned();
            let (_, data) = dimensional_converter(next_key.clone(), value, &Some(separator));
            return (this_key, json!({ next_key: data }));
        }
    }
    (key, json!(value))
}

fn prepare_upsert(entry: Entry<String, JsonValue>, data: JsonValue) -> JsonValue {
    match entry {
        Entry::Vacant(_) => data,
        Entry::Occupied(e) => {
            let old_value = e.remove();
            merge_values(old_value, data)
        }
    }
}

fn merge_values(v1: JsonValue, v2: JsonValue) -> JsonValue {
    // If both values are objects combine on keys
    if v1.is_object() && v2.is_object() {
        if let JsonValue::Object(mut o1) = v1 {
            if let JsonValue::Object(mut o2) = v2 {
                o2.into_iter().for_each(|(key2, value2)| {
                    let replacement = match o1.entry(key2.to_owned()) {
                        JsonEntry::Vacant(_) => value2,
                        JsonEntry::Occupied(e) => {
                            let value1 = e.remove();
                            merge_values(value1, value2)
                        }
                    };
                    o1.insert(key2, replacement);
                });
                return json!(o1);
            }
            panic!("This isn't possible");
        }
    }

    // If both values are arrays, add the other to it.
    if v1.is_array() && v2.is_array() {
        if let JsonValue::Array(mut a1) = v1 {
            if let JsonValue::Array(mut a2) = v2 {
                a1.append(&mut a2);
                return json!(a1);
            }
            panic!("This isn't possible");
        }
    }

    // If either is an array add the other to it.
    if let JsonValue::Array(mut a1) = v1 {
        a1.push(v2);
        return json!(a1);
    }
    if let JsonValue::Array(mut a2) = v2 {
        a2.push(v1);
        return json!(a2);
    }

    // Otherwise create a new array with both items
    json!([v1, v2])
}
