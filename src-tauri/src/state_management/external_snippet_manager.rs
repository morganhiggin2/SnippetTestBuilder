use std::{collections::HashMap, fmt, str::FromStr};
use bimap::BiHashMap;
use strum_macros::{EnumString, Display};

use crate::{core_components::snippet_manager::{PipelineConnectorComponent, SnippetParameterBaseStorage, SnippetParameterComponent}, core_services::directory_manager::{DirectoryManager, SnippetDirectoryEntry, SnippetDirectorySnippet, SnippetDirectoryType}, python_libraries::python_build_module::{FinalizedPythonSnipppetInitializerBuilder, InitializedPythonSnippetInitializerBuilder, PythonSnippetBuildInformation, PythonSnippetBuilderWrapper}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};

//TODO implement schema matching
pub type Schema = String;

pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>,
    external_snippets_to_directory_entries: BiHashMap<Uuid, Uuid>
}

pub struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String,
    package_path: PackagePath,
    io_points: HashMap<Uuid, ExternalSnippetIOPoint>,
    parameters: HashMap<Uuid, ExternalSnippetParameter>
}

#[derive(Debug)]
pub struct ExternalSnippetCategory {
    uuid: Uuid,
    name: String,
    // TODO determine structure for parent and child categories depending on the use cases (ex. reading all the categories, etc)
    // possibly as a tree
}

pub struct ExternalSnippetIOPoint {
    uuid: Uuid,
    name: String,
    //the type of content this point serves or receives
    schema: Schema,
    //if it is an input node
    input: bool
}

pub struct ExternalSnippetParameter {
    uuid: Uuid,
    name: String,
    p_type: ExternalSnippetParameterType
}

#[derive(Clone)]
pub struct PackagePath {
    path: String
}

// supported list of parameter types
#[derive(EnumString)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Display)]
#[derive(Clone)]
pub enum ExternalSnippetParameterType {
    SingleLineText
}

// trait to define the conversion of parameter type into base storage type
pub trait IntoStorageType {
    fn into_storage_type(external_snippet_parameter_type: &ExternalSnippetParameterType) -> SnippetParameterBaseStorage;
}

// TODO make it usable for self
impl IntoStorageType for ExternalSnippetParameterType {
    fn into_storage_type(external_snippet_parameter_type: &Self) -> SnippetParameterBaseStorage {
        match external_snippet_parameter_type {
            ExternalSnippetParameterType::SingleLineText => SnippetParameterBaseStorage::String(String::default()),
        }
    }
}

// impl way to get type from parameter type

impl Default for ExternalSnippetManager {
    fn default() -> Self {
        return ExternalSnippetManager {
            external_snippets: Vec::new(),
            external_snippets_to_directory_entries: BiHashMap::new()
        }
    }
}

impl Default for PackagePath {
    fn default() -> Self {
        return PackagePath {
            path: String::default()
        } 
    }
}

impl IntoIterator for PackagePath {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let packages: std::vec::Vec<Self::Item> = self.path.split('.').map(|s| s.to_string()).collect();

        return packages.into_iter();
    }
}

impl ExternalSnippetManager {
    /// Create the exernal snippets from the directory manager
    pub fn create_external_snippets_from_directory(&mut self, directory_manager: &DirectoryManager, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        let root_directory_entry = match directory_manager.snippet_directory.get_root_directory_entry() {
            Some(some) => some,
            None => {
                return Err("Directory Manager has not been initialized".to_string());
            }
        };
        let root_package = PackagePath::default();

        // Create python snippet builder
        let mut python_snippet_builder = InitializedPythonSnippetInitializerBuilder::new();

        self.directory_walker(root_directory_entry, &mut python_snippet_builder, sequential_id_generator, root_package)?;

        // Build python snippet builder
        let python_snippet_builder: FinalizedPythonSnipppetInitializerBuilder = python_snippet_builder.build()?;

        // create external snippets from python snippet builders
        for python_snippet_information in python_snippet_builder.get_build_information() {
            self.create_snippet_from_python_build_information(python_snippet_information, sequential_id_generator)?;
        }

        return Ok(());
    }

    /// Walk though the directory, creating snippets as we go
    fn directory_walker(&self, directory_entry: &SnippetDirectoryEntry, python_snippet_builder: &mut InitializedPythonSnippetInitializerBuilder, sequential_id_generator: &mut SequentialIdGenerator, mut package_path: PackagePath) -> Result<(), String> {
        // get name
        let name = directory_entry.get_name();
        let path = directory_entry.get_path();
        let directory_uuid = directory_entry.get_uuid();
        
        match directory_entry.get_inner_as_ref() {
            SnippetDirectoryType::Category(entry) => {
                // if category, traverse children 
                for child_entry in entry.get_children() {
                    // add child entry to package path
                    let mut child_package_path = package_path.to_owned();
                    child_package_path.add(child_entry.get_name());

                    self.directory_walker(child_entry, python_snippet_builder, sequential_id_generator, child_package_path)?;
                }
            }
            SnippetDirectoryType::Snippet(_entry) => {
                // add snippet information for python building
                python_snippet_builder.add_snippet(name, path, directory_uuid, package_path);
            }
        };

        return Ok(());
    }
    
    pub fn create_snippet_from_python_build_information(&mut self, python_build_information: &PythonSnippetBuilderWrapper, sequential_id_generator: &mut SequentialIdGenerator) -> Result<Uuid, String> {
        // Get io points from schema 

        // create external snippet
        let mut external_snippet = ExternalSnippet::empty(sequential_id_generator, &python_build_information.get_name(), python_build_information.get_package_path());

        // add io (input and output) points
        //TODO pass errors to client
        for input in python_build_information.get_inputs() {
            self.add_io_point_provided_external_snippet(sequential_id_generator, &mut external_snippet, input.to_owned(), "".to_string(), true)?;
        }

        for output in python_build_information.get_outputs() {
            self.add_io_point_provided_external_snippet(sequential_id_generator, &mut external_snippet, output.to_owned(), "".to_string(), false)?;
        }

        for parameter in python_build_information.get_parameters() {
            self.add_parameter(sequential_id_generator, &mut external_snippet, parameter.0.to_owned(), parameter.1.to_owned())?;
        }

        // get uuid of external snippet
        let uuid = external_snippet.uuid;

        // add directory entry to directory entry list
        self.external_snippets_to_directory_entries.insert(uuid.to_owned(), python_build_information.get_directory_entry_uuid());

        //add it to manager
        self.external_snippets.push(external_snippet);

        return Ok(uuid);
    }

    /*
    /// create snippet that does not input or output
    pub fn create_non_acting_snippet(&mut self, sequential_id_generator: &mut SequentialIdGenerator, name: &str) -> Uuid {
        //create external snippet
        let mut external_snippet = ExternalSnippet::empty(sequential_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //create two non-acting snippet io points, one input, one output
        for input_value in [true, false].iter() 
        {
            //create new snippet io points
            let snippet_io_point = ExternalSnippetIOPoint::new_non_acting_input(sequential_id_generator, input_value.to_owned());

            //add existing io point
            external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point);
        }

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }

    /// create a snippet that has no contents
    pub fn create_empty_snippet(&mut self, sequential_id_generator: &mut SequentialIdGenerator, name: &str) -> Uuid {
        //create external snippet
        let external_snippet = ExternalSnippet::empty(sequential_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }*/

    /// add io points, given the input and output points
    pub fn add_io_points(&mut self, sequential_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, io_points: Vec::<(String, Schema, bool)>) -> Result<(), &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("snippet uuid is not valid external snippet uuid in external snippet manager");
            }
        };

        for io_point in io_points {
            //create new point
            let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(sequential_id_generator, io_point.0, io_point.1, io_point.2);

            match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
                Some(_) => {
                    return Err("duplicate snippet io point inserted into external snippet");
                },
                None => ()
            };
        }
        
        return Ok(());
    }

    /// Add io point to snippet with uuid snippet_uuid
    /// 
    /// # Arguments
    /// 
    /// * 'snippet_uuid' - uuid of the external snippet
    /// * 'name' - name of the io point
    /// * 'schema' - binding type schema of the io point
    /// * 'input' - is input io point
    pub fn add_io_point(&mut self, sequential_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, name: String, schema: Schema, input: bool) -> Result<Uuid, &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("snippet uuid is not valid external snippet uuid in external snippet manager");
            }
        };

        //create new point
        let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(sequential_id_generator, name, schema, input);

        //add point
        let uuid = snippet_io_point.uuid;
        
        match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
            Some(_) => {
                return Err("duplicate snippet io point inserted into external snippet");
            },
            None => ()
        };

        //return good result
        return Ok(uuid);
    }
    
    /// Add io point to snippet given the exteranl snippet
    /// 
    /// # Arguments
    /// 
    /// * 'snippet_uuid' - uuid of the external snippet
    /// * 'name' - name of the io point
    /// * 'schema' - binding type schema of the io point
    /// * 'input' - is input io point
    pub fn add_io_point_provided_external_snippet(&mut self, sequential_id_generator: &mut SequentialIdGenerator, external_snippet: &mut ExternalSnippet, name: String, schema: Schema, input: bool) -> Result<Uuid, String> {
        //create new point
        let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(sequential_id_generator, name, schema, input);

        //add point
        let uuid = snippet_io_point.uuid;

        match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
            Some(_) => {
                return Err("duplicate snippet io point inserted into external snippet".to_string());
            },
            None => ()
        };

        //return good result
        return Ok(uuid);
    }

    /// Add io points which essitially acts as if it has no function
    /// # Arguments
    /// 
    /// * 'snippet_uuid' - uuid of the external snippet
    /// * 'input' - is input io point
    pub fn add_non_acting_point(&mut self, sequential_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, input: bool) -> Result<Uuid, &'static str>{
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("snippet uuid is not valid external snippet uuid in external snippet manager");
            }
        };

        //create new point
        let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(sequential_id_generator,"_".to_string(), String::new(), input);

        //add point
        let uuid = snippet_io_point.uuid;
        
        match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
            Some(_) => {
                return Err("duplicate snippet io point inserted into external snippet");
            },
            None => ()
        };

        //return good result
        return Ok(uuid);
    }

    pub fn add_parameter(&mut self, sequential_id_generator: &mut SequentialIdGenerator, external_snippet: &mut ExternalSnippet, parameter_name: String, parameter_type: String) -> Result<Uuid, &'static str> {
        // get as parameter type
        let parameter_type_proper = ExternalSnippetParameterType::from_str(&parameter_type).unwrap();

        // create external snippet parameter
        let external_snippet_parameter = ExternalSnippetParameter::new(sequential_id_generator, parameter_name, parameter_type_proper);

        // get uuid of snippet parameter
        let uuid = external_snippet_parameter.uuid;

        // add to list of parameters
        external_snippet.parameters.insert(external_snippet_parameter.uuid, external_snippet_parameter);

        return Ok(uuid);
    }

    /// find mutable reference external snippet from within the external snippet manager
    /// 
    /// # Arguments
    /// 
    /// * 'uuid' - uuid of the external snippet to find
    pub fn find_external_snippet_mut(&mut self, uuid: Uuid) -> Option<&mut ExternalSnippet> {
        return self.external_snippets.iter_mut().find(|pipe: &&mut ExternalSnippet | pipe.uuid == uuid);
    }
    
    /// find refernece to external snippet from within the external snippet manager
    /// 
    /// # Arguments
    /// 
    /// * 'uuid' - uuid of the external snippet to find
    pub fn find_external_snippet(&self, uuid: Uuid) -> Option<&ExternalSnippet> {
        return self.external_snippets.iter().find(|pipe: && ExternalSnippet | pipe.uuid == uuid);
    }

    pub fn find_external_snippet_from_directory_uuid(&self, uuid: Uuid) -> Option<&ExternalSnippet> {
        let external_snippet_uuid = self.external_snippets_to_directory_entries.get_by_right(&uuid)?.to_owned();

        return self.find_external_snippet(external_snippet_uuid);
    }

    pub fn find_directory_uuid_from_external_snippet(&self, uuid: Uuid) -> Option<Uuid> {
        return self.external_snippets_to_directory_entries.get_by_left(&uuid).copied();
    } 

    pub fn find_external_snippet_mut_from_directory_uuid(&mut self, uuid: Uuid) -> Option<&mut ExternalSnippet> {
        let external_snippet_uuid = self.external_snippets_to_directory_entries.get_by_right(&uuid)?.to_owned();

        return self.find_external_snippet_mut(external_snippet_uuid);
    }

    /*
    /// Create root category node
    /// serves the purpose of being the root of all category nodes, has no children by definition
    pub fn new_root_external_category_snippet(sequential_id_generator: &mut SequentialIdGenerator, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: sequential_id_generator.get_id(),
            name: name
        };

        //TODO have each snippet have entry in python service holding python info
    }
   
    /// Create child category node, has parent by definition
    pub fn new_child_external_category_snippet(sequential_id_generator: &mut SequentialIdGenerator, parent_uuid: Uuid, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: sequential_id_generator.get_id(),
            name: name
        };
    }*/
}

impl ExternalSnippet {
    //TODO new with name, inputs, outputs ready to go
    fn empty(sequential_id_generator: &mut SequentialIdGenerator, name: &str, package_path: PackagePath) -> Self {
        //create uuid for external snippet
        let uuid = sequential_id_generator.get_id();

        //external snippet creation
        let external_snippet = ExternalSnippet {
            uuid: uuid,
            name: name.clone().to_owned(),
            package_path: package_path, 
            sub_directory: String::new(),
            io_points: HashMap::with_capacity(2),
            parameters: HashMap::new()
        };

        return external_snippet;
    }

    /// find io point given uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the io point in question
    fn find_io_point(&mut self, uuid: Uuid) -> Result<&mut ExternalSnippetIOPoint, &str> {
        match self.io_points.get_mut(&uuid) {
            Some(result) => return Ok(result),
            None => return Err("snippet io point could not be found with uuid {uuid}")
        };
    }

    fn find_parameter(&mut self, uuid: Uuid) -> Result<&mut ExternalSnippetParameter, &str> {
        match self.parameters.get_mut(&uuid) {
            Some(result) => return Ok(result),
            None => return Err("snippet parameter could not be found with uuid {uuid}")
        };
    }

    //getter and setter methods
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_package_path(&self) -> PackagePath {
        return self.package_path.to_owned();
    }
    
    /// get the io points as pipeline connectors
    /// for the snippet manager
    pub fn create_pipeline_connectors_for_io_points(&self, sequential_id_generator: &mut SequentialIdGenerator) -> Vec<PipelineConnectorComponent> {
        let mut pipeline_connectors = Vec::with_capacity(self.io_points.len());

        for io_point_pair in &self.io_points {
            pipeline_connectors.push(
                PipelineConnectorComponent::new(sequential_id_generator, io_point_pair.0.clone(), &io_point_pair.1.name,  io_point_pair.1.input.clone())
            )
        }

        return pipeline_connectors;
    }

    /// get parameters as parameter types that can hold values
    pub fn create_parameter_components_for_parameters(&self, sequential_id_generator: &mut SequentialIdGenerator) -> Vec<SnippetParameterComponent> {
        let mut parameter_components = Vec::<SnippetParameterComponent>::new();

        for parameter in &self.parameters {
            // get paramter type as string
            let p_type = parameter.1.p_type.clone();
            // get parameter type as new parameter storeage
            let parameter_storage = ExternalSnippetParameterType::into_storage_type(&parameter.1.p_type);

            // create parameter component
            let parameter_component = SnippetParameterComponent::new(parameter_storage, parameter.1.name.to_owned(), p_type, sequential_id_generator);

            parameter_components.push(parameter_component);
        }

        return parameter_components;
    }
}

impl ExternalSnippetIOPoint {
    /// create non action io endpoint
    /// useful for connecting snippets together that share no data
    pub fn new_non_acting_input(sequential_id_generator: &mut SequentialIdGenerator, input: bool) -> Self {
        let snippet_io_point = ExternalSnippetIOPoint {
            uuid: sequential_id_generator.get_id(),
            name: String::from('_'),
            schema: Schema::new(),
            input: input
        };

        return snippet_io_point;
    }

    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, name: String, schema: Schema, input: bool) -> Self {
        let snippet_io_point = ExternalSnippetIOPoint {
            uuid: sequential_id_generator.get_id(),
            name: name,
            schema: schema,
            input: input
        };

        return snippet_io_point;
    }
}

impl ExternalSnippetParameter {
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, name: String, p_type: ExternalSnippetParameterType) -> Self {
        let snippet_parameter = ExternalSnippetParameter {
            uuid: sequential_id_generator.get_id(),
            name: name,
            p_type: p_type
        };

        return snippet_parameter
    }
}

impl PackagePath {
    /// Add package to package path
    /// 
    /// # Arguments
    /// * 'package' - package to add
    pub fn add(&mut self, package: String) {
        if self.path.len() == 0 {
            self.path = package;
        }
        else {
            self.path.push_str(&".".to_string());
            self.path.push_str(&package);
        }
    }
}

impl ToString for PackagePath {
    fn to_string(&self) -> String {
        return self.path.to_owned();
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{core_services::directory_manager::{self, DirectoryManager}, state_management::external_snippet_manager::{ExternalSnippetCategory, ExternalSnippetIOPoint, ExternalSnippetParameter, ExternalSnippetParameterType}, utils::sequential_id_generator::{self, SequentialIdGenerator}};

    use super::{ExternalSnippet, ExternalSnippetManager, PackagePath};

    #[test]
    /// Testing creating the external snippet manager from the directory manager.
    /// This will invoke the create directory manager and the python module to run the init() of each snippet as well
    /// This uses the sample directory 
    fn test_external_snippet_manager_from_directory_manager() {
        //TODO enforce minimum python version
        //TODO same snippet names in different category paths

        let mut sequential_id_generator = SequentialIdGenerator::default(); 

        // create and initialize the directory manager
        let mut directory_manager = DirectoryManager::default();
        directory_manager.initialize(&"tests/testing_files/sample_directory/data/snippets/root".to_string(), &mut sequential_id_generator).unwrap();

        // create and initalize the external snippet manager
        let mut external_snippet_manager = ExternalSnippetManager::default();
        external_snippet_manager.create_external_snippets_from_directory(&directory_manager, &mut sequential_id_generator).unwrap();

        assert_eq!(external_snippet_manager.external_snippets.len(), 6);

        // test for external snippet manager state
        let snippet_map: HashMap<String, &ExternalSnippet> = external_snippet_manager.external_snippets.iter().map(|element| -> (String, &ExternalSnippet) {
            return (element.name.to_owned(), element);
        })
        .collect();

        {
            let external_snippet = match snippet_map.get("basic_one_snippet") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.basic_one_snippet".to_string());

            // lookup io points in external snippet manager
            

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("numbers".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("numbers".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
            // check if io point uuid exists in external snippet manager io point to snippet uuid map, with correct external snippet uuid
        }

        {
            let external_snippet = match snippet_map.get("add") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.math.add".to_string());

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("a".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("b".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("c".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
        }

        {
            let external_snippet = match snippet_map.get("subtract") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.math.subtract".to_string());

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("a".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("b".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("c".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
        }

        {
            let external_snippet = match snippet_map.get("mul") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.math.mul".to_string());

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("a".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("b".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("c".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
        }

        {
            let external_snippet = match snippet_map.get("remove_index_in_str") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.string_operations.remove_index_in_str".to_string());

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("index".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("str".to_string(), true)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("new_str".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());

            // search for each one
            let io_point = match io_map.get(&("original_str".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
        }
        
        {
            let external_snippet = match snippet_map.get("str_param") {
                Some(snippet) => snippet,
                None => {
                    assert!(false);

                    return
                },
            };

            // test package path
            assert_eq!(external_snippet.package_path.to_string(), "main.params.str_param".to_string());

            // create map for io points based on name and input, output
            let io_map: HashMap<(String, bool), &ExternalSnippetIOPoint> = external_snippet.io_points.values().map(|element| -> ((String, bool), &ExternalSnippetIOPoint) {
                return ((element.name.to_owned(), element.input.to_owned()), element);
            })
            .collect();

            // search for each one
            let io_point = match io_map.get(&("str".to_string(), false)) {
                Some(io_point) => io_point,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(io_point.schema, "".to_string());
            
            // create map for params based on name and input, output
            let param_map: HashMap<String, &ExternalSnippetParameter> = external_snippet.parameters.iter().map(|element| -> ((String), &ExternalSnippetParameter) {
                return (element.1.name.to_owned(), &element.1);
            })
            .collect();

            // search for each one
            let param = match param_map.get(&"str_input".to_string()) {
                Some(param) => param,
                None => {
                    assert!(false);

                    return;
                },
            };

            assert_eq!(param.p_type, ExternalSnippetParameterType::SingleLineText);
        
        //TODO tests for parameters
        //TODO map them to correct directory entries, so look to see if we can get them, and if that directory entry has the right name
        }
    }

    // Test package path iterator
    #[test]
    fn test_package_path_iterator() {
        // create package path iterator with sample sub1.sub2.sub3.child

        let mut package_path = PackagePath::default();

        package_path.add("sub1".to_string());
        package_path.add("sub2".to_string());
        package_path.add("sub3".to_string());
        package_path.add("child".to_string());

        let mut package_path_iter = package_path.into_iter(); 

        assert_eq!(package_path_iter.next().unwrap(), "sub1".to_string());
        assert_eq!(package_path_iter.next().unwrap(), "sub2".to_string());
        assert_eq!(package_path_iter.next().unwrap(), "sub3".to_string());
        assert_eq!(package_path_iter.next().unwrap(), "child".to_string());
    }
}

/*
impl ExternalSnippetCategory {
    /// Create new category node
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: sequential_id_generator.get_id(),
            name: name
        };
    }
    
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl fmt::Display for ExternalSnippetCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
} */