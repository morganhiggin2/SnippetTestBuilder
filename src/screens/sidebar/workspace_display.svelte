<script>
    export let window_session_id;
    export let files = [];

    function fileExpand(e) {
        //get the id
        let id = e.detail.id;

        //level of found id
        let level = null;

        //expand files for level just under file of id
        for (const [i, file] of files.entries()) {
            if (file.id == id) {
                level = file.level;
            } else if (level != null) {
                if (file.level == level + 1) {
                    files[i].showing = true;
                } else if (file.level <= level) {
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
            } else if (level != null) {
                if (file.level >= level + 1) {
                    file.showing = false;
                } else if (file.level <= level) {
                    level = null;
                    break;
                }
            }
        }

        files = files;
    }

    //for context menu
    let showContextMenu;
    let contextMenuPosition = { x: 0, y: 0 };

    //right click on body detected
    function onRightClick(e) {
        if (showContextMenu) {
            showContextMenu = false;
        }

        contextMenuPosition = { x: e.clientX, y: e.clientY };
        showContextMenu = true;
    }

    //close the context menu
    function closeContextMenu() {
        showContextMenu = false;
    }

    /*
        <div on:contextmenu|preventDefault={onRightClick} class="body noselect">
            {#each files as file}
                {#if file.showing}
                    {#if file.file_type == "Snippet"}
                        <div>
                            <SnippetSidebarElement
                                {...file}
                                on:expand={fileExpand}
                                on:contract={fileContract}
                            />
                        </div>
                    {:else if file.file_type == "Directory"}
                        <div>
                            <DirectorySidebarElement
                                {...file}
                                on:expand={fileExpand}
                                on:contract={fileContract}
                            />
                        </div>
                    {/if}
                {/if}
            {/each}
        </div>

        {#if showContextMenu}
            <ContextMenu
                {...contextMenuPosition}
                on:click={closeContextMenu}
                on:clickoutside={closeContextMenu}
            >
                <ContextMenuOption on:click={() => {}} text="Do nothing" />
            </ContextMenu>
        {/if}*/
</script>

<div class="body"></div>

<style>
    .body {
        background-color: whitesmoke;
        height: 100%;
        width: 100%;
        padding-left: 5px;
        padding-top: 2px;
        border-top: 2px solid lightgrey;
        white-space: nowrap;
        overflow-y: auto;
        cursor: default;
    }
</style>
