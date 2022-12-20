import parseDate from '../general/parseDate';

export default async (filter, fetch) => {
	let transactions = await (await fetch(`/transactions.json`)).json();

	transactions.forEach((transaction) => {
		transaction.date = new Date(transaction.date);
	});

	transactions = transactions.sort((a, b) => b.date - a.date);

	if (filter.pod) {
		transactions = transactions.filter(
			(transaction) => transaction.sender === filter.pod || transaction.receiver === filter.pod
		);
	}

	if (filter.type) {
		transactions = transactions.filter((transaction) => transaction.type === filter.type);
	}

	if (filter.startDate) {
		transactions = transactions.filter(
			(transaction) => transaction.date > parseDate(filter.startDate)
		);
	}

	if (filter.endDate) {
		transactions = transactions.filter(
			(transaction) => transaction.date < parseDate(filter.endDate)
		);
	}

	if (filter.debt) {
		transactions = transactions.filter((transaction) => transaction.debts[filter.debt]);
	}

	if (filter.budget) {
		transactions = transactions.filter((transaction) => transaction.budgets[filter.budget]);
	}

	if (filter.inbudget) {
		transactions = transactions.filter((transaction) => transaction.inbudget[filter.inbudget]);
	}

	return transactions;
};
