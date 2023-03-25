<script>
    import { identity } from "svelte/internal";

    export let id;
    export let text;

    /*
    
    
    .body {
        background-color: whitesmoke;
        display: inline-block;
    }

    
    .text {

        text-decoration: none;
    }
<div class="body">
    <div class="text">
        {text}
    </div>
</div>
    */
</script>

<div class="body">
    <div class="text">
        {text}
    </div>
</div>

<style>
    .body {
        display: inline-block;
        padding: 4px;
        padding-right: 25px;
        border-bottom: 2px solid whitesmoke;
        border-left: 1px solid lightgrey;
        border-right: 1px solid lightgrey;
        margin-bottom: -2px;
    }

    .body:hover {
        border-bottom: 2px solid blue;
    }    

    .text {
        padding-bottom: 2px;
    }
</style>