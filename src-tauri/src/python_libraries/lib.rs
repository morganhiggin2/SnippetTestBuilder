use std::{collections::HashMap, path::Path};
use pyo3::{exceptions::{PyIOError, PyTypeError}, prelude::*};
//use crate::core_services::io_service::validate_file_location;

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetBuilder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    relative_file_location: String,
    #[pyo3(get)]
    inputs: HashMap<String, String>, 
    #[pyo3(get)]
    outputs: HashMap<String, String>
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
    fn add_input(&mut self, name: String, schema_file: String) -> PyResult<()> {
        //check if outputs already contains the output
        if self.inputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>(format!("Input {} already exists", &name))); 
        }

        //check if file exists
        if !validate_file_location(&schema_file) {
            return Err(PyErr::new::<PyIOError, _>(format!("File {} not found in adding input schema", &schema_file)));
        } 
       
        self.inputs.insert(name, schema_file);

        return Ok(());
    }

    
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
    fn add_output(&mut self, name: String, schema_file: String) -> PyResult<()> {
        //check if outputs already contains the output
        if self.outputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>(format!("Input {} already exists", &name))); 
        }

        //check if file exists
        if !validate_file_location(&schema_file) {
            return Err(PyErr::new::<PyIOError, _>(format!("File {} not found in adding input schema", &schema_file)));
        } 
       
        self.outputs.insert(name, schema_file);

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

impl PythonSnippetBuilder {
    pub fn new(relative_file_location: String) -> Self {
        return PythonSnippetBuilder {
            name: String::new(),
            relative_file_location: relative_file_location,
            outputs: HashMap::new(),
            inputs: HashMap::new()
        };
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /*pub fn get_inputs(&self) -> Vec<(String, PythonNestedSchema)> {
        //create empty vector of size inputs.len()
        /*let mut inputs: Vec<(String, String)> = Vec::with_capacity(self.inputs.len());

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
        });*/

        //let mut inputs: Vec<(String, PythonNestedSchema)> = Vec::with_capacity(self.inputs.len());

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



// important run notes
// cargo build --features pyo3/extension-module

/// Snipper module implemented in Rust
#[pymodule]
pub fn snippet_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonSnippetBuilder>()?;
    Ok(())
}

/// Check if a given file path exists
pub fn validate_file_location(relative_file_path: &str) -> bool {
    // create file path
    let file_path = Path::new(&relative_file_path);

    // check if exists
    return file_path.exists();
}