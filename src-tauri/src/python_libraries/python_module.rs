//https://pyo3.rs/main/building_and_distribution#dynamically-embedding-the-python-interpreter

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, GILPool, PyClass};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::*;
use tauri::utils::config::BuildConfig;

use crate::core_services::directory_manager::{self, DirectoryManager, SnippetDirectoryEntry, SnippetDirectorySnippet};
use crate::utils::sequential_id_generator::Uuid;

// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetInitializerBuilder {
    build_information: Vec<PythonSnippetBuildInformation>
}

// the build information for each snippet
pub struct PythonSnippetBuildInformation {
    directory_uuid: Uuid,
    name: String,
    path: PathBuf
}

// the state of the builder once the snippets have been built
pub struct FinalizedPythonSnipppetInitializerBuilder {
    built_snippets: Vec<PythonSnippetBuilderWrapper>
}

pub struct PythonSnippetBuilderWrapper {
    directory_entry_uuid: Uuid,
    python_snippet_builder: PythonSnippetBuilder,
}

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetBuilder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    inputs: Vec::<String>,
    #[pyo3(get)]
    outputs: Vec::<String>
}

impl InitializedPythonSnippetInitializerBuilder {
    /// initialize  new python builder
    pub fn new() -> Self {
        return InitializedPythonSnippetInitializerBuilder {
            build_information: Vec::<PythonSnippetBuildInformation>::new()
        };
    }
     
    /// add a new snippet information to the python builder
    pub fn add_snippet(&mut self, name: String, path: PathBuf, directory_uuid: Uuid) {
        let snippet_build_information = PythonSnippetBuildInformation::new(name, path, directory_uuid);

        self.build_information.push(snippet_build_information); 
    }

    pub fn build(self) -> Result<FinalizedPythonSnipppetInitializerBuilder, String> {
        let built_snippets = self.initialize_snippets()?;
        
        return Ok(FinalizedPythonSnipppetInitializerBuilder {
            built_snippets: built_snippets
        });
    }

    fn initialize_snippets(self) -> Result<Vec<PythonSnippetBuilderWrapper>, String> {
        let python_build_information_list = self.build_information;
        // We want to get the rust object since each python object will hold and maintain a reference to the gil and gil pool
        let python_built_snippets = Python::with_gil(|py| -> Result<Vec::<PythonSnippetBuilderWrapper>, String> {
            let mut python_snippet_builders: Vec::<PythonSnippetBuilderWrapper> = Vec::<PythonSnippetBuilderWrapper>::new();

            //TODO handle cases
            // misnames python file, how do we communicate this to the end user?
            // how do we get the python file parsing error to the end user?

            for python_build_information in python_build_information_list {
                // Create file path
                let mut main_python_file_path = python_build_information.path.into_os_string();
                main_python_file_path.push("/app.py");
                let full_path: PathBuf = main_python_file_path.into();

                // Read the main file and the main file only
                let mut file = match File::open(full_path) {
                    Ok(file) => file,
                    Err(e) => {
                        //TODO return error
                        return Err(format!("Could not open file to read python sippet: {}", e.to_string()));
                    }
                };

                // Read the contents of the file
                let mut contents = String::new();

                // Attempt to read the file contents in to the string
                match file.read_to_string(&mut contents) {
                    io::Result::Ok(_) => (),
                    io::Result::Err(e) => {
                        //TODO return error
                        return Err(format!("Could not read the contents of the main python file: {}", e.to_string())); 
                    }
                };

                // Create new gil pool
                /*let pool = unsafe { py.new_pool() };
                let py = pool.python();*/

                // import code to pool
                let fun = match PyModule::from_code_bound(
                    py,
                    &contents,
                    "",
                    "",
                ) {
                    PyResult::Ok(some) => some,
                    PyResult::Err(e) => {
                        return Err(format!("Could not create python code from main python file: {}", e.to_string()));
                    }
                };

                let fun: Py<PyAny> = match fun.getattr("init") {
                    PyResult::Ok(some) => some,
                    PyResult::Err(e) => {
                        return Err(format!("Could not get init function attribute from main python file code: {}", e.to_string()));
                    }
                }
                .into(); 

                // Create arguments for init function
                // which includes a python callable object
                let obj = Bound::new(py, PythonSnippetBuilder::new(python_build_information.name)).unwrap();
                //TODO pass it as argument with key 'snippet' in kargs
                let args = PyTuple::new_bound(py, &[obj]);

                // Define python function call closure
                let init_python_return = match fun.call1(py, args) {
                    PyResult::Ok(some) => some,
                    PyResult::Err(e) => {
                        return Err(format!("Error calling init function from main python file: {}", e.to_string()));
                    }
                };

                let init_python_return : PythonSnippetBuilder = match init_python_return.extract(py) {
                    PyResult::Ok(some) => some,
                    PyResult::Err(e) => {
                        return Err(format!("Error extacting python snippet builder result from init function call: {}", e.to_string()));
                    }
                };

                // Create wrapper contianing extra necessary information
                let python_return_wrapper = PythonSnippetBuilderWrapper::new(python_build_information.directory_uuid, init_python_return);

                python_snippet_builders.push(python_return_wrapper);
            }

            return Ok(python_snippet_builders);
        })?;

        return Ok(python_built_snippets);
    }}

impl PythonSnippetBuilderWrapper {
    pub fn new(directory_entry_uuid: Uuid, python_snippet_builder: PythonSnippetBuilder) -> Self {
        return PythonSnippetBuilderWrapper {
            directory_entry_uuid: directory_entry_uuid,
            python_snippet_builder: python_snippet_builder
        }
    }
    
    pub fn get_name(&self) -> String {
        return self.python_snippet_builder.get_name();
    }

    pub fn get_directory_entry_uuid(&self) -> Uuid {
        return self.directory_entry_uuid.to_owned();
    }
    
    pub fn get_inputs(&self) -> &Vec<String> {
        &self.python_snippet_builder.inputs
    }

    pub fn get_outputs(&self) -> &Vec<String> {
        &self.python_snippet_builder.outputs
    }
}    
impl PythonSnippetBuildInformation {
    /// create new python build information for a snippet to be built 
    pub fn new(name: String, path: PathBuf, directory_uuid: Uuid) -> Self {
        return PythonSnippetBuildInformation {
            directory_uuid: directory_uuid,
            name: name,
            path: path
        };
    }
}

impl FinalizedPythonSnipppetInitializerBuilder {
    pub fn get_build_information(&self) -> &Vec<PythonSnippetBuilderWrapper> {
        return &self.built_snippets;
    }
}

impl PythonSnippetBuilder {
    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }
}

#[pymethods]
impl PythonSnippetBuilder {
    #[new]
    fn new(name: String) -> Self {
        // placeholder for directory entry uuid as we are going to set this later
        PythonSnippetBuilder { name: name, inputs: Vec::<String>::new(), outputs: Vec::<String>::new()}
    }

    /*#[classmethod]
    fn add_property(cls: &PyType, name: &PyUnicode, property_type: &PyUnicode) -> PyResult<()>{
        //convert from pytypes to rust types
        let r_name: String = name.extract()?;
        let cl: Self = cls.extract()?;
        return Ok(());
    }*/
    /// callable method from python
    /// insert io input point to snippet
    #[pyo3(text_signature = "$self, name")]
    fn add_input(&mut self, name: String) -> PyResult<()> {
        // if inputs is already in output, raise error to python
        if self.inputs.contains(&name) {
            return Err(PyValueError::new_err(format!("Cannot insert {} into snippet {}, already exists as input", name, self.name)));
        }

        // insert input
        self.inputs.push(name);

        return Ok(());
    }
    
    /// callable method from python
    /// insert io input point to snippet
    #[pyo3(text_signature = "$self, name")]
    fn add_output(&mut self, name: String) -> PyResult<()> {
        // if inputs is already in output, raise error to python
        if self.outputs.contains(&name) {
            return Err(PyValueError::new_err(format!("Cannot insert {} into snippet {}, already exists as output", name, self.name)));
        }

        // insert input
        self.outputs.push(name);

        return Ok(());
    }

    /// callable method from python
    /// finishes the snippet creation
    /// adds snippet information to external snippet manager
    fn finish(&self) -> PyResult<()> {
        return Ok(());
    }
}

impl Default for PythonSnippetBuilder {
    fn default() -> Self {
        return PythonSnippetBuilder { 
            name: String::new(),
            inputs: Vec::<String>::new(),
            outputs: Vec::<String>::new() 
        }
    }
}


/*use pyo3::{prelude::*, PyClass};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::*;
use tauri::utils::config::BuildConfig;

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetBuilder {
    name: String,
    relative_file_location: String
}

//call the init function, from somewhere involving the external snippet manager, that creates an empty snippet creation object,
//that involves the external snippet
//or, better, just call the init() function, expecing the return value, and cast it to this class
fn call_init() {
    let py_snippet_obj = PythonSnippetBuilder {
        name: String::new(),
        relative_file_location: "".to_string()
    };
}

pub fn call_init_2() -> PyResult<()>{
    let mut a: u32 = 4;
    let _res = Python::with_gil(|py| -> PyResult<String> {
        let obj = PyCell::new(py, PythonSnippetBuilder::default()).unwrap();
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                snippet = args[0];
                snippet.name = \"hello\"
                )",
            "",
            "",
        )?
        .getattr("example")?
        .into(); 

        let args = PyTuple::new(py, &[obj]);
        let res: PythonSnippetBuilder = fun.call1(py, args)?.extract(py)?;
        //let res: PyAny = fun.call1(py, args)?.into_py(py);
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

        return Ok(res.name.clone());

        //convert pyany to the class? or just get obj since it will have been changed?

        //pyo3::py_run!(py, obj, "expr");
    })?;

    println!("{}", _res);
    return Ok(());
    //let psc: i32 = builtins.call1("init", ())?.extract()?;
}

#[pymethods]
impl PythonSnippetBuilder {
    #[new]
    fn new(name: String) -> Self {
        PythonSnippetBuilder { name: name, relative_file_location: "".to_string() }
    }

    /*#[classmethod]
    fn add_property(cls: &PyType, name: &PyUnicode, property_type: &PyUnicode) -> PyResult<()>{
        //convert from pytypes to rust types
        let r_name: String = name.extract()?;
        let cl: Self = cls.extract()?;
        return Ok(());
    }*/

    /// callable method from python
    /// add property parameter to snippet
    #[pyo3(text_signature = "$self, name, property_type")]
    fn add_property(&mut self, name: String, property_type: String) -> PyResult<()> {
        //convert from pytypes to rust types
        return Ok(());
    }
    
    /// callable method from python
    /// finishes the snippet creation
    /// adds snippet information to external snippet manager
    fn fnish(&self) -> PyResult<()> {
        return Ok(());
    }
}

pub fn call_init() -> Result<(), String> {
    /*let _res = match Python::with_gil(|py| -> PyResult<String> {
        let obj = PyCell::new(py, PythonSnippetBuilder::new("".to_string())).unwrap();
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
        let res: PythonSnippetBuilder = fun.call(py, (), Some(kwargs))?.extract(py)?;
        //let res: PyAny = fun.call1(py, args)?.into_py(py);
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

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
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

        return Ok(());
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    return Ok(());

}

impl Default for PythonSnippetBuilder {
    fn default() -> Self {
        return PythonSnippetBuilder { 
            name: String::new(), 
            relative_file_location: String::new() 
        }
    }
}

#[pymodule]
pub fn snippet_module(_py: Python, m:&PyModule) -> PyResult<()> {
    m.add_class::<PythonSnippetBuilder>()?;
    return Ok(());
}

fn register() {

}
/*#[pyclass]
struct MyClass {
    #[pyo3(get)]
    num: i32,
}
Python::with_gil(|py| {
    let obj = PyCell::new(py, MyClass { num: 3 }).unwrap();
    {
        let obj_ref = obj.borrow(); // Get PyRef
        assert_eq!(obj_ref.num, 3);
        // You cannot get PyRefMut unless all PyRefs are dropped
        assert!(obj.try_borrow_mut().is_err());
    }
    {
        let mut obj_mut = obj.borrow_mut(); // Get PyRefMut
        obj_mut.num = 5;
        // You cannot get any other refs until the PyRefMut is dropped
        assert!(obj.try_borrow().is_err());
        assert!(obj.try_borrow_mut().is_err());
    }

    // You can convert `&PyCell` to a Python object
    pyo3::py_run!(py, obj, "assert obj.num == 5");
}); */

//https://pyo3.rs/main/function

/*#[pyclass]
struct Person {
    name: String,
    age: u8,
    height_cm: f32,
}

impl pyo3::FromPyObject<'_> for Person {
    fn extract(any: &PyAny) -> PyResult<Self> {
        Ok(any.downcast().unwrap())
               ^^^^^^^^ method not found in `&pyo3::types::any::PyAny`
    }
}

#[pyfunction]
fn make_person() -> PyResult<Person> {
    Ok(Person {
        name: "Bilbo Baggins".to_string(),
        age: 51,
        height_cm: 91.44,
    })
}

#[pyfunction]
fn person_info(py:Python, p: PyObject) -> PyResult<()> {
    let p : Person = p.extract(py)?;
    println!("{} is {} years old", p.name, p.age);
    Ok(())
} */




/*# Load pyenv automatically by appending
# the following to 
~/.bash_profile if it exists, otherwise ~/.profile (for login shells)
and ~/.bashrc (for interactive shells) :

export PYENV_ROOT="$HOME/.pyenv"
command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"
eval "$(pyenv init -)"

# Restart your shell for the changes to take effect.

# Load pyenv-virtualenv automatically by adding
# the following to ~/.bashrc:

eval "$(pyenv virtualenv-init -)"
 */ */