<script>
	import List from '../../lib/components/List.svelte';
	import fetch from '../../lib/api/fetch';
	import libExportTransactionsJSON from '../../lib/export/transactions_json'
	import { onMount } from 'svelte';
	import { page } from '$app/stores';

	let transactions = null;
	const reload = async () => {
		const filter = $page.url.href.split('?')[1] ? '?' + $page.url.href.split('?')[1] : '';
		let lastMonth = null;
		const today = new Date();
		transactions = (await (await fetch(`/api/transactions${filter}`)).json())
			.map((transaction) => {
				transaction.date = new Date(transaction.date);
				return transaction;
			})
			.sort((a, b) => b.date - a.date)
			.map((transaction) => {
				if (lastMonth !== transaction.date.getMonth()) {
					transaction.newBlock = transaction.date.toLocaleString('en-US', { month: 'long' });
					if (transaction.date.getFullYear() < today.getFullYear()) {
						transaction.newBlock = transaction.newBlock + ' ' + transaction.date.getFullYear();
					}
				}
				lastMonth = transaction.date.getMonth();
				return transaction;
			});
	}
	onMount(reload);

	let extendDownload = false;
	const toggleExtendDownload = () => extendDownload = !extendDownload;
	const exportTransactionsJSON = () => {
		libExportTransactionsJSON(transactions)
		toggleExtendDownload()
	}

	let extendUpload = false;
	const toggleExtendUpload = () => extendUpload = !extendUpload;
	function upload() {
		const reader = new FileReader();
    	reader.onload = async event => {
			try {
				await (await fetch("/api/transactions", {
					method: 'POST',
					body: event.target.result
				})).json()
				await reload()
			} catch (e) {
    			console.error(e);
			} finally {
				toggleExtendUpload()
			}
    	};
    	reader.readAsText(this.files[0]);
	}

</script>

<div class="export">
	<div class="flex-fill"></div>
	<span style={extendUpload ? "" : "visibility: hidden;"}>
		<input id="upload-file" type="file" on:change={upload}/>
	</span>
	<span style={extendDownload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => exportTransactionsJSON()}>
		JSON <span class="material-icons-sharp">download</span>
	</span>
	<span style={extendDownload || extendUpload ? "visibility: hidden;" : ""} on:click={() => toggleExtendUpload()}>
		<span class="material-icons-sharp">upload</span>
	</span>	
	<span style={extendDownload || extendUpload ? "visibility: hidden;" : ""} on:click={() => toggleExtendDownload()}>
		<span class="material-icons-sharp">download</span>
	</span>	
</div>
{#if transactions}
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
			link: `/transaction?id=${transaction.id}`,
			link2: null,
			link3: null,
			newBlock: transaction.newBlock
		}))}
	/>
{/if}

<style>
	.export {
		display: flex;
	}
	.flex-fill {
		flex: 1;
	}
</style>
