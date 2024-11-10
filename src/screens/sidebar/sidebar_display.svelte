<script>
    import SnippetDisplay from "./snippet_display.svelte";
    import WorkspaceDisplay from "./workspace_display.svelte";
    import { createEventDispatcher } from "svelte";
    import { onMount } from "svelte";
    import { invoke, event } from "@tauri-apps/api";

    export let window_session_id;
    export let trigger_logging;

    let screens = ["snippet_directory", "workspace"];
    let active_screen = "snippet_directory";

    // local states
    let snippet_files = [];
    let workspace_files = [];

    // project actions
    export let delete_project;

    let logging_dispatch = createEventDispatcher();

    onMount(() => {
        // wait for done event
        event.once("directory_and_workspace_initialized", (event) => {
            invoke("get_snippet_directory_details", {}).then((result) => {
                //set files to be the list of snippet files and directories
                snippet_files = result;

                //set parent snippet_files to be showing
                for (const [i, file] of snippet_files.entries()) {
                    if (file.level == 0) {
                        snippet_files[i].showing = true;
                    }
                }
            });

            invoke("get_workspace_details", {}).then((result) => {
                //set files to be the list of snippet files and directories
                workspace_files = result;
            });
        });

        logging_dispatch("triggerLogging", {
            log_id: window_session_id,
        });

        // call the spawn initalize snippet directory
        invoke("spawn_initialize_snippet_directory_and_workspace", {
            windowSessionUuid: window_session_id,
        }).then((_log_id) => {});
    });

    export function register_listen_to_workspace_refresh() {
        // wait for done event
        event.once("workspace_refreshed", (event) => {
            invoke("get_workspace_details", {}).then((result) => {
                // keep showing the same for files that still exist
                let new_workspace_files = [];

                // build map
                let old_workspace_files_showing_map = new Map();

                for (const workspace_file of workspace_files) {
                    old_workspace_files_showing_map.set(
                        workspace_file.id,
                        workspace_file.showing,
                    );
                }

                for (const workspace_file of result) {
                    if (
                        old_workspace_files_showing_map.has(workspace_file.id)
                    ) {
                        workspace_file.showing =
                            old_workspace_files_showing_map.get(
                                workspace_file.id,
                            );
                    }

                    new_workspace_files.push(workspace_file);
                }

                // This works on the premise that if an entry on a level is showing, all other entries on that level until
                //  we enter a sublevel should show (as in a file system)
                // Ensure that anything on the expanded levels is open
                let level_to_index_map = new Map();
                let level_start_indexes = [];

                for (let i = 0; i < new_workspace_files.length; i++) {
                    const workspace_file = new_workspace_files[i];
                    let level = workspace_file.level;

                    // prune higher levels
                    while (
                        level_start_indexes.length > 0 &&
                        level_start_indexes[level_start_indexes.length - 1]
                            .level > level
                    ) {
                        let previous_level_information =
                            level_start_indexes.pop();

                        // go back and if this level is marked as showing, then for each nonshowing in nonshowing list
                        //  mark as showing
                        // if all should be showing
                        if (previous_level_information.all_showing == true) {
                            for (const not_showing_index of previous_level_information.not_showing_indexes) {
                                // set to showing
                                new_workspace_files[not_showing_index].showing =
                                    true;
                            }
                        }
                    }

                    // if a greater level or nothing in levels array
                    // new entry in levels
                    if (
                        level_start_indexes.length == 0 ||
                        level >
                            level_start_indexes[level_start_indexes.length - 1]
                                .level
                    ) {
                        level_start_indexes.push({
                            level: level,
                            all_showing: false,
                            not_showing_indexes: [],
                        });

                        level_to_index_map[level] =
                            level_start_indexes.length - 1;
                    }

                    // if level is showing
                    if (workspace_file.showing == true) {
                        // get level information
                        let level_information =
                            level_start_indexes[level_to_index_map[level]];

                        // set to is showing if not already showing
                        level_information.all_showing = true;
                    } else {
                        // get level information
                        let level_information =
                            level_start_indexes[level_to_index_map[level]];

                        // add to not showing list
                        level_information.not_showing_indexes.push(i);
                    }
                }

                // for any other levels that have yet to be evaluated
                for (const previous_level_information of level_start_indexes) {
                    // go back and if this level is marked as showing, then for each nonshowing in nonshowing list
                    //  mark as showing
                    // if all should be showing
                    if (previous_level_information.all_showing == true) {
                        for (const not_showing_index of previous_level_information.not_showing_indexes) {
                            // set to showing
                            new_workspace_files[not_showing_index].showing =
                                true;
                        }
                    }
                }

                //set files to be the list of snippet files and directories
                workspace_files = new_workspace_files;
            });
        });
    }

    export function change_screen(screen) {
        active_screen = screen;
    }
</script>

<div class="body">
    {#if screens.includes(active_screen)}
        {#if active_screen === "snippet_directory"}
            <SnippetDisplay
                {window_session_id}
                on:triggerLogging={trigger_logging}
                files={snippet_files}
            />
        {:else if active_screen === "workspace"}
            <WorkspaceDisplay
                {window_session_id}
                {delete_project}
                on:triggerLogging={trigger_logging}
                files={workspace_files}
            />
        {/if}
    {/if}
</div>

<style>
</style>
