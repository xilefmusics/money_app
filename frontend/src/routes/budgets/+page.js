import arrayToObject from '../../lib/general/arrayToObject';
import getTransactions from '../../lib/transaction/getTransactions';
import getBudgets from '../../lib/budget/getBudgets';
import getInBudgets from '../../lib/budget/getInBudgets';
import sumBudgetAmount from '../../lib/budget/sumBudgetAmount';
import sumInBudgetAmount from '../../lib/budget/sumInBudgetAmount';
import sumIncome from '../../lib/transaction/sumIncome';
import sumOutcome from '../../lib/transaction/sumOutcome';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);
	const income = sumIncome(transactions);
	const outcome = sumOutcome(transactions);

	let budgets = getBudgets(transactions).map((budget) => ({
		name: budget,
		amount: sumBudgetAmount(budget, transactions)
	}));
	const budgetUndefined = {
		name: 'gen-undefined',
		amount: outcome - budgets.map((budget) => budget.amount).reduce((a, b) => a + b, 0)
	};
	if (budgetUndefined.amount > 0) {
		budgets.push(budgetUndefined);
	}
	const budgetSavings = {
		name: 'gen-savings',
		amount: income - budgets.map((budget) => budget.amount).reduce((a, b) => a + b, 0)
	};
	if (budgetSavings.amount > 0) {
		budgets.push(budgetSavings);
	}

	const inbudgets = getInBudgets(transactions).map((budget) => ({
		name: budget,
		amount: sumInBudgetAmount(budget, transactions)
	}));
	const inbudgetUndefined = {
		name: 'gen-undefined',
		amount: income - inbudgets.map((inbudget) => inbudget.amount).reduce((a, b) => a + b, 0)
	};
	if (inbudgetUndefined.amount > 0) {
		inbudgets.push(inbudgetUndefined);
	}

	return { budgets, inbudgets };
}
