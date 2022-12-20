<script>
	import Chart from '../../lib/components/Chart.svelte';

	export let data;
	const amountHistory = data.amountHistory;
</script>

<Chart
	type="line"
	data={{
		labels: amountHistory.map((amount) => amount.date),
		datasets: [
			{
				label: 'Sum',
				data: amountHistory.map((amount) => amount.sum / 100),
				borderWidth: 1,
				borderColor: 'blue',
				backgroundColor: 'blue'
			},
			{
				label: 'Real',
				data: amountHistory.map((amount) => amount.real / 100),
				borderWidth: 1,
				borderColor: 'lightgreen',
				backgroundColor: 'lightgreen'
			},
			{
				label: 'Debt',
				data: amountHistory.map((amount) => amount.debt / 100),
				borderWidth: 1,
				borderColor: 'purple',
				backgroundColor: 'purple'
			},
			{
				label: 'In',
				data: amountHistory.map((amount) => amount.income / 100),
				borderWidth: 1,
				borderColor: 'green',
				backgroundColor: 'green'
			},
			{
				label: 'Out',
				data: amountHistory.map((amount) => amount.outcome / 100),
				borderWidth: 1,
				borderColor: 'red',
				backgroundColor: 'red'
			}
		],
		options: {}
	}}
/>
