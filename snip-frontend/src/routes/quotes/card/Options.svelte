<script lang="ts">
	import Fa from 'svelte-fa';
	import { faEdit, faTrashCan } from '@fortawesome/free-solid-svg-icons';
	import { trashSnippet } from '$lib/server';
	export let id: number;
</script>

<div id="quote-options">
	<button id="option-edit">
		<Fa icon={faEdit} size="1x" fw scale={1.2} color="#2e5507" />
	</button>
	<button id="option-trash" on:click={() => trashSnippet(id)}>
		<Fa icon={faTrashCan} size="1.5x" fw scale={0.8} />
	</button>
</div>

<style>
	#quote-options {
		display: flex;
		flex-direction: column;
		width: 30px;
		padding-top: 10px;
	}

	#option-edit,
	#option-trash {
		border-radius: 5px;
		border: 0px;
		padding: 10px 0px 10px 0px;
		cursor: pointer;
	}

	#option-edit:hover,
	#option-trash:hover {
		background-color: #f8f7f9;
	}

	#option-edit:active,
	#option-trash:active {
		background-color: #d8fe87;
	}

	#option-edit {
		margin-bottom: 8px;
	}
</style>
