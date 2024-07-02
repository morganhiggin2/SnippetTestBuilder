//https://pyo3.rs/main/building_and_distribution#dynamically-embedding-the-python-interpreter

use pyo3::{prelude::*, PyClass};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::*;
use tauri::utils::config::BuildConfig;

use crate::core_services::directory_manager::{self, DirectoryManager, SnippetDirectoryEntry, SnippetDirectorySnippet};

pub fn initialize_snippets(directory_snippets: Vec<&SnippetDirectoryEntry>) {
    // We want to get the rust object since each python object will hold and maintain a reference to the gil and gil pool
    let py_result = Python::with_gil(|py| -> PyResult<()> {
        for directory_snippet in directory_snippets {
            let mut path = directory_snippet.get_path();
            let name = directory_snippet.get_name();

            let directory_snippet_snippet = directory_snippet.get_as_snippet();
            
            // Read the main file and the main file only
            let main_file_path = path.extend(name + ".py");

            //.Open main file
            // Read main file
            // Close main file 

            // Create new gil pool
            // Run main file, get result
            // Return result as rust type only
        }

        return PyResult::Ok(());
    });

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