const getDebts = (transactions) => {
	let debts = [];
	transactions.forEach((transaction) =>
		Object.keys(transaction.debts).forEach((debt) => !debts.includes(debt) && debts.push(debt))
	);
	return debts;
};

const getCurrentDebtAmount = (debt, transactions) =>
	transactions
		.filter((transaction) => transaction.debts[debt])
		.map((transaction) =>
			transaction.type === 'in'
				? transaction.debts[debt]
				: transaction.type === 'out'
				? -transaction.debts[debt]
				: 0
		)
		.reduce((a, b) => a + b, 0);

export default (transactions) => {
	return getDebts(transactions).map((debt) => ({
		name: debt,
		amount: getCurrentDebtAmount(debt, transactions)
	}));
};
