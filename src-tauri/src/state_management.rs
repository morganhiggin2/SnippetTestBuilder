use std::sync::{Mutex, MutexGuard};
use tauri::Window;

use crate::core_services::io_service::{DirectoryManager, ExternalSnippetCategory, ExternalSnippetFileContainer};
use crate::state_management::window_manager::WindowManager;
use crate::utils::sequential_id_generator::{SequentialIdGenerator, self};
use crate::state_management::external_snippet_manager::{ExternalSnippetManager};

use self::external_snippet_manager::IOContentType;

pub mod window_manager;
pub mod external_snippet_manager;

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

struct Foo {
    a: u32, 
    b: u32,
    c: u32
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
        {
            let sequential_id_generator = &mut application_state.seq_id_generator;
            let external_snippet_manager = &mut application_state.external_snippet_manager;   
            let directory_manager = &mut application_state.directory_manager;

            let mut category = ExternalSnippetCategory::new_parent(sequential_id_generator, "utils".to_string(), 2, 0);
            let category_uuid = category.get_uuid();

            let mut snippet_uuid = ExternalSnippetManager::create_empty_snippet(sequential_id_generator, external_snippet_manager, "rest_api call");
            ExternalSnippetManager::add_non_acting_point(sequential_id_generator, external_snippet_manager, snippet_uuid, true); 
            ExternalSnippetManager::add_io_point(sequential_id_generator, external_snippet_manager, snippet_uuid, "body", IOContentType::JSON, false);

            let external_snippet_file_container = ExternalSnippetFileContainer::new(sequential_id_generator, snippet_uuid, category_uuid);
            category.child_snippet_uuids.push(external_snippet_file_container.get_uuid());
            directory_manager.snippet_structure.external_snippet_containers.insert(external_snippet_file_container.get_uuid(), external_snippet_file_container);

           
            snippet_uuid = ExternalSnippetManager::create_empty_snippet(sequential_id_generator, external_snippet_manager, "middle_body_validator");
            ExternalSnippetManager::add_io_point(sequential_id_generator, external_snippet_manager, snippet_uuid, "json_input", IOContentType::JSON, true); 
            ExternalSnippetManager::add_non_acting_point(sequential_id_generator, external_snippet_manager, snippet_uuid, false); let external_snippet_file_container = ExternalSnippetFileContainer::new(sequential_id_generator, snippet_uuid, category_uuid);
            category.child_snippet_uuids.push(external_snippet_file_container.get_uuid());
            directory_manager.snippet_structure.external_snippet_containers.insert(external_snippet_file_container.get_uuid(), external_snippet_file_container);
            directory_manager.snippet_structure.root_categories.push(category.get_uuid());
            directory_manager.snippet_structure.categories.insert(category.get_uuid(), category);

        }
    }
}














