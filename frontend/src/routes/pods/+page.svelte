<script>
	import List from '../../lib/components/List.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch'

	let pods = null
    onMount(async () => {
		let podHistory = await (await fetch(`/api/history/pod`)).json();
		pods = Object.keys(podHistory[0]).filter(key => key !== 'date').map((key) => ({
			'name': key,
			'amount': podHistory[0][key].value,
		}));
    })
</script>

{#if pods}
<List
	items={pods.map((pod) => ({
		title: pod.name,
		subtitle: null,
		subtitleIcon: null,
		amount: pod.amount,
		color: pod.amount > 0 ? 'green' : pod.amount < 0 ? 'red' : 'gray',
		link: `/pod/${pod.name}`,
		link2: `/transactions?pod=${pod.name}`,
		link3: `/pod-history/${pod.name}`,
		newBlock: null
	}))}
/>
{/if}

<style>
</style>
