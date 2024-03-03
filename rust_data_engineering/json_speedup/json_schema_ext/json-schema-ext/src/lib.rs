use pyo3::prelude::*;

// Rust code
use jsonschema::JSONSchema;
use pyo3::exceptions::PyIOError;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    address: Address,
    contacts: Vec<Contact>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Address {
    street: String,
    city: String,
    country: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    r#type: String,
    value: String,
}

#[pyclass]
struct FastJsonSchema {
    schema: JSONSchema,
}
#[pymethods]
impl FastJsonSchema {
    #[new]
    fn load(schema_path: &str) -> PyResult<Self> {
        let mut file = match File::open(schema_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(pyo3::exceptions::PyFileNotFoundError::new_err(format!(
                    "Failed to open schema file: {}",
                    err
                )));
            }
        };
        // Read the file contents into a string
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            return Err(pyo3::exceptions::PyIOError::new_err(format!(
                "Failed to read schema file: {}",
                err
            )));
        };

        let schema_json: JsonValue = serde_json::from_str(&contents)
            .map_err(|err| PyIOError::new_err(format!("Failed to read schema file: {}", err)))?;
        let compiled_schema = JSONSchema::compile(&schema_json)
            .map_err(|err| PyIOError::new_err(format!("Failed to compile schema file: {}", err)))?;
        Ok(FastJsonSchema {
            schema: compiled_schema,
        })
    }
    fn validate(&self, data_path: &str) -> PyResult<Vec<String>> {
        let data_file = File::open(data_path)
            .map_err(|err| PyIOError::new_err(format!("Failed to open json file: {}", err)))?;
        let reader = BufReader::new(data_file);
        let data_json: JsonValue = serde_json::from_reader(reader)
            .map_err(|err| PyIOError::new_err(format!("Failed to read json file: {}", err)))?;

        let validation_result = self.schema.validate(&data_json);
        let errors = match validation_result {
            Ok(_) => Vec::new(),
            Err(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
        };
        Ok(errors)
    }
}

#[pymodule]
fn fast_json(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FastJsonSchema>()?;

    Ok(())
}
