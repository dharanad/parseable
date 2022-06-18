/*
 * Parseable Server (C) 2022 Parseable, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use actix_web::web;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::Error;

pub fn flatten_json_body(
    body: web::Json<serde_json::Value>,
    labels: Option<String>,
) -> Result<String, Error> {
    let mut collector_labels = HashMap::new();

    collector_labels.insert("labels".to_string(), labels.unwrap());

    let mut flat_value: Value = json!({});
    let new_body = merge(&body, &collector_labels);
    flatten_json::flatten(&new_body, &mut flat_value, None, true, Some("_")).unwrap();
    let flattened = serde_json::to_string(&flat_value)?;

    Ok(flattened)
}

fn merge(v: &Value, fields: &HashMap<String, String>) -> Value {
    match v {
        Value::Object(m) => {
            let mut m = m.clone();
            for (k, v) in fields {
                m.insert(k.clone(), Value::String(v.clone()));
            }

            Value::Object(m)
        }
        v => v.clone(),
    }
}

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}
