<script lang="ts">
	import Fa from 'svelte-fa';
	import { fade } from 'svelte/transition';

	export let icon: any;
	export let setting: any;
	export let settingValue: boolean;
	export let text: string;
	export let isExpanded: boolean;
	export let size: number = 1;

	const fadeIn = {
		delay: 100,
		duration: 200
	};

	const fadeOut = {
		delay: 0,
		duration: 100
	};
</script>

<li on:click={() => setting.toggle()}>
	<Fa {icon} size="{size}x" fw scale={1.2} color={settingValue ? '#439a68' : '#aa5042'} />
	{#if isExpanded}<span in:fade={fadeIn} out:fade={fadeOut}>{text}</span>{/if}
</li>

<style>
	li {
		display: flex;
		align-items: center;
		width: 200px;
		border-top: solid 1px #dfdfdf;
		padding-left: 20px;
		height: 42px;
	}

	li:hover {
		background-color: rgb(234, 250, 255);
		cursor: pointer;
	}

	span {
		color: #363821;
		padding-left: 30px;
		font-family: 'Lato', Arial, sans-serif;
		font-weight: 300;
		font-size: 16px;
	}
</style>
