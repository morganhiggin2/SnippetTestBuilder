use std::{collections::HashMap, fmt};
use crate::{utils::sequential_id_generator::{Uuid, SequentialIdGenerator}, core_components::snippet_manager::PipelineConnectorComponent};

//TODO implement schema matching
pub type Schema = String;

pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>
}

pub struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String,
    io_points: HashMap<Uuid, ExternalSnippetIOPoint>
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

impl Default for ExternalSnippetManager {
    fn default() -> Self {
        return ExternalSnippetManager { 
            external_snippets: Vec::with_capacity(24)
        };
    }
}

impl ExternalSnippetManager {
    /// create snippet that does not input or output
    pub fn create_non_acting_snippet(&mut self, seq_id_generator: &mut SequentialIdGenerator, name: &str) -> Uuid {
        //create external snippet
        let mut external_snippet = ExternalSnippet::empty(seq_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //create two non-acting snippet io points, one input, one output
        for input_value in [true, false].iter() 
        {
            //create new snippet io points
            let snippet_io_point = ExternalSnippetIOPoint::new_non_acting_input(seq_id_generator, input_value.to_owned());

            //add existing io point
            external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point);
        }

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }

    /// create a snippet that has no contents
    pub fn create_empty_snippet(&mut self, seq_id_generator: &mut SequentialIdGenerator, name: &str) -> Uuid {
        //create external snippet
        let external_snippet = ExternalSnippet::empty(seq_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }

    /// add io points, given the input and output points
    pub fn add_io_points(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, io_points: Vec::<(String, Schema, bool)>) -> Result<(), &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        for io_point in io_points {
            //create new point
            let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(seq_id_generator, io_point.0, io_point.1, io_point.2);

            //add point
            let uuid = snippet_io_point.uuid;

            match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
                Some(_) => (),
                None => {
                    return Err("duplicate snippet io point inserted into external snippet");
                }
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
    pub fn add_io_point(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, name: String, schema: Schema, input: bool) -> Result<Uuid, &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(seq_id_generator, name, schema, input);

        //add point
        let uuid = snippet_io_point.uuid;
        
        match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
            Some(_) => (),
            None => {
                return Err("duplicate snippet io point inserted into external snippet");
            }
        };

        //return good result
        return Ok(uuid);
    }

    /// Add io points which essitially acts as if it has no function
    /// # Arguments
    /// 
    /// * 'snippet_uuid' - uuid of the external snippet
    /// * 'input' - is input io point
    pub fn add_non_acting_point(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, input: bool) -> Result<Uuid, &'static str>{
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: ExternalSnippetIOPoint = ExternalSnippetIOPoint::new(seq_id_generator,"_".to_string(), String::new(), input);

        //add point
        let uuid = snippet_io_point.uuid;
        
        match external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point) {
            Some(_) => (),
            None => {
                return Err("duplicate snippet io point inserted into external snippet");
            }
        };

        //return good result
        return Ok(uuid);
    }

    /// find mutable reference external snippet from within the external snippet manager
    /// 
    /// # Arguments
    /// 
    /// * 'uuid' - uuid of the external snippet to find
    pub fn find_external_snippet_mut(&mut self, uuid: Uuid) -> Result<&mut ExternalSnippet, &'static str> {
        match self.external_snippets.iter_mut().find(|pipe: &&mut ExternalSnippet | pipe.uuid == uuid) {
            Some(result) => return Ok(result),
            None => return Err("external snippet could not be found with uuid")
        };
    }
    
    /// find refernece to external snippet from within the external snippet manager
    /// 
    /// # Arguments
    /// 
    /// * 'uuid' - uuid of the external snippet to find
    pub fn find_external_snippet(&self, uuid: Uuid) -> Result<&ExternalSnippet, &'static str> {
        match self.external_snippets.iter().find(|pipe: && ExternalSnippet | pipe.uuid == uuid) {
            Some(result) => return Ok(result),
            None => return Err("external snippet could not be found with uuid")
        };
    }

    /*
    /// Create root category node
    /// serves the purpose of being the root of all category nodes, has no children by definition
    pub fn new_root_external_category_snippet(seq_id_generator: &mut SequentialIdGenerator, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name
        };

        //TODO have each snippet have entry in python service holding python info
    }
   
    /// Create child category node, has parent by definition
    pub fn new_child_external_category_snippet(seq_id_generator: &mut SequentialIdGenerator, parent_uuid: Uuid, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name
        };
    }*/
}

impl ExternalSnippet {
    //TODO new with name, inputs, outputs ready to go
    fn empty(seq_id_generator: &mut SequentialIdGenerator, name: &str) -> Self {
        //create uuid for external snippet
        let uuid = seq_id_generator.get_id();

        //external snippet creation
        let external_snippet = ExternalSnippet {
            uuid: uuid,
            name: name.clone().to_owned(),
            sub_directory: String::new(),
            io_points: HashMap::with_capacity(2)
        };

        return external_snippet;
    }

    /// find io point given uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the io point in question
    fn find_io_point(&mut self, external_snippet: &mut ExternalSnippet, uuid: Uuid) -> Result<&mut ExternalSnippetIOPoint, &str>{
        match self.io_points.get_mut(&uuid) {
            Some(result) => return Ok(result),
            None => return Err("snippet io point could not be found with uuid {uuid}")
        };
    }

    //getter and setter methods
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    
    /// get the io points as pipeline connectors
    /// for the snippet manager
    pub fn create_pipeline_connectors_for_io_points(&self, seq_id_generator: &mut SequentialIdGenerator) -> Vec<PipelineConnectorComponent> {
        let mut pipeline_connectors = Vec::with_capacity(self.io_points.len());

        for io_point_pair in &self.io_points {
            pipeline_connectors.push(
                PipelineConnectorComponent::new(seq_id_generator, io_point_pair.0.clone(), &io_point_pair.1.name,  io_point_pair.1.input.clone())
            )
        }

        return pipeline_connectors;
    }
}

/*
impl ExternalSnippetCategory {
    /// Create new category node
    pub fn new(seq_id_generator: &mut SequentialIdGenerator, name: String) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
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

impl ExternalSnippetIOPoint {
    /// create non action io endpoint
    /// useful for connecting snippets together that share no data
    pub fn new_non_acting_input(seq_id_generator: &mut SequentialIdGenerator, input: bool) -> Self {
        let snippet_io_point = ExternalSnippetIOPoint {
            uuid: seq_id_generator.get_id(),
            name: String::from('_'),
            schema: Schema::new(),
            input: input
        };

        return snippet_io_point;
    }

    pub fn new(seq_id_generator: &mut SequentialIdGenerator, name: String, schema: Schema, input: bool) -> Self {
        let snippet_io_point = ExternalSnippetIOPoint {
            uuid: seq_id_generator.get_id(),
            name: name,
            schema: schema,
            input: input
        };

        return snippet_io_point;
    }
}