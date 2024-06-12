// This is the front components for the visual directory component manager
// This will contains the mappings from front to io service directory objects
use core::fmt;
use std::{borrow::Borrow, collections::HashMap, ffi::OsStr, fs::{self, DirEntry}, io::Empty, path::{Display, PathBuf}, rc::Rc, sync::Arc};
use serde::{Serialize, Deserialize};
use std::env;
use pathdiff;
use std::path::Path;

use crate::{state_management::external_snippet_manager::ExternalSnippetManager, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::{directory_manager::{ExternalSnippetFileContainer, SnippetStructure}, visual_directory_component_manager::VisualDirectoryComponentManager};


//struct for the josn serialization
#[derive(Serialize, Deserialize)]
pub struct FrontExternalSnippetContent {
    id: Uuid,
    name: String,
    file_type: FrontExternalSnippetContentType,
    is_directory: bool,
    level: u32,
    showing: bool,
}

#[derive(Serialize, Deserialize)]
pub enum FrontExternalSnippetContentType {
    Directory,
    Snippet
}

impl FrontExternalSnippetContent {
    pub fn new(uuid: Uuid, name: String, internal_id: Uuid, file_type: FrontExternalSnippetContentType, is_directory: bool, level: u32, showing: bool) -> Self {
        let front_content = FrontExternalSnippetContent {
            id: uuid,
            name: name,
            file_type: file_type,
            is_directory: is_directory,
            level: level,
            showing: showing,
        };

        return front_content;
    }

    /// create new front snippet content of type external snippet file container 
    pub fn new_snippet(visual_directory_component_manager: &mut VisualDirectoryComponentManager, external_snippet_manager: &ExternalSnippetManager, snippet_structure: &SnippetStructure, seq_id_generator: &mut SequentialIdGenerator, external_snippet_file_container: &ExternalSnippetFileContainer, level: u32) -> Result<Self, String> {
        //TODO: ask why &str instead of String is not working
        //call front method on file container
        let front_external_snippet_content = match external_snippet_file_container.get_as_front_content(external_snippet_manager, seq_id_generator, level) {
            Ok(result) => result,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        //add front content to visual component manager
        visual_directory_component_manager.put_snippet_file_container_uuid(front_external_snippet_content.id, external_snippet_file_container.get_uuid()); 

        return Ok(front_external_snippet_content);

        //TODO delete these comments
        //find external snippet
        //let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_container.get_external_snippet_uuid()).unwrap();

        //return external_snippet.get_as_front_content(visual_directory_component_manager, external_snippet_manager, seq_id_generator, level);        
    }

    /// create new front snippet content of type category 
    pub fn new_category(visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, name: String,  level: u32) -> Self {
        return FrontExternalSnippetContent::new(
            seq_id_generator.get_id(),
            name.clone(),
            0,
            FrontExternalSnippetContentType::Directory,
            true,
            level,
            false,
        );
    }
}
