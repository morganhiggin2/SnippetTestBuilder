<script>
    import PropertiesArea from "./properties_area.svelte";
    import TestCreationArea from "./test_creation_area.svelte";

    let window_height = 0;

</script>

<div class="body">
    <div class="container" style="grid-template-rows: {window_height - 150}px 150px;">
        <div class="test-creation-area">
            <TestCreationArea/>
        </div>
        <div class="properties-view">
            <PropertiesArea/>
        </div>
    </div>
</div>

<svelte:window bind:innerHeight={window_height}/>

<style>
    .body {
        height: 100%;
    }

    .container {
        display: grid;
    }

    .test-creation-area {
        grid-row: 1 / span 1;
    }

    .properties-view {
        grid-row: 2 / span 1;
    }
</style>