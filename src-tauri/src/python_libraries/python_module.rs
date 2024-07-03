//https://pyo3.rs/main/building_and_distribution#dynamically-embedding-the-python-interpreter

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

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
    
}

#[pyclass]
#[derive(FromPyObject)]
pub struct PythonSnippetBuilder {
    name: String,
    relative_file_location: String
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



        todo!();


    }

    fn initialize_snippets(&self, python_build_information_list: Vec<PythonSnippetBuildInformation>) -> Result<(), String> {
        // We want to get the rust object since each python object will hold and maintain a reference to the gil and gil pool
        let py_result = Python::with_gil(|py| -> Result<Vec::<PythonSnippetBuilder>, String> {
            let mut python_snippet_builders: Vec::<PythonSnippetBuilder> = Vec::<PythonSnippetBuilder>::new();

            //TODO handle cases
            // misnames python file, how do we communicate this to the end user?
            // how do we get the python file parsing error to the end user?

            for python_build_information in python_build_information_list {
                // Create file path
                let mut main_python_file_path = python_build_information.path.into_os_string();
                main_python_file_path.push(python_build_information.name + ".py");
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
                let pool = unsafe { py.new_pool() };
                let py = pool.python();

                // import code to pool
                let fun: Py<PyAny> = match PyModule::from_code_bound(
                    py,
                    &contents,
                    "",
                    "",
                ) {
                    PyResult::Ok(some) => some,
                    PyResult::Err(e) => {
                        return Err(format!("Could not create python code from main python file: {}", e.to_string()));
                    }
                }
                .getattr("init")?
                .into(); 

                // Create arguments for init function
                // which includes a python callable object
                let obj = Bound::new(py, PythonSnippetBuilder::default()).unwrap();
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

                //TODO include python snippet information

                python_snippet_builders.push(init_python_return);
            }

            return Ok(python_snippet_builders);
        });

        // Iterate over python snippet results
        /*
        Python::with_gil(|py| -> PyResult<()> {
        for _ in 0..10 {
            let pool = unsafe { py.new_pool() };
            let py = pool.python();
            let hello: &PyString = py.eval("\"Hello World!\"", None, None)?.extract()?;
            println!("Python says: {}", hello);
        }
        Ok(())
    })?;
        */ 

        return Ok(());
    }}

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

impl Default for PythonSnippetBuilder {
    fn default() -> Self {
        return PythonSnippetBuilder { 
            name: String::new(), 
            relative_file_location: String::new() 
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