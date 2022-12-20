export default (budget, transactions) =>
	transactions
		.filter((transaction) => transaction.inbudget[budget])
		.map((transaction) => transaction.inbudget[budget])
		.reduce((a, b) => a + b, 0);
