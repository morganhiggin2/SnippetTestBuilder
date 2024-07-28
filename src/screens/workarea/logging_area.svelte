<script>
    import { invoke, event} from "@tauri-apps/api";

    export let window_session_id;
    
    export const trigger_logging = (stream_i) => {
        invoke('logln', {text: 'hello ' + stream_i});
    }
    /*
    
    // logging
    let logging_active = null;

    export const trigger_logging = (stream_i) => {
        var stream_id = "log_" + stream_i;

        //invoke('logln', {text: 'registering event ' + stream_id});

        let unlisten = event.listen(stream_id, (event) => {
            var payload = event.payload;

            // if we receive the close log event
            if (payload == "") {
                // unlisten to self
                logging_active();
                logging_active = null;
            }
            else {
                // append log to logging component
                invoke('logln', {text: JSON.stringify(payload)});
            }
        });

        logging_active = unlisten;
    };*/
</script>

<div class="body">

</div>

<style>
    .body {
        height: 100%;
        width: 100%;
        background-color: white;
        border-top: 2px solid lightgrey;
        overflow-y: auto;
    }
</style>
