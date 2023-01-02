<script>
	import List from '../../lib/components/List.svelte';
	import fetch from '../../lib/api/fetch'
	import { onMount } from 'svelte';
	import { page } from '$app/stores'

	let transactions = null
    onMount(async () => {
        const filter = $page.url.href.split("?")[1] ? "?" + $page.url.href.split("?")[1] : ""
		let lastMonth = null;
		const today = new Date();
		transactions = (await (await fetch(`/api/transactions${filter}`)).json()).map(transaction => {
			transaction.date = new Date(transaction.date);
			return transaction;
		}).sort((a, b) => b.date - a.date).map(transaction => {
			if (lastMonth !== transaction.date.getMonth()) {
				transaction.newBlock = transaction.date.toLocaleString('en-US', { month: 'long' });
				if (transaction.date.getFullYear() < today.getFullYear()) {
					transaction.newBlock = transaction.newBlock + ' ' + transaction.date.getFullYear();
				}
			}
			lastMonth = transaction.date.getMonth();
			return transaction;
		});
    })

</script>

{#if transactions}
<List
	items={transactions.map((transaction) => ({
		title:
			transaction.type === 'out'
				? transaction.sender
				: transaction.type === 'in'
				? transaction.receiver
				: `${transaction.sender} to ${transaction.receiver}`,
		subtitle: `${transaction.date.getDate()} ${transaction.date.toLocaleString('en-US', {
			month: 'short'
		})}`,
		subtitleIcon: null,
		amount: transaction.amount,
		color: transaction.type === 'out' ? 'red' : transaction.type === 'in' ? 'green' : 'gray',
		link: `/transaction/${transaction.id}`,
		link2: null,
		link3: null,
		newBlock: transaction.newBlock
	}))}
/>
{/if}

<style>
</style>
