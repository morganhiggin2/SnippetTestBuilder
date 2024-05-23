use pyo3::Python;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use crate::state_management::external_snippet_manager::ExternalSnippetManager;
use crate::state_management::external_snippet_manager::IOContentType;
use crate::utils::sequential_id_generator::SequentialIdGenerator;
use ::snippet_python_module::{PythonSnippetBuilder, snippet_module};

/// call init python function from inside the python snippet
/// get the snippet information
/// add to external snippet manager
pub fn call_init_todo_delete_this_method(sequence_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Result<(), &'static str> {
    //initialize python enviorment
    //TODO keep gil active amoung snippet calls
    pyo3::append_to_inittab!(snippet_module);
    pyo3::prepare_freethreaded_python();

    let python_snippet_creation = match Python::with_gil(|py| -> Result<PythonSnippetBuilder, &'static str> {
        let obj = PyCell::new(py, PythonSnippetBuilder::new("".to_string())).unwrap();

        let res_1 = Python::run(py, "a = 5", None, None);
        let res_2 = Python::run(py, "a * 2", None, None);
        let tmp = match PyModule::from_code(
            py,
            "import snippet_module as spm

def init(*args, **kargs):
    snippet = kargs[\"snippet\"]
    snippet.set_name(\"very_new_name\")

    schema = spm.create_base_schema()

    return snippet;",
            "",
            "",
        ) {
            Ok(result) => result,
            Err(e) => {
                //TODO pass back to user in front end python error (e.to_string())
                println!("{}", e.to_string().as_str());
                return Err("could not convert python code to function object");
            }
        };

        let tmp = match tmp.getattr("init") {
            Ok(result) => result,
            Err(_) => {
                return Err("could not get python function 'init', from python code module object")
            }
        };

        let fun: Py<PyAny> = tmp.into();

        //Py<PyAny> 
        let kwargs = [("snippet", obj)].into_py_dict(py);

        //let res: PythonSnippetBuilder = fun.call(py, (), Some(kwargs))?.extract(py)?;
        let tmp: &PyAny = match fun.call(py, (), Some(kwargs)) {
            Ok(result) => result.into_ref(py),
            Err(_) => {
                return Err("could not call function init from python module object");
            }
        };

        let tmp: &PyCell<PythonSnippetBuilder> = match tmp.downcast() {
            Ok(result) => result,
            Err(_) => {
                return Err("snippet not returned from init function, or did so in inproper form")
            }
        };

        let tmp: PyResult<PythonSnippetBuilder> = tmp.extract(); 
        let obj = tmp.unwrap();

        //get the rust struct from python object
        //let res: PyAny = fun.call(py, (), Some(kwargs))?.into_py(py);
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

        return Ok(obj);
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
    };

    //external_snippet_manager.

    /*let _res = match Python::with_gil(|py| -> PyResult<()> {
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
    };*/

    println!("{}", python_snippet_creation.get_name());

    //return add_python_snippet_creation_to_external_snippet_manager(sequence_id_generator, external_snippet_manager, python_snippet_creation);
    return Ok(());
}

/*
pub fn add_python_snippet_creation_to_external_snippet_manager(sequence_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, python_snippet_creation: PythonSnippetBuilder) -> Result<(), &'static str> {
    //create empty snippet
    let external_snippet_uuid = external_snippet_manager.create_empty_snippet(sequence_id_generator, &python_snippet_creation.get_name());

    //initialize vector of io points
    let mut io_points: Vec<(String, IOContentType, bool)> = Vec::with_capacity(python_snippet_creation.get_num_inputs() + python_snippet_creation.get_num_outputs());

    //add inputs and outputs to io points  
    for (name, datatype) in python_snippet_creation.get_inputs().iter() {
        //convert data type string to enum
        let content_type = match datatype.as_str() {
            "XML" => IOContentType::XML,
            "JSON" => IOContentType::JSON,
            "" => IOContentType::None,
            //can safely create custom type as python module checks if it already exists as a datatype
            def => IOContentType::Custom(def.to_string()) 
        };

        io_points.push((name.clone(), content_type, true));
    }

    //add inputs and outputs to io points 
    for (name, datatype) in python_snippet_creation.get_outputs().iter() {
        //convert data type string to enum
        let content_type = match datatype.as_str() {
            "XML" => IOContentType::XML,
            "JSON" => IOContentType::JSON,
            "" => IOContentType::None,
            //can safely create custom type as python module checks if it already exists as a datatype
            def => IOContentType::Custom(def.to_string()) 
        };

        io_points.push((name.clone(), content_type, false));
    }

    //add io points to external snippet
    return external_snippet_manager.add_io_points(sequence_id_generator, external_snippet_uuid, io_points);
}*/

//calling python function
//getting pyresult back (which is pydict of return values based on outputs defined for snippet)
//converting py result
//check if matches desgniated outputs and types
//return values as py values (no need to cast to rust (maybe for checking, but try not to, as it is more overhead))

/*[lib]
name = "snippet_python_module"
version = "0.0.0"
path = "src/lib/lib.rs"
crate-type = ["cdylib"] */
//https://pyo3.rs/main/building_and_distribution#dynamically-embedding-the-python-interpreter
/*use pyo3::{prelude::*, PyClass};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::*;
use tauri::utils::config::BuildConfig;

#[pyclass]
#[derive(FromPyObject)]
struct PythonSnippetBuilder {
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

impl Default for PythonSnippetBuilder {
    fn default() -> Self {
        return PythonSnippetBuilder { 
            name: String::new(), 
            relative_file_location: String::new() 
        }
    }
}

#[pymodule]
fn snippet_module(_py: Python, m:&PyModule) -> PyResult<()> {
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
} */ */




/*
  include module in here and don't need it seperatly 
  use pyo3::prelude::*;

#[pyfunction]
fn add_one(x: i64) -> i64 {
    x + 1
}

#[pymodule]
fn foo(_py: Python<'_>, foo_module: &PyModule) -> PyResult<()> {
    foo_module.add_function(wrap_pyfunction!(add_one, foo_module)?)?;
    Ok(())
}

fn main() -> PyResult<()> {
    pyo3::append_to_inittab!(foo);
    Python::with_gil(|py| Python::run(py, "import foo; foo.add_one(6)", None, None))
}
 */


/*\
to export this for non techincal users, set LD_LIBRARY_PATH to the root folder of where to look for the libpython3.10.so
    or on windows, the PATH variable to look for python310.dll (append to these, don't reset, also look to see if these are correct before appending)
    or (as suggested), add python's lib path (or path if windows)
        linux: /opt/python__version__/lib
        windows: C:\Windows\System
to technical people, simply have then run their python virtual enviorment before starting the program, and this should be able to find it
 */



//with multiple peices: https://pyo3.rs/main/python_from_rust
pub fn call_snippets_init(sequence_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Result<(), &'static str> {
    //get snippets and locations from snipet directory manager

    //initialize python enviorment
    //TODO keep gil active amoung snippet calls
    pyo3::append_to_inittab!(snippet_module);
    pyo3::prepare_freethreaded_python();

    //start process
    let python_snippet_creation = match Python::with_gil(|py| -> Result<PythonSnippetBuilder, &'static str> {
        let obj = PyCell::new(py, PythonSnippetBuilder::new("".to_string())).unwrap();

        //use PyModule::import to import package importlib, do once for entire program
        
        //loop for each snippet
            //gather inputs, datatypes, and arguements for program in proper form in pydicts pylists etc
            //run, getting result
            //parse py result
                //pass result back to front end though window handler mutex??

        //for each snippet
            //import code from location
            //...
        
        
        let py_snippet_module = match PyModule::from_code(py, "raw_code", "file name (must be unique, enforce this by including relative location)", "")
         {
            Ok(result) => result,
            Err(e) => {
                //TODO pass back to user in front end python error (e.to_string())
                println!("{}", e.to_string().as_str());
                return Err("could not convert python code to function object");
            }
        };

        let tmp = match py_snippet_module.getattr("init") {
            Ok(result) => result,
            Err(_) => {
                return Err("could not get python function 'init', from python code module object")
            }
        };

        let fun: Py<PyAny> = tmp.into();

        //Py<PyAny> 
        let kwargs = [("snippet", obj)].into_py_dict(py);

        //let res: PythonSnippetBuilder = fun.call(py, (), Some(kwargs))?.extract(py)?;
        let tmp: &PyAny = match fun.call(py, (), Some(kwargs)) {
            Ok(result) => result.into_ref(py),
            Err(_) => {
                return Err("could not call function init from python module object");
            }
        };

        let tmp: &PyCell<PythonSnippetBuilder> = match tmp.downcast() {
            Ok(result) => result,
            Err(_) => {
                return Err("snippet not returned from init function, or did so in inproper form")
            }
        };

        let tmp: PyResult<PythonSnippetBuilder> = tmp.extract(); 
        let obj = tmp.unwrap();

        //get the rust struct from python object
        //let res: PyAny = fun.call(py, (), Some(kwargs))?.into_py(py);
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

        return Ok(obj);
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
    };

    //todo parse and return possible erros from python_snippet_creation, not as runtime errors, but as user errors to front end

    return Ok(());
}

pub fn call_snippets_run(sequence_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Result<(), &'static str> {
    //get snippets and locations from snipet directory manager

    //initialize python enviorment
    //TODO keep gil active amoung snippet calls
    pyo3::append_to_inittab!(snippet_module);
    pyo3::prepare_freethreaded_python();

    //start process
    let python_snippet_creation = match Python::with_gil(|py| -> Result<PythonSnippetBuilder, &'static str> {
        let obj = PyCell::new(py, PythonSnippetBuilder::new("".to_string())).unwrap();

        //use PyModule::import to import package importlib, do once for entire program
        
        //loop for each snippet
            //gather inputs, datatypes, and arguements for program in proper form in pydicts pylists etc
            //run, getting result
            //parse py result
                //pass result back to front end though window handler mutex??

        //for each snippet
            //import code from location
            //...
        
        
        let py_snippet_module = match PyModule::from_code(py, "raw_code", "file name (must be unique, enforce this by including relative location)", "")
         {
            Ok(result) => result,
            Err(e) => {
                //TODO pass back to user in front end python error (e.to_string())
                println!("{}", e.to_string().as_str());
                return Err("could not convert python code to function object");
            }
        };

        let tmp = match py_snippet_module.getattr("init") {
            Ok(result) => result,
            Err(_) => {
                return Err("could not get python function 'init', from python code module object")
            }
        };

        let fun: Py<PyAny> = tmp.into();

        //Py<PyAny> 
        let kwargs = [("snippet", obj)].into_py_dict(py);

        //let res: PythonSnippetBuilder = fun.call(py, (), Some(kwargs))?.extract(py)?;
        let tmp: &PyAny = match fun.call(py, (), Some(kwargs)) {
            Ok(result) => result.into_ref(py),
            Err(_) => {
                return Err("could not call function init from python module object");
            }
        };

        let tmp: &PyCell<PythonSnippetBuilder> = match tmp.downcast() {
            Ok(result) => result,
            Err(_) => {
                return Err("snippet not returned from init function, or did so in inproper form")
            }
        };

        let tmp: PyResult<PythonSnippetBuilder> = tmp.extract(); 
        let obj = tmp.unwrap();

        //get the rust struct from python object
        //let res: PyAny = fun.call(py, (), Some(kwargs))?.into_py(py);
        //let res_class: PyResult<PythonSnippetBuilder> = any.downcast().unwrap();
        //let o_res: Py<PythonSnippetBuilder> = res.extract::<PythonSnippetBuilder>(py)?;
        //let res: &PyCell<PythonSnippetBuilder> = fun.call1(py, args)?.extract()?;

        return Ok(obj);
    }) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
    };

    //todo parse and return possible erros from python_snippet_creation, not as runtime errors, but as user errors to front end

    return Ok(());
}