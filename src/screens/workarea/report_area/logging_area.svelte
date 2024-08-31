<script>
    import { invoke, event} from "@tauri-apps/api";
    import { onMount } from "svelte";

    export let window_session_id;
    export let logging_state;
    
    // logging
    let logging_active;

    // TODO possible issue: if there are events being emmited and you are switching between these screens,
    //      is it going to be rendered?
    export const trigger_logging = (stream_i) => {
        // clear text
        logging_state.log_text = "";

        var stream_id = "log_" + stream_i;

        event.listen(stream_id, (event) => {
            var payload = event.payload;

            // if we receive the close log event
            if (payload == "") {
                //TODO maybe not actually ending, maybe not timly (i.e. other events queued before this could get called) 
                logging_active();
            }
            else {
                // append log to logging component
                logging_state.log_text += payload;
            }
        }).then((unlisten) => {
            logging_active = unlisten; 
        });
    };
</script>

<div class="body">
    <textarea bind:value={logging_state.log_text} class="logging-area courier-prime-regular" readonly wrap="hard"/>
</div>

<style>
    .body {
        height: 100%;
        width: 100%;
        background-color: lightgray;
        overflow-y: auto;
        font-family: monospace, monospace;
        border-top: 1px solid lightgrey;
    }
    .logging-area {
        width: 100%;
        height: 100%;
        overflow-y: scroll;
        border-color: white;
        background-color: white;
        padding: 4px;
    }
    .courier-prime-regular {
        font-family: "Courier Prime", monospace;
        font-weight: 400;
        font-style: normal;
    }
</style>
