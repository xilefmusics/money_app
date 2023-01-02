<script>
	import { onMount } from 'svelte';
	import fetch from '../../../lib/api/fetch';

	export let data;
	const id = data.id;

	let transaction = null;
	onMount(async () => {
		transaction = (await (await fetch(`/api/transactions?id=${id}`)).json()).map((transaction) => {
			transaction.date = new Date(transaction.date);
			return transaction;
		})[0];
	});
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
