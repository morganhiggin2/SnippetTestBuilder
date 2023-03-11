<script>
    import SidebarElement from "./sidebar-element.svelte";
    import ContextMenu from './context_menus/context_menu.svelte';
    import ContextMenuOption from './context_menus/context_menu_option.svelte';
    //files to sidebar
    let files = [
        {id: 0, name: "CustomProject", level: 0, file: false},
        {id: 1, name: "src", level: 1, file: false},
        {id: 2, name: "main.rs", level: 2, file: true}
    ];

    //for context menu
    let showContextMenu;
    let contextMenuPosition = {x: 0, y: 0};

    function onRightClick(e) {
        if (showContextMenu) {
            showContextMenu = false;
        }

        contextMenuPosition = {x: e.clientX, y: e.clientY};
        showContextMenu = true;
    }

    function closeContextMenu() {
        showContextMenu = false;
    }
</script>

<div on:contextmenu|preventDefault={onRightClick} class="body">
    {#each files as file}
        <SidebarElement {...file}/>
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
        background-color: white;
        height: 100%;
        padding-left: 5px;
        padding-top: 2px;
        border-top: 2px solid lightgrey;
        border-right: 2px solid lightgrey;
    }
</style>