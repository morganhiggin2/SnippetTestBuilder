use std::sync::MutexGuard;

use crate::utils::sequential_id_generator::Uuid;
use super::ApplicationState;


pub struct ExternalSnippetManager {
    external_snippets: Vec<ExternalSnippet>
}

struct ExternalSnippet {
    uuid: Uuid,
    sub_directory: String,
    name: String ,
    io_points: Vec<SnippetIOPoint>
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

        //add two nonaction io endpoints
        external_snippet.io_points.push(
            SnippetIOPoint::non_acting_input(application_state, true)
        );
        external_snippet.io_points.push(
            SnippetIOPoint::non_acting_input(application_state, false)
        );

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
    pub fn add_io_point(snippet_uuid: Uuid) -> Result {

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
            io_points: Vec::with_capacity(2)
        };

        return external_snippet;
    }
}

impl SnippetIOPoint {
    /// create non action io endpoint
    /// useful for connecting snippets together that share no data
    pub fn non_acting_input(application_state: &mut MutexGuard<ApplicationState>, input: bool) -> Self {
        let snippet_io_point = SnippetIOPoint {
            uuid: application_state.seq_id_generator.get_id(),
            name: String::from('_'),
            content_type: IOContentType::None,
            input: input
        };

        return snippet_io_point;
    }
}