<script>
	import List from '../../lib/components/List.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch'

	let debts = null
    onMount(async () => {
		let debtHistory = await (await fetch(`/api/history/debt`)).json();
		debts = Object.keys(debtHistory[0]).filter(key => key !== 'date').map((key) => ({
			'name': key,
			'amount': debtHistory[0][key].value,
		}));
    })
</script>

{#if debts}
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
{/if}

<style>
</style>
