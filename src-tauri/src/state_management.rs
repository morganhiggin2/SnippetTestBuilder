use std::sync::{Mutex};

use crate::core_services::directory_manager::{DirectoryManager, ExternalSnippetFileContainer};
use crate::state_management::window_manager::WindowManager;
use crate::utils::sequential_id_generator::{SequentialIdGenerator};
use crate::state_management::external_snippet_manager::{ExternalSnippetManager};
//use crate::core_services::python_service::{call_init_todo_delete_this_method};

use self::external_snippet_manager::IOContentType;

pub mod window_manager;
pub mod external_snippet_manager;
pub mod visual_snippet_component_manager;

pub struct MutexApplicationState(pub Mutex<ApplicationState>);

pub struct ApplicationState {
    //sequential id generator
    pub seq_id_generator: SequentialIdGenerator,
    pub window_manager: WindowManager,
    pub external_snippet_manager: ExternalSnippetManager,
    pub directory_manager: DirectoryManager 
}

impl Default for MutexApplicationState {
    fn default() -> Self {
        return MutexApplicationState(Mutex::new(ApplicationState::default()));
    }
}

impl Default for ApplicationState {
    /// implement default application state
    fn default() -> ApplicationState {
        return ApplicationState {
            seq_id_generator: SequentialIdGenerator::default(),
            window_manager: WindowManager::default(),
            external_snippet_manager: ExternalSnippetManager::default(),
            directory_manager: DirectoryManager::default() 
        };
    }    
}

impl ApplicationState {
    pub fn get_window_manager(&mut self) -> &mut WindowManager {
        return &mut self.window_manager; 
    }

    pub fn get_sequence_id_generator(&mut self) ->  &mut SequentialIdGenerator {
        return  &mut self.seq_id_generator;
    }

    /// init for the state managment 
    pub fn init(application_state: &mut ApplicationState) {
        //let mut foo = foo_guard.lock().unwrap();
        
        //let mut app = app_guard.lock().unwrap();


        //let sig = &mut app.seq_id_generator;
        //let esp = &mut app.external_snippet_manager;

        //load external snippets
        //TODO delete
        //creating snippets for testing
        /*{
            let sequential_id_generator = &mut application_state.seq_id_generator;
            let external_snippet_manager = &mut application_state.external_snippet_manager;   
            let directory_manager = &mut application_state.directory_manager;

            let mut category = ExternalSnippetCategory::new_parent(sequential_id_generator, "utils".to_string(), 2, 0);
            let category_uuid = category.get_uuid();

            let mut snippet_uuid = external_snippet_manager.create_empty_snippet(sequential_id_generator, "rest_api call");

            external_snippet_manager.add_non_acting_point(sequential_id_generator, snippet_uuid, true); 
            external_snippet_manager.add_io_point(sequential_id_generator, snippet_uuid, "body".to_string(), IOContentType::JSON, false);

            let external_snippet_file_container = ExternalSnippetFileContainer::new(sequential_id_generator, snippet_uuid, category_uuid);
            category.child_snippet_uuids.push(external_snippet_file_container.get_uuid());
            directory_manager.snippet_structure.external_snippet_containers.insert(external_snippet_file_container.get_uuid(), external_snippet_file_container);

           
            snippet_uuid = external_snippet_manager.create_empty_snippet(sequential_id_generator, "middle_body_validator");
            external_snippet_manager.add_io_point(sequential_id_generator, snippet_uuid, "json_input".to_string(), IOContentType::JSON, true); 
            external_snippet_manager.add_non_acting_point(sequential_id_generator, snippet_uuid, false); let external_snippet_file_container = ExternalSnippetFileContainer::new(sequential_id_generator, snippet_uuid, category_uuid);
            category.child_snippet_uuids.push(external_snippet_file_container.get_uuid());
            directory_manager.snippet_structure.external_snippet_containers.insert(external_snippet_file_container.get_uuid(), external_snippet_file_container);
            directory_manager.snippet_structure.root_categories.push(category.get_uuid());
            directory_manager.snippet_structure.categories.insert(category.get_uuid(), category);

            call_init_todo_delete_this_method(sequential_id_generator, external_snippet_manager);
        }*/

        let sequential_id_generator = &mut application_state.seq_id_generator;
        let external_snippet_manager = &mut application_state.external_snippet_manager;   
        let directory_manager = &mut application_state.directory_manager;

        // Initialize directory of snippets
        directory_manager.init(external_snippet_manager, sequential_id_generator);

    }
}















