<script>
	import ContextMenu from './context_menu.svelte';
	import ContextMenuOption from './context_menu_option.svelte';
	import { tick } from 'svelte'
	
	let pos = { x: 0, y: 0 };
	let showMenu = false;
	
	async function onRightClick(e) {
		if (showMenu) {
			showMenu = false;
			await new Promise(res => setTimeout(res, 100));
		}
		
		pos = { x: e.clientX, y: e.clientY };
		showMenu = true;
	}
	
	function closeMenu() {
		showMenu = false;
	}
</script>

{#if showMenu}
	<ContextMenu {...pos} on:click={closeMenu} on:clickoutside={closeMenu}>
		<ContextMenuOption 
			on:click={console.log} 
			text="Do nothing" />
	</ContextMenu>
{/if}

<svelte:body on:contextmenu|preventDefault={onRightClick} />