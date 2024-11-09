<script>
    import FaHammer from "svelte-icons/fa/FaHammer.svelte";
    import GoFileSubmodule from "svelte-icons/go/GoFileSubmodule.svelte";
    //<div class="icon">&#9874;</div>
    //<img class="icon" src={BuildImage} alt="build icon" />
    import BuildImage from "./../../images/BuildIcon.png";

    export let change_screen;
</script>

<div class="body">
    <div
        class="icon"
        on:click={() => {
            change_screen("snippet_directory");
        }}
    >
        <FaHammer />
    </div>
    <div
        class="icon"
        on:click={() => {
            change_screen("workspace");
        }}
    >
        <GoFileSubmodule />
    </div>
</div>

<style>
    .body {
        height: 100%;
        background-color: #3776ab;
        border-top: 2px solid lightgrey;
    }

    .icon {
        margin: 6px;
        margin-top: 6px;
        color: white;
        cursor: pointer;
        &:hover {
            color: black;
        }
    }
</style>
