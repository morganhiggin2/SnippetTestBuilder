use std::{sync::{MutexGuard, Mutex}, collections::HashMap};

use crate::utils::sequential_id_generator::Uuid;
use super::ApplicationState;


pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>
}

struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String ,
    io_points: HashMap<Uuid, SnippetIOPoint>
}

struct SnippetIOPoint {
    uuid: Uuid,
    name: String,
    //the type of content this point serves or receives
    content_type: IOContentType,
    //if it is an input node
    input: bool
}

/// enum for type of content an io point can serve
enum IOContentType {
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
    pub fn create_enpty_snippet(application_state: &mut MutexGuard<ApplicationState>, name: &str) -> Uuid {
        //create external snippet
        let mut external_snippet = ExternalSnippet::empty(application_state, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;


        //create two non-acting snippet io points, one input, one output
        for input_value in [true, false].iter() 
        {
            //create new snippet io points
            let snippet_io_point = SnippetIOPoint::new_non_acting_input(application_state, input_value.to_owned());

            //add existing io point
            external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point);
        }

        //add it to manager
        application_state.external_snippet_manager.external_snippets.push(external_snippet);

        return uuid;
    }

    pub fn create_empty_snippet(application_state: &mut MutexGuard<ApplicationState>, name: &str) -> Uuid {
        //create external snippet
        let mut external_snippet = ExternalSnippet::empty(application_state, name);

        //get uuid of external snippet
        let uuid = external_snippet.uuid;

        //add it to manager
        application_state.external_snippet_manager.external_snippets.push(external_snippet);

        return uuid;
    }

    /// add point to snippet with uuid snippet_uuid
    pub fn add_io_point(application_state: &mut MutexGuard<ApplicationState>, snippet_uuid: Uuid, name: &str, content_type: IOContentType, input: bool) -> Result<(), &'static str> {
        //find external snippet
        let external_snippet = match application_state.external_snippet_manager.find_external_snippet(snippet_uuid) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        //create new point
        let snippet_io_point: SnippetIOPoint = SnippetIOPoint::new(application_state, name, content_type, input);

        //add point
        external_snippet.io_points.insert(snippet_io_point.uuid, snippet_io_point);

        //return good result
        return Ok(());
    }

    /// find external snippet from within the external snippet manager
    /// 
    /// # Arguments
    /// 
    /// * 'uuid' - uuid of the external snippet to find
    fn find_external_snippet(&mut self, uuid: Uuid) -> Result<&mut ExternalSnippet, &str> {
        let result = match self.external_snippets.iter_mut().find(|pipe: &&mut ExternalSnippet | pipe.uuid == uuid) {
            Some(result) => return Ok(result),
            None => return Err("external snippet could not be found with uuid {uuid}")
        };
    }

}

impl ExternalSnippet {
    fn empty(application_state: &mut MutexGuard<ApplicationState>, name: &str) -> Self {
        //create uuid for external snippet
        let uuid = application_state.seq_id_generator.get_id();

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
        let result = match self.io_points.get_mut(&uuid) {
            Some(result) => return Ok(result),
            None => return Err("snippet io point could not be found with uuid {uuid}")
        };
    }
}

impl SnippetIOPoint {
    /// create non action io endpoint
    /// useful for connecting snippets together that share no data
    pub fn new_non_acting_input(application_state: &mut MutexGuard<ApplicationState>, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: application_state.seq_id_generator.get_id(),
            name: String::from('_'),
            content_type: IOContentType::None,
            input: input
        };

        return snippet_io_point;
    }

    pub fn new(application_state: &mut MutexGuard<ApplicationState>, name: &str, content_type: IOContentType, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: application_state.seq_id_generator.get_id(),
            name: String::from(name),
            content_type: content_type,
            input: input
        };

        return snippet_io_point;
    }
}