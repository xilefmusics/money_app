<script>
	import List from '../../lib/components/List.svelte';

	export let data;
	const transactions = data.transactions;
</script>

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

<style>
</style>
