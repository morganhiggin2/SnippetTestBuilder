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
                //set files to be the list of snippet files and directories
                workspace_files = result;
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
                on:triggerLogging={trigger_logging}
                files={workspace_files}
            />
        {/if}
    {/if}
</div>

<style>
</style>
