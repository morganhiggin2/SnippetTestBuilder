use serde::{Deserialize, Serialize};

use crate::state_management::{
    external_snippet_manager::{self, ExternalSnippetManager, PackagePath},
    window_manager::WindowSession,
};

#[derive(Serialize, Deserialize, Default)]
struct Plan {
    actions: PlanActions,
}

#[derive(Serialize, Deserialize, Default)]
struct PlanActions {
    build_snippet_actions: Vec<BuildSnippetAction>,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetAction {
    python_path: PackagePath,
}

pub fn save_project(
    window_session: &mut WindowSession,
    external_snippet_manager: &mut ExternalSnippetManager,
) -> Result((), String) {
    // get components
    let snippet_manager = &mut window_session.snippet_manager;

    // create plan
    let plan = Plan::default();
    // build plan

    // build snippets plan
    // build snippet actions
    for snippet in snippet_manager.get_snippets_as_ref() {
        // find it in external snippet manager
        let external_snippet = match external_snippet_manager
            .find_external_snippet(snippet.get_external_snippet_id())
        {
            None => {
                return Err(format!("Could not find snippet in external snippet manager in project build actions step"));
            }
            Some(external_snippet) => external_snippet,
        };

        // get python path
        let python_path = external_snippet.get_package_path();

        // create snippet action
        let snippet_action = BuildSnippetAction {
            python_path: python_path,
        };

        plan.actions.build_snippet_actions.push(snippet_action);
    }

    // export plan

    return Ok(());
}
