use std::collections::{HashMap};
use serde_json;

use pyo3::{prelude::*, exceptions::PyTypeError};

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetBuilder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    relative_file_location: String,
    //#[pyo3(get)]
    //inputs: HashMap<String, PythonSchemaBuilder>, 
    //#[pyo3(get)]
    //outputs: HashMap<String, PythonSchemaBuilder>
}

/*
#[pyfunction]
fn create_snippet() -> PyResult<PythonSnippetBuilder> {
    return Ok(PythonSnippetBuilder::new("".to_string()));
}*/

#[pymethods]
impl PythonSnippetBuilder {
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = name; 
        return Ok(());
    }

    //TODO change schema params from string to schema type (or the id returned)

    //TODO input will be the name of the input, and the schema
    //TODO same thing with the output, these are created dynamically

    /*
    /// add input to the snippet
    /// 
    /// # Arguments
    /// 
    /// * name - name of the input
    /// * data_type - json string which is the schema of the output being returned
    /// 
    /// # Schema Schema
    /// types are denoted with a name a value, as an example:
    /// {
    ///     "input_one": "int" 
    /// }
    /// 
    /// values can themselfs be a nesting of other sub schemas, and follow any format:
    /// {
    ///     "input_one": {
    ///         "sub_input_one": "int" 
    ///     } 
    ///     "input_two": "str"
    /// }
    /// 
    /// the string json values are themselfs the name of the primitive type, which includes
    /// * str
    /// * bytes
    /// * bool
    /// * int 
    /// * float
    /// 
    /// additional types supported:
    /// * tuple[T: other primitive types, U: other primitive types]
    /// * set[T: other primitive types]
    /// * list[T: other primitive types] 
    /// 
    fn add_input(&mut self, name: String, schema: String) -> PyResult<()> {
        //check if outputs already contains the output
        if self.inputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>(format!("Input {} already exists", &name))); 
        }
        
        //parse schema string as loose json
        let parsed_json_schema: serde_json::Value = match serde_json::from_str(&schema) {
            Ok(some) => some,
            Err(e) => {
                return Err(PyErr::new::<PyTypeError, _>(format!("Schema is imparsable: {}", e)));
            }
        };

        //TODO validate types in schema are valid
        self.inputs.insert(name, parsed_json_schema);

        return Ok(());
    }*/

    /*
    /// add an output for the snippet
    /// 
    /// # Arguments
    /// 
    /// * name - name of the output
    /// * schema - json string which is the schema of the output being returned 
    /// 
    /// # Schema Schema
    /// types are denoted with a name a value, as an example:
    /// {
    ///     "input_one": "int" 
    /// }
    /// 
    /// values can themselfs be a nesting of other sub schemas, and follow any format:
    /// {
    ///     "input_one": {
    ///         "sub_input_one": "int" 
    ///     } 
    ///     "input_two": "str"
    /// }
    /// 
    /// the string json values are themselfs the name of the primitive type, which includes
    /// * str
    /// * bytes
    /// * bool
    /// * int 
    /// * float
    /// 
    /// additional types supported:
    /// * tuple[T: other primitive types, U: other primitive types]
    /// * set[T: other primitive types]
    /// * list[T: other primitive types] 
    /// 
    fn add_output(&mut self, name: String, schema: String) -> PyResult<()> {
        //check if outputs already contains the output
        if self.outputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>(format!("Input {} already exists", &name))); 
        }
        
        //parse schema string as loose json
        let parsed_json_schema: serde_json::Value = match serde_json::from_str(&schema) {
            Ok(some) => some,
            Err(e) => {
                return Err(PyErr::new::<PyTypeError, _>(format!("Schema is imparsable: {}", e)));
            }
        };

        self.outputs.insert(name, parsed_json_schema);

        return Ok(());

    }*/
}

//empty schema
//const EMPTY_SCHEMA_JSON_STRING: &str = "{}";

impl PythonSnippetBuilder {
    pub fn new(relative_file_location: String) -> Self {
        return PythonSnippetBuilder {
            name: String::new(),
            relative_file_location: relative_file_location
            //outputs: HashMap::new(),
            //inputs: HashMap::new()
        };
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /*pub fn get_inputs(&self) -> Vec<(String, String)> {
        //create empty vector of size inputs.len()
        let mut inputs: Vec<(String, String)> = Vec::with_capacity(self.inputs.len());

        self.inputs.iter().for_each(|(name, schema)| {
            //convert serde json to string
            let json_string_schema = match serde_json::to_string(schema) {
                Ok(some) => some,
                Err(e) => 
                    //TODO log error
                    //Could not convert json string into json, this should work unless
                    //there is an error in the library, or the string got corrupted
                    EMPTY_SCHEMA_JSON_STRING.to_owned()
            };
            inputs.push((name.clone(), json_string_schema));
        });

        return inputs;
    }

    pub fn get_outputs(&self) -> Vec<(String, String)> {
        //create empty vector of size inputs.len()
        let mut outputs: Vec<(String, String)> = Vec::with_capacity(self.inputs.len());

        self.outputs.iter().for_each(|(name, schema)| {
            //convert serde json to string
            let json_string_schema = match serde_json::to_string(schema) {
                Ok(some) => some,
                Err(e) => 
                    //TODO log error
                    //Could not convert json string into json, this should work unless
                    //there is an error in the library, or the string got corrupted
                    EMPTY_SCHEMA_JSON_STRING.to_owned()
            };
            outputs.push((name.clone(), json_string_schema));
        });

        return outputs;
    }

    pub fn get_num_inputs(&self) -> usize {
        return self.inputs.len();
    }

    pub fn get_num_outputs(&self) -> usize {
        return self.outputs.len();
    }*/
}



// Snipper builder

/*
#[pyclass]
#[derive(FromPyObject)]
struct PythonSchemaBuilder {
    schema_fields: HashMap<String, SchemaField>
}*/

enum SchemaFieldType {
    Bool,
    NestedSchema
}

/*
    Str,
    Bytes,
    Bool,
    Float,
    Int,
    Tuple,
    Set,
    List, */

#[pyclass]
#[derive(FromPyObject)]

struct PythonFieldValue {
    // The field type, maps to the SchemaFieldType enum
    field_type: String, 
    // nested field schema, only if NestedSchema type
    nested_schema: Option<PythonNestedSchema> 
}

#[pyclass]
#[derive(FromPyObject)]
struct PythonNestedSchema {
    nested_fields: HashMap<String, PythonFieldValue>
}

#[pymethods]
impl PythonNestedSchema {
    fn add_bool_field(&mut self, field_name: String) -> PyResult<()> {
        // Create field value for boolean type
        let field_value = PythonFieldValue {
            field_type: "Bool".to_string(),
            nested_schema: None
        };

        // Add as field to nested field
        self.nested_fields.insert(field_name, field_value);

        return Ok(());
    }

    fn add_nested_schema(&mut self, field_name: String, nested_schema: PythonNestedSchema) -> PyResult<()> {
        // Create field value for nested schema type
        let field_value = PythonFieldValue {
            field_type: "NestedSchema".to_string(),
            nested_schema: Some(nested_schema) 
        };

        // Add as field to nested field
        self.nested_fields.insert(field_name, field_value);

        return Ok(());
    }
}

/// Create the base schema, which can be build upon
#[pyfunction]
pub fn create_base_schema() -> PyResult<PythonNestedSchema> {
    let nested_schema = PythonNestedSchema {
        nested_fields: HashMap::new()
    };

    return Ok(nested_schema);
}

// important run notes
// cargo build --features pyo3/extension-module

/// Snipper module implemented in Rust
#[pymodule]
pub fn snippet_module(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(create_snippet, m)?)?;
    //m.add_class::<PythonSchemaBuilder>()?;
    m.add_function(wrap_pyfunction!(create_base_schema, m)?)?;
    Ok(())
}