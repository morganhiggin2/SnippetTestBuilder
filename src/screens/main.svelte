<script>
    import { onMount } from "svelte";
    import NavigationBar from "./navigation_bar.svelte";
    import SectionSidebar from "./sidebar/section_sidebar.svelte";
    import Workarea from "./workarea/work_area.svelte";
    import { invoke, window } from "@tauri-apps/api";
    import SidebarDisplay from "./sidebar/sidebar_display.svelte";

    // for window uuid
    let window_session_id = 0;

    // properties state
    let project_properties_state;

    // for section sidebar screens
    let change_sidebar_screen;

    //for border resizing
    //sidebar-workarea
    let mouse_pos = { x: 0, y: 0 };

    let secondary_sidebar_width = 150;
    let secondary_sidebar_workarea_resize_x_pos = 0;
    let secondary_sidebar_workarea_resize_in_action = false;

    // temporary open project
    let open_project;
    let delete_project;
    let register_listen_to_workspace_refresh;

    function secondarySidebarWorkareaResizeStart(event) {
        secondary_sidebar_workarea_resize_x_pos = event.pageX;
        secondary_sidebar_workarea_resize_in_action = true;
    }

    function handleMouseMove(event) {
        mouse_pos = { x: event.clientX, y: event.clientY };

        if (secondary_sidebar_workarea_resize_in_action) {
            //change in position of mouse from when it was on the resizable border
            let delta = event.pageX - secondary_sidebar_workarea_resize_x_pos;

            secondary_sidebar_width += delta;
            secondary_sidebar_workarea_resize_x_pos += delta;
        }
    }

    function handleMouseUp() {
        //clear all possible events that require mousedown to be active
        secondary_sidebar_workarea_resize_in_action = false;
    }

    // logging
    let trigger_logging_;

    function trigger_logging(event) {
        trigger_logging_(event.detail.log_id);
    }

    onMount(() => {
        //create new window sesison
        //set id on completion
        invoke("new_window_session").then((result) => {
            window_session_id = result;
        });
    });
</script>

<div>
    <div class="navigation-bar">
        <NavigationBar
            {window_session_id}
            {project_properties_state}
            on:triggerLogging={trigger_logging}
            {register_listen_to_workspace_refresh}
        />
    </div>
    <div
        class="container"
        style="grid-template-columns: 50px {secondary_sidebar_width}px 2px 100%;"
        on:mousemove={handleMouseMove}
        on:mouseup={handleMouseUp}
    >
        <div class="body sidebar" id="primary">
            <SectionSidebar change_screen={change_sidebar_screen} />
        </div>
        <div class="body sidebar" id="secondary">
            <SidebarDisplay
                {window_session_id}
                trigger_logging
                bind:change_screen={change_sidebar_screen}
                bind:register_listen_to_workspace_refresh
                {delete_project}
            />
        </div>
        <div
            class="border"
            id="sidebar-workarea"
            on:mousedown={secondarySidebarWorkareaResizeStart}
        />
        <div class="body work-area">
            <Workarea
                {window_session_id}
                {register_listen_to_workspace_refresh}
                bind:trigger_logging={trigger_logging_}
                bind:project_properties_state
                bind:delete_project
                sidebar_width={50 + secondary_sidebar_width}
            />
        </div>
    </div>
</div>

<style>
    .container {
        display: grid;
    }

    .navigation-bar {
        grid-column: 1 / span 4;
    }

    #primary.sidebar {
        grid-column: 1 / span 1;
    }

    #secondary.sidebar {
        grid-column: 2 / span 1;
    }

    #sidebar-workarea.border {
        grid-column: 3 / span 1;
        background-color: lightgrey;
        cursor: col-resize;
    }

    .work-area {
        grid-column: 4 / span 1;
    }

    .body {
        min-height: 100vh;
    }
</style>
