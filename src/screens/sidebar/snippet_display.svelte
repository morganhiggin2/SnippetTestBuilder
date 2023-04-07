<script>
    import SidebarElement from "./sidebar_element.svelte";
    import ContextMenu from './context_menus/context_menu.svelte';
    import ContextMenuOption from './context_menus/context_menu_option.svelte';
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    //files to sidebar
    let files = [];

    onMount(() => {
        files = invoke('get_snippet_directory', {}).then();
        //set parent files to be showing
        for (const [i, file] of files.entries()) {
            if (file.level == 0) {
                files[i].showing = true;
            }
        }
    });

    function fileExpand(id) {
        //expand files for level just under file of id

    }

    function fileContract(id) {
        //contract all files under level of file of id
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
        <div>
            <SidebarElement {...file} on:expand={fileExpand} on:contract={fileContract}/>
        </div>
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
    }
</style>