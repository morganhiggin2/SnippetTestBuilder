use std::collections::{HashMap};

use pyo3::{prelude::*, exceptions::PyTypeError};

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetCreation {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    relative_file_location: String,
    #[pyo3(get)]
    data_types: HashMap<String, String>,
    /// name, datatype
    #[pyo3(get)]
    inputs: HashMap<String, String>, 
    #[pyo3(get)]
    outputs: HashMap<String, String>
}

#[pyfunction]
fn create_snippet() -> PyResult<PythonSnippetCreation> {
    return Ok(PythonSnippetCreation::new("".to_string()));
}

#[pymethods]
impl PythonSnippetCreation {
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = name; 
        return Ok(());
    }

    /// Add datatype to the python snippet creation
    /// 
    /// # Arguments
    /// 
    /// * name - name of the data type
    /// * schema - schema of the data type
    fn add_type(&mut self, name: String, schema: String) -> PyResult<()> {
        //insert data type into self map, if it already contains it, no error will be thrown
        self.data_types.insert(name, schema);
        return Ok(());
    }

    /// add input to the snippet
    /// 
    /// # Arguments
    /// 
    /// * name - name of the input
    /// * data_type - data_type of data going into the input
    fn add_input(&mut self, name: String, data_type: String) -> PyResult<()> {
        //check if data_type is valid
        if !self.data_types.contains_key(&data_type) {
            return Err(PyErr::new::<PyTypeError, _>("Not valid datatype, consider adding with add_type() class method"));
        }

        //check if outputs already contains the output
        if self.inputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>("Error message")); 
        }

        self.inputs.insert(name, data_type);

        return Ok(());
    }

    /// add an output for the snippet
    /// 
    /// # Arguments
    /// 
    /// * name - name of the output
    /// * data_type - data_type of the data going out of the output 
    fn add_output(&mut self, name: String, data_type: String) -> PyResult<()> {
        //check if data_type is valid
        if !self.data_types.contains_key(&data_type) {
            return Err(PyErr::new::<PyTypeError, _>("Not valid datatype, consider adding with add_type() class method"));
        }

        //check if outputs already contains the output
        if self.outputs.contains_key(&name) {
            return Err(PyErr::new::<PyTypeError, _>("Error message")); 
        }

        self.outputs.insert(name, data_type);

        return Ok(());
    }
}

impl PythonSnippetCreation {
    pub fn new(relative_file_location: String) -> Self {
        return PythonSnippetCreation {
            name: String::new(),
            relative_file_location: relative_file_location,
            data_types: HashMap::new(),
            outputs: HashMap::new(),
            inputs: HashMap::new()
        };
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_inputs(&self) -> Vec<(String, String)> {
        //create empty vector of size inputs.len()
        let mut inputs: Vec<(String, String)> = Vec::with_capacity(self.inputs.len());

        self.inputs.iter().for_each(|(key, value)| {
            inputs.push((key.clone(), value.clone()));
        });

        return inputs;
    }

    pub fn get_outputs(&self) -> Vec<(String, String)> {
        //create empty vector of size inputs.len()
        let mut inputs: Vec<(String, String)> = Vec::with_capacity(self.inputs.len());

        self.inputs.iter().for_each(|(key, value)| {
            inputs.push((key.clone(), value.clone()));
        });

        return inputs;
    }

    pub fn get_num_inputs(&self) -> usize {
        return self.inputs.len();
    }

    pub fn get_num_outputs(&self) -> usize {
        return self.outputs.len();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn snippet_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_snippet, m)?)?;
    Ok(())
}

// important run notes
// cargo build --features pyo3/extension-module