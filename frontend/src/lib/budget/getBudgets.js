export default (transactions) => {
	let budgets = [];
	transactions.forEach((transaction) =>
		Object.keys(transaction.budgets).forEach(
			(budget) => !budgets.includes(budget) && budgets.push(budget)
		)
	);
	return budgets;
};
