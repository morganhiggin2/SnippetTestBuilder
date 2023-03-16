<script>
	import { onMount, setContext, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { key } from './context_menu_key.js';

	export let x;
	export let y;
	
	// whenever x and y is changed, restrict box to be within bounds
    function onPositionChange() {
        // if the menu element is null
        if (!menuElement) {
            return;
        } 
		
        //get the bounding rectangle and make sure the context menu is within the window
		const rect = menuElement.getBoundingClientRect();
		x = Math.min(window.innerWidth - rect.width, x);

        //make sure that the context menu is within the window
		if (y > window.innerHeight - rect.height) { 
            y -= rect.height;
        }
    }

	$: x, y, onPositionChange(); 
	
	const dispatch = createEventDispatcher();	
	
    //send event to children to dispatch to parent of context menu
	setContext(key, {
		dispatchClick: () => dispatch('click')
	});
	
	let menuElement;

    //if the page is clicked
	function onPageClick(e) {
		if (e.target === menuElement || menuElement.contains(e.target)) {
            return;
        } 

		dispatch('clickoutside');
	}
</script>

<style>
	div {
		position: absolute;
		display: grid;
		border: 1px solid #0003;
		box-shadow: 2px 2px 5px 0px #0002;
		background: white;
	}
</style>

<svelte:body on:click={onPageClick} />

<div transition:fade={{ duration: 100 }} bind:this={menuElement} style="top: {y}px; left: {x}px;">
	<slot />
</div>