<script lang="ts">
	import Fa from 'svelte-fa';
	export let icon: any;
	export let toggleMenuExpand: any;
</script>

<li class="align-right" on:click={toggleMenuExpand}>
	<Fa {icon} fw scale={1.2} />
</li>

<style>
	.align-right {
		display: flex;
		align-items: center;
		justify-content: right;
		height: 42px;
		padding-right: 20px;
	}

	li:hover {
		cursor: pointer;
	}
</style>
