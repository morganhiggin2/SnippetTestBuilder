<!--
<div class="body">
    <ul class="navigation-bar">
      <li class="navigation-option"><a href="#">One</a></li>
      <li class="navigation-option"><a href="#">Two</a>
        <ul class="dropdown">
          <li class="dropdown-option"><a href="#">Sub-1</a></li>
          <li class="dropdown-option"><a href="#">Sub-2</a></li>
          <li class="dropdown-option"><a href="#">Sub-3</a></li>
        </ul>
      </li>
      <li class="navigation-option"><a href="#">Three</a></li>
    </ul>
</div>

<style>
    .body {
        background-color: whitesmoke;
        height: 100%;
    }

    .navigation-option {
        display: block;
        position: relative;
        text-decoration: none;
        text-align: left;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px;
        float: left;
        border-bottom: 2px solid whitesmoke;
        z-index: 99;
    }

    .navigation-option:hover {
        border-color: blue;
        cursor: pointer;
    }

    .navigation-option a {
        color: black;
        z-index: 99;
    }

    .dropdown {
        background-color: whitesmoke;
        visibility: hidden;
        position: absolute;
        padding-left: 2px;
        margin-top: 4px;
        margin-left: -5px;
        opacity: 0;
        min-width: 80px;
        z-index: 99;
    }

    .navigation-option:hover > ul,

    .dropdown:hover {
        visibility: visible;
        opacity: 1;
        display: block;
    }

    .dropdown-option {
        clear: both;
        width: 100%;
        padding-top: 2px;
        padding-bottom: 2px;
        border-right: 2px solid whitesmoke;
        z-index: 99;
    }

    .dropdown-option:hover {
        border-color: blue;
    }
</style>
-->
<script>
    import { createEventDispatcher } from "svelte";
    import { invoke, event } from "@tauri-apps/api";

    export let window_session_id;

    let logging_dispatch = createEventDispatcher();

    function handleRunClick(e) {
        // wait for done event
        event.once("snippets ran", (event) => {
            // nothing?
        });

        logging_dispatch("triggerLogging", {
            log_id: window_session_id,
        });

        // call run for snippet state
        invoke("spawn_run_snippets", { windowSessionUuid: window_session_id })
            .then((stream_id) => {})
            .catch((e) => {
                invoke("logln", { text: JSON.stringify(e) });
            });
    }

    function handleSaveClick(e) {
        invoke("save_project", { windowSessionUuid: window_session_id })
            .then(() => {})
            .catch((e) => {
                invoke("logln", { text: JSON.stringify(e) });
            });
    }
    /*
<div class="body">
    <ul class="navigation-bar">
        <li class="navigation-option" id="plain">File</li>
        <li class="navigation-option" id="plain">Options</li>
        <li class="navigation-option" id="dropdown">
            <div class="dropdown-text">
                Tools
            </div>
            <ul class="dropdown-content">
                <li class="dropdown-option">Sub one</li>
                <li class="dropdown-option">Sub two</li>
                <li class="dropdown-option">Sub three</li>
            </ul>
        </li>
    </ul>
    <ul class="nagivation-bar">
        <div class="button play" on:click={handleRunClick} on:keydown={() => {}}>
        </div>
    </ul>
</div>*/
</script>

<div class="body">
    <div />
    <div class="navigation-bar">
        <div
            class="button play"
            on:click={handleRunClick}
            on:keydown={() => {}}
        ></div>
        <button
            class="button save"
            on:click={handleSaveClick}
            on:keydown={() => {}}
        >
            Save
        </button>
    </div>
</div>

<style>
    .body {
        background-color: whitesmoke;
        cursor: default;
        display: flex;
        justify-content: space-between;
        width: 100%;
    }
    .navigation-bar {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .navigation-bar #plain.navigation-option {
        float: left;
        font-size: 16px;
        color: black;
        text-decoration: none;
        text-align: left;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px;
        border-bottom: 2px solid whitesmoke;
    }

    #dropdown.navigation-option {
        float: left;
        overflow: hidden;
    }

    #dropdown.navigation-option .dropdown-text {
        font-size: 16px;
        border: none;
        outline: none;
        color: black;
        background-color: inherit;
        font-family: inherit;
        margin: 0;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px;
        border-bottom: 2px solid whitesmoke;
    }

    #plain.navigation-option:hover,
    #dropdown.navigation-option:hover .dropdown-text {
        border-color: blue;
    }

    .dropdown-content {
        display: none;
        position: absolute;
        min-width: 80px;
        background-color: whitesmoke;
        box-shadow: 0px 8px 16px 0px rgba(0, 0, 0, 0.2);
        z-index: 1;
    }

    .dropdown-content li {
        color: black;
        text-decoration: none;
        display: block;
        text-align: left;
        border-right: 2px solid whitesmoke;
    }

    .dropdown-content li:hover {
        border-color: blue;
    }

    #dropdown.navigation-option:hover .dropdown-content {
        display: block;
    }

    .dropdown-option {
        padding-left: 2px;
        margin: 4px;
    }

    .button {
        background-color: blue;
        height: 100%;
        margin: 4px;
    }

    .button.play {
        border-top: 11px solid whitesmoke;
        border-bottom: 12px solid whitesmoke;
        border-left: 19px solid #02a667;
        height: 0px;
    }

    .button.play:hover {
        border-left: 19px solid lightgreen;
        cursor: pointer;
    }

    .button.save {
        background-color: #02a667;
        height: 20px;
        margin: 4px;
    }

    .button.save:hover {
        background-color: lightgreen;
        cursor: pointer;
    }

    .button.save:active {
        background-color: darkgreen;
    }
</style>
