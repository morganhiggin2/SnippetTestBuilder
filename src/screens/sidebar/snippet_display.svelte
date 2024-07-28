<script>
    import ContextMenu from './context_menus/context_menu.svelte';
    import ContextMenuOption from './context_menus/context_menu_option.svelte';
    import { invoke, event } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import DirectorySidebarElement from "./snippet_sidebar_elements/directory_sidebar_element.svelte";
    import SnippetSidebarElement from "./snippet_sidebar_elements/snippet_sidebar_element.svelte";

    //files to sidebar
    let files = [];

    onMount(() => {
        //invoke('get_snippet_directory', {}).then((result) => {files = result;});
        //console.log(files);
         
        // call the spawn initalize snippet directory
        invoke('spawn_initialize_snippet_directory', {}).then((log_id) => {
            /*logging_dispatch('triggerLogging', {
                log_id: log_id 
            });*/
        });

        // wait for done event
        event.once('directory_initialized', (event) => {
            invoke('get_snippet_directory_details', {}).then((result) => {
                //set files to be the list of snippet files and directories
                files = result;

                //set parent files to be showing
                for (const [i, file] of files.entries()) {
                    if (file.level == 0) {
                        files[i].showing = true;
                    }
                }
            });
        });
        /*logging_dispatch('triggerLogging', {
            log_id: log_id
        });*/
    });

    function fileExpand(e) {
        //get the id
        let id = e.detail.id;

        //level of found id
        let level = null;

        //expand files for level just under file of id
        for (const [i, file] of files.entries()) {
            if (file.id == id) {
                level = file.level;
            }
            else if (level != null) {
                if (file.level == level + 1) {
                    files[i].showing = true;
                }
                else if (file.level <= level) {
                    level = null;
                    break;
                }
            }
        }

        files = files;
    }

    function fileContract(e) {
        //get the id
        let id = e.detail.id;

        //contract all files under level of file of id
        //level of found id
        let level = null;

        //expand files for level just under file of id
        for (const [i, file] of files.entries()) {
            if (file.id == id) {
                level = file.level;
            }
            else if (level != null) {
                if (file.level >= level + 1) {
                    file.showing = false;
                }
                else if (file.level <= level) {
                    level = null;
                    break;
                }
            }
        }

        files = files;
    }

    //for context menu
    let showContextMenu;
    let contextMenuPosition = {x: 0, y: 0};

    //right click on body detected
    function onRightClick(e) {
        if (showContextMenu) {
            showContextMenu = false;
        }

        contextMenuPosition = {x: e.clientX, y: e.clientY};
        showContextMenu = true;
    }

    //close the context menu
    function closeContextMenu() {
        showContextMenu = false;
    }
</script>

<div on:contextmenu|preventDefault={onRightClick} class="body">
    {#each files as file}
        {#if file.showing}
            {#if file.file_type == "Snippet"}
                <div>
                    <SnippetSidebarElement {...file} on:expand={fileExpand} on:contract={fileContract}/>
                </div>
            {:else if file.file_type == "Directory"}
                <div>
                    <DirectorySidebarElement {...file} on:expand={fileExpand} on:contract={fileContract}/>
                </div>
            {/if}
        {/if}
    {/each}
</div>

{#if showContextMenu}
	<ContextMenu {...contextMenuPosition} on:click={closeContextMenu} on:clickoutside={closeContextMenu}>
		<ContextMenuOption 
			on:click={() => {}} 
			text="Do nothing" />
	</ContextMenu>
{/if}

<style>
    .body {
        background-color: whitesmoke;
        height: 100%;
        padding-left: 5px;
        padding-top: 2px;
        border-top: 2px solid lightgrey;
        white-space: nowrap;
        overflow-y: auto;
        cursor: default;
    }
</style>