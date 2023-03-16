<script>
    import NavigationBar from './navigation-bar.svelte';
    import Sidebar from './sidebar/sidebar.svelte';
    import Workarea from './workarea.svelte';
</script>

<div class="container">
    <div class="navigation-bar">
        <NavigationBar/> 
    </div>
    <div class="body sidebar">
        <Sidebar/>
    </div>
    <div class="body work-area">
        <Workarea/> 
    </div>
</div>

<style>
    .container{
        display: grid;
        grid-template-columns: 150px 100%;
    }

    .navigation-bar {
        grid-column: 1 / span 2;
    }

    .sidebar {
        grid-column: 1 / span 1;
    }

    .work-area{
        grid-column: 2 / span 1;
    }

    .body {
        min-height: 100vh;
    }
</style>