<script>
    import { invoke, event} from "@tauri-apps/api";

    export let window_session_id;
    
    // logging
    let logging_active;
    let log_text = "";

    export const trigger_logging = (stream_i) => {
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
                log_text += "> " + payload;
            }
        }).then((unlisten) => {
            logging_active = unlisten; 
        });
    };
</script>

<div class="body">
    <textarea bind:value={log_text} class="logging-area" readonly wrap="hard"/>
</div>

<style>
    .body {
        height: 100%;
        width: 100%;
        background-color: white;
        border-top: 2px solid lightgrey;
        overflow-y: auto;
    }
    .logging-area {
        width: 100%;
        height: 100%;
        overflow-y: scroll;
    }
</style>
