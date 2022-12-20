<script>
	export let data;
	const pod = data.pod;
</script>

<div class="main">
	<a href="/pods">back</a>
	{#if pod}
		<table>
			<tr><td>pod-type</td><td>{pod['pod-type']}</td></tr>
			<tr><td>name</td><td>{pod.name}</td></tr>
		</table>
	{/if}
</div>
