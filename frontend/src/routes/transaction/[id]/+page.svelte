<script>
	export let data;
	const transaction = data.transaction;
</script>

<div class="main">
	<a href="/transactions">back</a>
	{#if transaction}
		<table>
			<tr><td>transaction-type</td><td>{transaction.type}</td></tr>
			<tr><td>date</td><td>{transaction.date}</td></tr>
			<tr><td>amount</td><td>{transaction.amount}</td></tr>
			<tr><td>sender</td><td>{transaction.sender}</td></tr>
			<tr><td>receiver</td><td>{transaction.receiver}</td></tr>
		</table>
	{/if}
</div>
