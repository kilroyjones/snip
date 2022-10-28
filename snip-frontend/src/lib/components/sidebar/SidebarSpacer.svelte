<script lang="ts">
	export let height: number;
	export let color: string;
</script>

<li style="height: {height}px; color:{color}" />

<style>
	li {
		display: flex;
		width: 200px;
	}
</style>
