use std::collections::{HashMap, HashSet};

use pyo3::{prelude::*, PyClass, exceptions::PyTypeError};
use tauri::utils::assets::phf::Set;

#[pyclass]
//#[derive(FromPyObject)]
struct PythonSnippetCreation {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    relative_file_location: String,
    data_types: HashSet<String>,
    /// name, datatype
    properties: HashMap<String, String> 
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
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

    //add type
    //could be json type enforced by schema, not sure yet
    fn add_type(&mut self, name: String) -> PyResult<()> {
        //insert data type into self map, if it already contains it, no error will be thrown
        self.data_types.insert(name);
        return Ok(());
    }

    fn add_property(&mut self, name: String, data_type: String) -> PyResult<()> {
        let exists = self.properties.contains_key(&name);

        if exists {
            return Err(PyErr::new::<PyTypeError, _>("Error message")); 
        }

        return Ok(());
    }
}

pub fn call_init() -> Result<(), String> {
    /*let _res = match Python::with_gil(|py| -> PyResult<String> {
        let obj = PyCell::new(py, PythonSnippetCreation::new("".to_string())).unwrap();
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "
            import snippet_python_module as spm;

            def init(*args, **kargs):
                snippet = kargs[\"snippet\"]
                snippet.set_name(\"new-name\")

                return snippet;
            
            ",
            "",
            "",
        )?
        .getattr("init")?
        .into(); 

        let kwargs = [("snippet", obj)].into_py_dict(py);
        let res: PythonSnippetCreation = fun.call(py, (), Some(kwargs))?.extract(py)?;
        //let res: PyAny = fun.call1(py, args)?.into_py(py);
        //let res_class: PyResult<PythonSnippetCreation> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetCreation> = res.extract::<PythonSnippetCreation>(py)?;
        //let res: &PyCell<PythonSnippetCreation> = fun.call1(py, args)?.extract()?;

        return Ok(res.name.clone());
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    println!("{}", _res);*/
    let _res = match Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "
            def say_hello(*args, **kwargs): 
                print(\"hello\")
            ",
            "",
            "",
        )?
        .getattr("hello")?
        .into(); 

        fun.call0(py)?;
        //let res: PyAny = fun.call1(py, args)?.into_py(py);
        //let res_class: PyResult<PythonSnippetCreation> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetCreation> = res.extract::<PythonSnippetCreation>(py)?;
        //let res: &PyCell<PythonSnippetCreation> = fun.call1(py, args)?.extract()?;

        return Ok(());
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    return Ok(());

}

impl PythonSnippetCreation {
    fn new(relative_file_location: String) -> Self {
        return PythonSnippetCreation {
            name: String::new(),
            relative_file_location: relative_file_location,
            data_types: HashSet::new(),
            properties: HashMap::new()
        };
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn snippet_python_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_snippet, m)?)?;
    Ok(())
}

// important run notes
// cargo build --features pyo3/extension-module