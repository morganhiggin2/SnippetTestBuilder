use std::{collections::{HashMap, hash_map::Values}};

use serde::{Serialize, Deserialize};

use crate::{utils::sequential_id_generator::{Uuid, SequentialIdGenerator}, core_components::snippet::PipelineConnectorComponent, core_services::{io_service::{FrontExternalSnippetContent, FrontExternalSnippetContentType}, visual_directory_component_manager::{self, VisualDirectoryComponentManager}}};


pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>
}

pub struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String ,
    io_points: HashMap<Uuid, SnippetIOPoint>
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
    JSON
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
    pub fn create_non_acting_snippet(seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, name: &str) -> Uuid {
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
        external_snippet_manager.external_snippets.push(external_snippet);

        return uuid;
    }

    pub fn create_empty_snippet(seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, name: &str) -> Uuid {
        //create external snippet
        let external_snippet = ExternalSnippet::empty(seq_id_generator, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //add it to manager
        external_snippet_manager.external_snippets.push(external_snippet);

        return uuid;
    }

    /// add point to snippet with uuid snippet_uuid
    pub fn add_io_point(seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, snippet_uuid: Uuid, name: &str, content_type: IOContentType, input: bool) -> Result<Uuid, &'static str> {
        //find external snippet
        let external_snippet = match external_snippet_manager.find_external_snippet_mut(snippet_uuid) {
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

    pub fn add_non_acting_point(seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, snippet_uuid: Uuid, input: bool) -> Result<Uuid, &'static str>{
        //find external snippet
        let external_snippet = match external_snippet_manager.find_external_snippet_mut(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: SnippetIOPoint = SnippetIOPoint::new(seq_id_generator,"_", IOContentType::None, input);

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

    pub fn new(seq_id_generator: &mut SequentialIdGenerator, name: &str, content_type: IOContentType, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: seq_id_generator.get_id(),
            name: String::from(name),
            content_type: content_type,
            input: input
        };

        return snippet_io_point;
    }
}