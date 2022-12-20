<script>
	export let data;
	const history = data.history;
</script>

<div class="main">
	<a href="/pods">back</a>
	{#if history}
		<table>
			<tr>
				<th>Date</th>
				<th>Amount</th>
				<th>Change</th>
				<th>In</th>
				<th>Out</th>
			</tr>
			{#each history as historyItem, idx}
				<tr>
					<td
						>{historyItem.date.toLocaleString('en-US', { month: 'short' })}. {historyItem.date.getFullYear()}</td
					>
					<td>{(historyItem.amount / 100).toFixed(2)} €</td>
					<td>{(historyItem.change / 100).toFixed(2)} €</td>
					<td>{(historyItem.in / 100).toFixed(2)} €</td>
					<td>{(historyItem.out / 100).toFixed(2)} €</td>
				</tr>
			{/each}
		</table>
	{/if}
</div>
