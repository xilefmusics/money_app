<script>
	import List from '../../lib/components/List.svelte';

	export let data;
	const debts = data.debts;
</script>

<List
	items={debts.map((debt) => ({
		title: debt.name,
		subtitle: null,
		subtitleIcon: null,
		amount: debt.amount,
		color: debt.amount < 0 ? 'green' : debt.amount > 0 ? 'red' : 'gray',
		link: null,
		link2: `/transactions?debt=${debt.name}`,
		link3: null,
		newBlock: null
	}))}
/>

<style>
</style>
