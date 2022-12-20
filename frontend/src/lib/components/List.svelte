<script>
	import CharacterIcon from '../../lib/components/CharacterIcon.svelte';

	export let items;
</script>

<div class="main">
	{#each items as item, idx}
		{#if item.newBlock}
			{item.newBlock}
		{/if}
		<a href={item.link} class="item">
			<div class="left">
				<CharacterIcon character={item.title[0]} />
			</div>
			<div class="middle">
				{#if item.title}
					<div class="title">
						{item.title}
					</div>
				{/if}
				{#if item.subtitle}
					<div class="subtitle">
						{#if item.subtitleIcon}
							<span class="material-icons-sharp">{item.subtitleIcon}</span>
						{/if}
						{item.subtitle}
					</div>
				{/if}
			</div>
			<div class="middle2">
				<div class="flex-fill" />
				<div class="middle2inner">
					{#if item.link2}
						<a href={item.link2} class="material-icons-sharp">list_alt</a>
					{/if}
					{#if item.link3}
						<a href={item.link3} class="material-icons-sharp">equalizer</a>
					{/if}
				</div>
				<div class="flex-fill" />
			</div>
			<div class="right">
				<div class="flex-fill" />
				<div class="flex-fill" />
				{#if item.amount}
					<div
						class={`amount ${
							item.color === 'red' ? 'red' : item.color === 'green' ? 'green' : 'gray'
						}`}
					>
						{(item.amount / 100).toFixed(2)} €
					</div>
				{/if}
				<div class="flex-fill" />
			</div>
		</a>
	{/each}
</div>

<style>
	a {
		text-decoration: none;
	}
	.item {
		display: flex;
		padding: 1em;
		margin: 0.75em;
		background-color: #3c3836;
		box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.3);
	}
	.left {
		text-align: left;
		display: flex;
		flex-direction: column;
	}
	.middle {
		text-align: left;
		flex: 1;
		display: flex;
		flex-direction: column;
	}
	.middle2 {
		text-align: right;
		display: flex;
		flex-direction: column;
		font-size: 1.5em;
		flex-direction: column;
		margin-right: 1em;
	}
	.right {
		text-align: right;
		display: flex;
		flex-direction: column;
	}
	.middle2inner a {
		font-size: 2rem;
	}
	.subtitle {
		font-size: 0.9em;
		margin-top: 0.2em;
		color: #d5c4a1;
	}
	.subtitle .material-icons-sharp {
		font-size: 0.9em;
	}
	.amount {
		padding: 0.4em;
		border-radius: 5px;
		flex: 0;
	}
	.green {
		background-color: #98971a;
	}
	.red {
		background-color: #cc241d;
	}
	.gray {
		background-color: #665c54;
	}
</style>
