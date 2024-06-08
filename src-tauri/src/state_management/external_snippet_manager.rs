use std::{collections::HashMap, fmt};

use serde::{Serialize, Deserialize};

use crate::{utils::sequential_id_generator::{Uuid, SequentialIdGenerator}, core_components::snippet::PipelineConnectorComponent};


pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>
    // mapping of external snippets to external snippet file containers
}

pub struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String,
    io_points: HashMap<Uuid, SnippetIOPoint>
}

#[derive(Debug)]
pub struct ExternalSnippetCategory {
    uuid: Uuid,
    name: String,
    parent_category_uuid: Option<Uuid>,
    //TODO remove pub
    pub child_snippet_uuids: Vec<Uuid>,
    child_category_uuids: Vec<Uuid> 
}

pub struct SnippetIOPoint {
    uuid: Uuid,
    name: String,
    //the type of content this point serves or receives
    content_type: IOContentType,
    //if it is an input node
    input: bool
}

/// enum for type of content an io point can serve
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum IOContentType {
    //none type for endpoints that send no data, these should have name '_'
    None,
    XML,
    JSON,
    //custom type defined by user
    Custom(String)
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
            let snippet_io_point = SnippetIOPoint::new_non_acting_input(seq_id_generator, input_value.to_owned());

            //add existing io point
            external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point);
        }

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }

    pub fn create_empty_snippet(&mut self, seq_id_generator: &mut SequentialIdGenerator, name: &str) -> Uuid {
        //create external snippet
        let external_snippet = ExternalSnippet::empty(seq_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //add it to manager
        self.external_snippets.push(external_snippet);

        return uuid;
    }

    pub fn add_io_points(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, io_points: Vec::<(String, IOContentType, bool)>) -> Result<(), &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        for io_point in io_points {
            //create new point
            let snippet_io_point: SnippetIOPoint = SnippetIOPoint::new(seq_id_generator, io_point.0, io_point.1, io_point.2);

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

    /// add point to snippet with uuid snippet_uuid
    pub fn add_io_point(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, name: String, content_type: IOContentType, input: bool) -> Result<Uuid, &'static str> {
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: SnippetIOPoint = SnippetIOPoint::new(seq_id_generator, name, content_type, input);

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

    pub fn add_non_acting_point(&mut self, seq_id_generator: &mut SequentialIdGenerator, snippet_uuid: Uuid, input: bool) -> Result<Uuid, &'static str>{
        //find external snippet
        let external_snippet = match self.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: SnippetIOPoint = SnippetIOPoint::new(seq_id_generator,"_".to_string(), IOContentType::None, input);

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
    fn find_io_point(&mut self, external_snippet: &mut ExternalSnippet, uuid: Uuid) -> Result<&mut SnippetIOPoint, &str>{
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
    
    pub fn get_io_points_as_pipeline_connectors(&self, seq_id_generator: &mut SequentialIdGenerator) -> Vec<PipelineConnectorComponent> {
        let mut pipeline_connectors = Vec::with_capacity(self.io_points.len());

        for io_point_pair in &self.io_points {
            pipeline_connectors.push(
                PipelineConnectorComponent::new(seq_id_generator, io_point_pair.0.clone(), &io_point_pair.1.name, &io_point_pair.1.content_type, io_point_pair.1.input.clone())
            )
        }

        return pipeline_connectors;
    }
}

impl ExternalSnippetCategory {
    pub fn new_root(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name,
            parent_category_uuid: None,
            child_snippet_uuids: Vec::with_capacity(num_snippets),
            child_category_uuids: Vec::with_capacity(num_categories)
        };
    }
   
    pub fn new_child(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize, parent_category_uuid: Uuid) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name,
            parent_category_uuid: Some(parent_category_uuid),
            child_snippet_uuids: Vec::with_capacity(num_snippets),
            child_category_uuids: Vec::with_capacity(num_categories)
        };
    }
    
    //TODO remove pub
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_child_categoryies

    pub fn add_child_category(&mut self, category: &ExternalSnippetCategory) {
        self.child_category_uuids.push(category.get_uuid());
    }
}

impl fmt::Display for ExternalSnippetCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl SnippetIOPoint {
    /// create non action io endpoint
    /// useful for connecting snippets together that share no data
    pub fn new_non_acting_input(seq_id_generator: &mut SequentialIdGenerator, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: seq_id_generator.get_id(),
            name: String::from('_'),
            content_type: IOContentType::None,
            input: input,
        };

        return snippet_io_point;
    }

    pub fn new(seq_id_generator: &mut SequentialIdGenerator, name: String, content_type: IOContentType, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: seq_id_generator.get_id(),
            name: name,
            content_type: content_type,
            input: input
        };

        return snippet_io_point;
    }
}