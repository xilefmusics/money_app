export default (transactions) => {
	let budgets = [];
	transactions.forEach((transaction) =>
		Object.keys(transaction.inbudget).forEach(
			(budget) => !budgets.includes(budget) && budgets.push(budget)
		)
	);
	return budgets;
};
