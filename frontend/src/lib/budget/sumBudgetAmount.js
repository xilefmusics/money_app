export default (budget, transactions) =>
	transactions
		.filter((transaction) => transaction.budgets[budget])
		.map((transaction) => transaction.budgets[budget])
		.reduce((a, b) => a + b, 0);
