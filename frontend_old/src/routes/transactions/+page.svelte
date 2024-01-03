<script>
	import List from "../../lib/components/List.svelte";
	import libExportTransactionsJSON from "../../lib/export/transactions_json";
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import url2params from "../../lib/url/url2params";
	import params2url from "../../lib/url/params2url";

	let transactions = null;
	let params = null;
	const reload = async () => {
		params = url2params($page.url.href);
		if (!params.year) {
			params.year = new Date().getFullYear();
		}

		let lastMonth = null;
		const today = new Date();
		transactions = (
			await (await fetch(params2url("/api/transactions", params))).json()
		)
			.map((transaction) => {
				transaction.date = new Date(transaction.date);
				return transaction;
			})
			.sort((a, b) => b.date - a.date)
			.map((transaction) => {
				if (lastMonth !== transaction.date.getMonth()) {
					transaction.newBlock = transaction.date.toLocaleString("en-US", {
						month: "long",
					});
					if (transaction.date.getFullYear() < today.getFullYear()) {
						transaction.newBlock =
							transaction.newBlock + " " + transaction.date.getFullYear();
					}
				}
				lastMonth = transaction.date.getMonth();
				return transaction;
			});
	};
	onMount(reload);

	let extendDownload = false;
	const toggleExtendDownload = () => (extendDownload = !extendDownload);
	const exportTransactionsJSON = () => {
		libExportTransactionsJSON(transactions);
		toggleExtendDownload();
	};

	let extendUpload = false;
	const toggleExtendUpload = () => (extendUpload = !extendUpload);
	function upload() {
		const reader = new FileReader();
		reader.onload = async (event) => {
			try {
				await (
					await fetch("/api/transactions", {
						method: "POST",
						body: event.target.result,
					})
				).json();
				await reload();
			} catch (e) {
				console.error(e);
			} finally {
				toggleExtendUpload();
			}
		};
		reader.readAsText(this.files[0]);
	}

	let deleteMode = false;
	let transactionsToDelete = [];
	const toggleDelete = async () => {
		deleteMode = !deleteMode;
		if (deleteMode) {
			transactionsToDelete = [];
			return;
		}
		transactionsToDelete = transactions.filter((transaction) =>
			transactionsToDelete.includes(transaction.id),
		);
		try {
			await fetch("/api/transactions", {
				method: "DELETE",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(transactionsToDelete),
			});
			await reload();
		} catch (e) {
			console.error(e);
		}
	};
	const deleteToggleTransaction = (id) => {
		if (transactionsToDelete.includes(id)) {
			transactionsToDelete.splice(transactionsToDelete.indexOf(id), 1);
			transactionsToDelete = [...transactionsToDelete];
		} else {
			transactionsToDelete = [...transactionsToDelete, id];
		}
	};
	const toggleDeleteToggleTransaction = () =>
		transactions.forEach((transaction) =>
			deleteToggleTransaction(transaction.id),
		);

	const createTransaction = async () => {
		const transactions = [
			{
				type: "out",
				date: new Date(),
				amount: 0,
				sender: "Bar",
				receiver: "",
				budgets: {},
				inbudgets: {},
				debts: {},
				tags: {
					manual: "true",
				},
			},
		];
		try {
			await fetch("/api/transactions", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(transactions),
			});
			await reload();
		} catch (e) {
			console.error(e);
		}
	};

	const lint = (transaction) => {
		const budgetSum = Object.values(transaction.budgets).reduce(
			(a, b) => a + b,
			0,
		);
		const inbudgetSum = Object.values(transaction.inbudgets).reduce(
			(a, b) => a + b,
			0,
		);
		const debtSum = Object.values(transaction.debts).reduce((a, b) => a + b, 0);
		if (transaction.type === "in") {
			return inbudgetSum + debtSum === transaction.amount && budgetSum === 0;
		}
		if (transaction.type === "out") {
			return budgetSum + debtSum === transaction.amount && inbudgetSum === 0;
		}
		return budgetSum === 0 && inbudgetSum === 0 && debtSum === 0;
	};
</script>

<div class="export">
	<div class="flex-fill"></div>
	{#if extendUpload}
		<span>
			<input id="upload-file" type="file" on:change={upload} />
		</span>
	{/if}
	{#if deleteMode}
		<span on:click={() => toggleDeleteToggleTransaction()}>
			<span class="material-icons-sharp">toggle_on</span>
		</span>
	{/if}
	{#if extendDownload}
		<span on:click={() => exportTransactionsJSON()}>
			JSON <span class="material-icons-sharp">download</span>
		</span>
	{/if}
	{#if !(extendDownload || extendUpload || deleteMode)}
		<span on:click={() => toggleExtendUpload()}>
			<span class="material-icons-sharp">upload</span>
		</span>
		<span on:click={() => toggleExtendDownload()}>
			<span class="material-icons-sharp">download</span>
		</span>
		<span on:click={() => createTransaction()}>
			<span class="material-icons-sharp">add</span>
		</span>
	{/if}
	{#if !(extendDownload || extendUpload)}
		<span on:click={() => toggleDelete()}>
			<span class="material-icons-sharp">delete</span>
		</span>
	{/if}
</div>
{#if transactions}
	<List
		items={transactions.map((transaction) => ({
			title:
				transaction.type === "out"
					? transaction.sender
					: transaction.type === "in"
						? transaction.receiver
						: `${transaction.sender} to ${transaction.receiver}`,
			subtitle: `${transaction.date.getDate()} ${transaction.date.toLocaleString(
				"en-US",
				{
					month: "short",
				},
			)}`,
			subtitleIcon: null,
			amount: transaction.amount,
			color:
				transaction.type === "out"
					? "red"
					: transaction.type === "in"
						? "green"
						: "gray",
			link: deleteMode ? "" : `/transaction?id=${transaction.id}`,
			link2:
				transaction.attachment && transaction.attachment.length > 0
					? `/api/attachments/${transaction.attachment}`
					: null,
			link2icon: "attach_file",
			link3: null,
			newBlock: transaction.newBlock,
			selected:
				(transactionsToDelete.includes(transaction.id) && deleteMode) ||
				(!deleteMode && !lint(transaction)),
			onclick: deleteMode
				? () => deleteToggleTransaction(transaction.id)
				: null,
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
