<script>
	import { onMount, setContext, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { key } from './context_menu_key.js';

	export let x;
	export let y;
	
	// whenever x and y is changed, restrict box to be within bounds
    function onPositionChange() {
        if (!menuElement) return;
		
		const rect = menuElement.getBoundingClientRect();
		x = Math.min(window.innerWidth - rect.width, x);
		if (y > window.innerHeight - rect.height) y -= rect.height;
    }

	$: x, y, onPositionChange(); 
	
	const dispatch = createEventDispatcher();	
	
	setContext(key, {
		dispatchClick: () => dispatch('click')
	});
	
	let menuElement;
	function onPageClick(e) {
		if (e.target === menuElement || menuElement.contains(e.target)) return;
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