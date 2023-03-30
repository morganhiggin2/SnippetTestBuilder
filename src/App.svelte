<script>
    import "@fontsource/pt-mono";
    import "@fontsource/inter";
    import Main from './screens/main.svelte';
/**
    font-family: 'PT Mono cyrillic', monospace;
*/
</script>

<style>
    :root {
        background-color: #f6f6f6;
        margin: 0;
        padding: 0;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
        font-family: "Inter";
        font-size: 11pt;
    }

    :global(body) {
        margin: 0;
        padding: 0;
    }
</style>
<Main/>