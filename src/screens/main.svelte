<script>
    import NavigationBar from './navigation_bar.svelte';
    import SectionSidebar from './sidebar/section_sidebar.svelte';
    import SnippetDisplay from './sidebar/snippet_display.svelte';
    import Workarea from './workarea/work_area.svelte';

    //for border resizing
    //sidebar-workarea
    let mouse_pos = {x: 0, y: 0};

    let secondary_sidebar_width = 150;
    let secondary_sidebar_workarea_resize_x_pos = 0;
    let secondary_sidebar_workarea_resize_in_action = false;

    function secondarySidebarWorkareaResizeStart(event) {
        secondary_sidebar_workarea_resize_x_pos = event.pageX;
        secondary_sidebar_workarea_resize_in_action = true;
    }

    function handleMouseMove(event) {
        mouse_pos = {x: event.clientX, y: event.clientY};

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

</script>

<div class="container" style="grid-template-columns: 50px {secondary_sidebar_width}px 2px 100%;" on:mousemove={handleMouseMove} on:mouseup={handleMouseUp}>
    <div class="navigation-bar">
        <NavigationBar/> 
    </div>
    <div class="body sidebar" id="primary">
        <SectionSidebar/>
    </div>
    <div class="body sidebar" id="secondary">
        <SnippetDisplay/>
    </div>
    <div class="border" id="sidebar-workarea" on:mousedown={secondarySidebarWorkareaResizeStart}/>
    <div class="body work-area">
        <Workarea/> 
    </div>
</div>

<style>
    .container{
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

    .work-area{
        grid-column: 4 / span 1;
    }

    .body {
        min-height: 100vh;
    }
</style>