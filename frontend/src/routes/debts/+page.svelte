<script>
	import Chart from '../../lib/components/Chart.svelte';
	import List from '../../lib/components/List.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch'

	let debtHistory = null
	let debts = null
	let datasets = null
    onMount(async () => {
		debtHistory = await (await fetch(`/api/history/debt?len=10&month=3&year=0`)).json();
		debts = Object.keys(debtHistory[0]).filter(key => key !== 'date').map((key) => ({
			'name': key,
			'amount': debtHistory[0][key].value,
		}));
		datasets = debts.map(item => item.name).map(name => ({
			label: name,
			data: debtHistory.map(item => item[name].value / 100),
			borderWidth: 1,
		}));
    })
</script>

{#if debts}
<Chart
	type="line"
	data={{
		labels: debtHistory.map(item => `${new Date(item.date).getMonth()+1}-${new Date(item.date).getFullYear()}`),
		datasets: datasets,
	}}
/>
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
