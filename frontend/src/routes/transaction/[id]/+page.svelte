<script>
	import { onMount } from 'svelte';
	import fetch from '../../../lib/api/fetch';

	let editMode = false;

	export let data;
	const id = data.id;
	let transaction = null;
	let pods = null;
	let budgets = null;
	let inbudgets = null;
	let debts = null;
	let tags = null;
	onMount(async () => {
		transaction = (await (await fetch(`/api/transactions?id=${id}`)).json()).map((transaction) => {
			const d = new Date(transaction.date);
			const year = `${d.getFullYear()}`;
			const month = d.getMonth() < 9 ? `0${d.getMonth() + 1}` : `${d.getMonth() + 1}`;
			const day = d.getDate() < 9 ? `0${d.getDate()}` : `${d.getDate()}`;
			transaction.date = `${year}-${month}-${day}`;
			return transaction;
		})[0];
		pods = await (await fetch(`/api/pods`)).json();
		budgets = await (await fetch(`/api/budgets`)).json();
		inbudgets = await (await fetch(`/api/inbudgets`)).json();
		debts = await (await fetch(`/api/debts`)).json();
		tags = await (await fetch(`/api/tags`)).json();
	});

	let newBudgetName = null;
	let newBudgetAmount = null;
	const addBudget = () => {
		if (
			newBudgetAmount === null ||
			newBudgetAmount === 0 ||
			newBudgetName === null ||
			newBudgetName === ''
		) {
			return;
		}
		transaction.budgets[newBudgetName] = newBudgetAmount;
		newBudgetName = null;
		newBudgetAmount = null;
	};
	const removeBudget = (name) => {
		let budgets = { ...transaction.budgets };
		delete budgets[name];
		transaction.budgets = budgets;
	};

	let newInbudgetName = null;
	let newInbudgetAmount = null;
	const addInbudget = () => {
		if (
			newInbudgetAmount === null ||
			newInbudgetAmount === 0 ||
			newInbudgetName === null ||
			newInbudgetName === ''
		) {
			return;
		}
		transaction.inbudgets[newInbudgetName] = newInbudgetAmount;
		newInbudgetName = null;
		newInbudgetAmount = null;
	};
	const removeInbudget = (name) => {
		let inbudgets = { ...transaction.inbudgets };
		delete inbudgets[name];
		transaction.inbudgets = inbudgets;
	};

	let newDebtName = null;
	let newDebtAmount = null;
	const addDebt = () => {
		if (
			newDebtAmount === null ||
			newDebtAmount === 0 ||
			newDebtName === null ||
			newDebtName === ''
		) {
			return;
		}
		transaction.debts[newDebtName] = newDebtAmount;
		newDebtName = null;
		newDebtAmount = null;
	};
	const removeDebt = (name) => {
		let debts = { ...transaction.debts };
		delete debts[name];
		transaction.debts = debts;
	};

	let newTagName = null;
	let newTagValue = null;
	const addTag = () => {
		if (newTagValue === null || newTagValue === '' || newTagName === null || newTagName === '') {
			return;
		}
		transaction.tags[newTagName] = newTagValue;
		newTagName = null;
		newTagValue = null;
	};
	const removeTag = (name) => {
		let tags = { ...transaction.tags };
		delete tags[name];
		transaction.tags = tags;
	};

	const updateTransaction = () => {
		let t = transaction

		if (t.type !== "out") {
			t.budgets = {}
		}
		if (t.type !== "in") {
			t.inbudgets = {}
		}

		t.date = new Date(t.date)

		console.log(t)
	}
</script>

<div class="main">
	{#if transaction}
		<table>
			<tr>
				<td colspan="3" class="header">General</td>
			</tr>
			<tr
				><td>Type</td><td
					><select name="type" id="type" bind:value={transaction.type}>
						<option value="in">In</option>
						<option value="out">Out</option>
						<option value="move">Move</option>
					</select></td
				></tr
			>
			<tr><td>Date</td><td><input type="date" bind:value={transaction.date} /></td></tr>
			<tr
				><td>Amount</td><td
					><input
						type="number"
						min="0"
						oninput="validity.valid||(value='');"
						bind:value={transaction.amount}
					/></td
				></tr
			>
			{#if transaction.type === 'out' || transaction.type === 'move'}
				<tr>
					<td>Sender</td>
					<td>
						<input list="sender" bind:value={transaction.sender} />
						<datalist id="sender">
							{#if pods}
								{#each pods as pod, idx}
									<option value={pod} />
								{/each}
							{/if}
						</datalist>
					</td>
				</tr>
			{/if}
			{#if transaction.type === 'in' || transaction.type === 'move'}
				<tr>
					<td>Receiver</td>
					<td>
						<input list="receiver" bind:value={transaction.receiver} />
						<datalist id="receiver">
							{#if pods}
								{#each pods as pod, idx}
									<option value={pod} />
								{/each}
							{/if}
						</datalist>
					</td>
				</tr>
			{/if}
			<!-- Budgets -->
			{#if transaction.type === 'out'}
				<tr>
					<td colspan="3">&nbsp;</td>
				</tr>
				<tr>
					<td colspan="3" class="header">Budgets</td>
				</tr>
				{#each Object.keys(transaction.budgets) as budget, idx}
					<tr>
						<td>{budget}</td>
						<td>{transaction.budgets[budget]}</td>
						<td><span on:click={removeBudget(budget)} class="material-icons-sharp">delete</span></td>
					</tr>
				{/each}
				<tr>
					<td>
						<input list="budget" bind:value={newBudgetName} placeholder="name" />
						<datalist id="budget">
							{#if budgets}
								{#each budgets as budget, idx}
									<option value={budget} />
								{/each}
							{/if}
						</datalist>
					</td>
					<td>
						<input
							type="number"
							bind:value={newBudgetAmount}
							placeholder="amount"
							min="0"
							oninput="validity.valid||(value='');"
						/>
					</td>
					<td><span on:click={addBudget} class="material-icons-sharp">add</span></td>
				</tr>
			{/if}
			<!-- Inbudgets -->
			{#if transaction.type === 'in'}
				<tr>
					<td colspan="3">&nbsp;</td>
				</tr>
				<tr>
					<td colspan="3" class="header">Inbudgets</td>
				</tr>
				{#each Object.keys(transaction.inbudgets) as inbudget, idx}
					<tr>
						<td>{inbudget}</td>
						<td>{transaction.inbudgets[inbudget]}</td>
						<td><span on:click={removeInbudget(inbudget)} class="material-icons-sharp">delete</span></td>
					</tr>
				{/each}
				<tr>
					<td>
						<input list="inbudget" bind:value={newInbudgetName} placeholder="name" />
						<datalist id="inbudget">
							{#if inbudgets}
								{#each inbudgets as inbudget, idx}
									<option value={inbudget} />
								{/each}
							{/if}
						</datalist>
					</td>
					<td>
						<input
							type="number"
							bind:value={newInbudgetAmount}
							placeholder="amount"
							min="0"
							oninput="validity.valid||(value='');"
						/>
					</td>
					<td><span on:click={addInbudget} class="material-icons-sharp">add</span></td>
				</tr>
			{/if}
			<!-- Debts -->
			{#if transaction.type === 'out' || transaction.type === 'in'}
				<tr>
					<td colspan="3">&nbsp;</td>
				</tr>
				<tr>
					<td colspan="3" class="header">Debts</td>
				</tr>
				{#each Object.keys(transaction.debts) as debt, idx}
					<tr>
						<td>{debt}</td>
						<td>{transaction.debts[debt]}</td>
						<td><span on:click={removeDebt(debt)} class="material-icons-sharp">delete</span></td>
					</tr>
				{/each}
				<tr>
					<td>
						<input list="debt" bind:value={newDebtName} placeholder="name" />
						<datalist id="debt">
							{#if debts}
								{#each debts as debt, idx}
									<option value={debt} />
								{/each}
							{/if}
						</datalist>
					</td>
					<td>
						<input
							type="number"
							bind:value={newDebtAmount}
							placeholder="amount"
							min="0"
							oninput="validity.valid||(value='');"
						/>
					</td>
					<td><span on:click={addDebt} class="material-icons-sharp">add</span></td>
				</tr>
			{/if}
			<!-- Tags -->
			<tr>
				<td colspan="3">&nbsp;</td>
			</tr>
			<tr>
				<td colspan="3" class="header">Tags</td>
			</tr>
			{#each Object.keys(transaction.tags) as tag, idx}
				<tr>
					<td>{tag}</td>
					<td>{transaction.tags[tag]}</td>
					<td><span on:click={removeTag(tag)} class="material-icons-sharp">delete</span></td>
				</tr>
			{/each}
			<tr>
				<td>
					<input list="tag" bind:value={newTagName} placeholder="name" />
					<datalist id="tag">
						{#if tags}
							{#each tags as tag, idx}
								<option value={tag} />
							{/each}
						{/if}
					</datalist>
				</td>
				<td>
					<input type="text" bind:value={newTagValue} placeholder="value" />
				</td>
				<td><span on:click={addTag} class="material-icons-sharp">add</span></td>
			</tr>
		</table>
	{/if}
	<tr>
		<td colspan=3>
			<div class="btn" on:click={updateTransaction}>Update</div>
		</td>
	</tr>
</div>

<style>
	.header {
		font-weight: bold;
		text-align: center;
	}
	input, select {
		background-color: var(--fg1);
		color: var(--bg2);
	}
	.btn {
		background-color: var(--fg1);
		color: var(--bg2);
		padding: 0.2rem;
		border-radius: 5%;
		font-weight: bold;
	}
	.btn:hover {
		background-color: var(--bg1);
		color: var(--fg1);
	}
</style>
