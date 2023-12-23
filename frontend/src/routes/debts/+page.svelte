<script>
	import Chart from "../../lib/components/Chart.svelte";
	import List from "../../lib/components/List.svelte";
	import { onMount } from "svelte";

	const isMobile = () => innerHeight > innerWidth;

	let debtHistory = null;
	let debts = null;
	let datasets = null;
	const reload = async () => {
		debtHistory = await (
			await fetch(
				`/api/history/debts?len=${isMobile() ? 6 : 26}&month=3&year=0`,
			)
		).json();

		debts = Object.entries(debtHistory[debtHistory.length - 1].data)
			.map(([key, value]) => ({
				name: key,
				amount: value.value,
			}))
			.sort((a, b) => {
				if (a.name < b.name) return -1;
				if (a.name > b.name) return 1;
				return 0;
			});

		datasets = debts.map((debt) => ({
			label: debt.name,
			data: debtHistory.map((item) => item.data[debt.name].value / 100),
			borderWidth: 1,
		}));
	};
	onMount(reload);
</script>

{#if debts}
	<Chart
		type="line"
		data={{
			labels: debtHistory.map(
				(item) =>
					`${new Date(item.date).getMonth() + 1}-${new Date(
						item.date,
					).getFullYear()}`,
			),
			datasets: datasets,
		}}
		options={{
			responsive: true,
			maintainAspectRatio: true,
			aspectRatio: isMobile() ? 0.8 : 2,
		}}
	/>
	<List
		items={debts.map((debt) => ({
			title: debt.name,
			subtitle: null,
			subtitleIcon: null,
			amount: debt.amount,
			color: debt.amount < 0 ? "green" : debt.amount > 0 ? "red" : "gray",
			link: null,
			link2: `/transactions?debt=${debt.name}`,
			link3: null,
			newBlock: null,
		}))}
	/>
{/if}

<style>
</style>
