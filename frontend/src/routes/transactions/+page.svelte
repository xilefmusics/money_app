<script>
	import List from '../../lib/components/List.svelte';
	import fetch from '../../lib/api/fetch';
	import libExportTransactionsCSV from '../../lib/export/transactions_csv'
	import libExportTransactionsJSON from '../../lib/export/transactions_json'
	import { onMount } from 'svelte';
	import { page } from '$app/stores';

	let transactions = null;
	onMount(async () => {
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
	});

	let extendDownload = false;
	const toggleExtendDownload = () => extendDownload = !extendDownload;
	const exportTransactionsJSON = () => {
		libExportTransactionsJSON(transactions)
		toggleExtendDownload()
	}
	const exportTransactionsCSV = () => {
		libExportTransactionsCSV(transactions)
		toggleExtendDownload()
	}

	let extendUpload = false;
	const toggleExtendUpload = () => extendUpload = !extendUpload;
	const importJSON = () => {
		console.log("import JSON")
		toggleExtendUpload()
	}
	const importCSV = () => {
		console.log("import CSV")
		toggleExtendUpload()
	}
	const importN26 = () => {
		console.log("import N26")
		toggleExtendUpload()
	}
	const importBarclays = () => {
		console.log("import Barclays")
		toggleExtendUpload()
	}

</script>

{#if transactions}
	<div class="export">
		<div class="flex-fill"></div>
		<span style={extendUpload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => importJSON()}>
			JSON <span class="material-icons-sharp">upload</span>
		</span>
		<span style={extendUpload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => importCSV()}>
			CSV <span class="material-icons-sharp">upload</span>
		</span>
		<span style={extendUpload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => importN26()}>
			N26 <span class="material-icons-sharp">upload</span>
		</span>
		<span style={extendUpload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => importBarclays()}>
			Barclays <span class="material-icons-sharp">upload</span>
		</span>
		<span style={extendDownload ? "margin-right: 1rem;" : "visibility: hidden;"} on:click={() => exportTransactionsJSON()}>
			JSON <span class="material-icons-sharp">download</span>
		</span>
		<span style={extendDownload ? "" : "visibility: hidden;"} on:click={() => exportTransactionsCSV()}>
			CSV <span class="material-icons-sharp" >download</span>
		</span>
		<span style={extendDownload || extendUpload ? "visibility: hidden;" : ""} on:click={() => toggleExtendUpload()}>
			<span class="material-icons-sharp">upload</span>
		</span>	
		<span style={extendDownload || extendUpload ? "visibility: hidden;" : ""} on:click={() => toggleExtendDownload()}>
			<span class="material-icons-sharp">download</span>
		</span>	
	</div>
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
			link: `/transaction/${transaction.id}`,
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
