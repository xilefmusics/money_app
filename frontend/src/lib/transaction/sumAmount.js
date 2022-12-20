export default (transactions) =>
	transactions
		.map((transaction) =>
			transaction.type === 'in'
				? transaction.amount
				: transaction.type === 'out'
				? -transaction.amount
				: 0
		)
		.reduce((a, b) => a + b, 0);
