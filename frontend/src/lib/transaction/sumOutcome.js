export default (transactions) =>
	transactions
		.filter((transaction) => transaction.type === 'out')
		.map((transaction) => transaction.amount)
		.reduce((a, b) => a + b, 0);
