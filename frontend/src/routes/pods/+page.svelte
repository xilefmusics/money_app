<script>
	import List from '../../lib/components/List.svelte';

	export let data;
	const pods = data.pods;
</script>

<List
	items={pods.map((pod) => ({
		title: pod.name,
		subtitle: pod.type,
		subtitleIcon: pod.type === 'insolvent' ? 'lock' : null,
		amount: pod.amount,
		color: pod.amount > 0 ? 'green' : pod.amount < 0 ? 'red' : 'gray',
		link: `/pod/${pod.name}`,
		link2: `/transactions?pod=${pod.name}`,
		link3: `/pod-history/${pod.name}`,
		newBlock: null
	}))}
/>

<style>
</style>
