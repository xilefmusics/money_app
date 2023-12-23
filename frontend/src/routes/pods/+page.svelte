<script>
	import Chart from "../../lib/components/Chart.svelte";
	import List from "../../lib/components/List.svelte";
	import { onMount } from "svelte";

	const isMobile = () => innerHeight > innerWidth;

	let pods = null;
	let podHistory = null;
	let datasets = null;
	const reload = async () => {
		podHistory = await (
			await fetch(`/api/history/pods?len=${isMobile() ? 6 : 26}&month=3&year=0`)
		).json();

		pods = Object.entries(podHistory[podHistory.length - 1].data).map(
			([key, value]) => ({
				name: key,
				amount: value.value,
			}),
		);

		datasets = pods.map((pod) => ({
			label: pod.name,
			data: podHistory.map((item) => item.data[pod.name].value / 100),
			borderWidth: 1,
		}));
	};
	onMount(reload);
</script>

{#if pods}
	<Chart
		type="line"
		data={{
			labels: podHistory.map(
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
		items={pods
			.filter((pod) => pod.amount > 0)
			.map((pod) => ({
				title: pod.name,
				subtitle: null,
				subtitleIcon: null,
				amount: pod.amount,
				color: pod.amount > 0 ? "green" : pod.amount < 0 ? "red" : "gray",
				link: null,
				link2: `/transactions?pod=${pod.name}`,
				link3: null,
				newBlock: null,
			}))}
	/>
{/if}

<style>
</style>
