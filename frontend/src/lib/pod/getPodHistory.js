export default (pod, transactions) => {
	let history = [];
	let historyItem = {
		date: new Date(transactions[0].date),
		amount: 0,
		in: 0,
		out: 0,
		change: 0
	};
	historyItem.date = new Date(historyItem.date.getFullYear(), historyItem.date.getMonth(), 1);

	transactions.forEach((transaction) => {
		transaction.date = new Date(transaction.date);

		if (historyItem.date < transaction.date) {
			historyItem.change = historyItem.in - historyItem.out;
			history.push(structuredClone(historyItem));
			historyItem.date.setMonth(historyItem.date.getMonth() + 1);
			historyItem.in = 0;
			historyItem.out = 0;
		}

		if (transaction.sender === pod) {
			historyItem.amount -= transaction.amount;
			historyItem.out += transaction.amount;
		} else if (transaction.receiver === pod) {
			historyItem.amount += transaction.amount;
			historyItem.in += transaction.amount;
		}
	});

	historyItem.change = historyItem.in - historyItem.out;
	history.push(historyItem);

	while (history[0].amount === 0) {
		history.shift();
	}
	history.reverse();
	history.forEach((item) => item.date.setMonth(item.date.getMonth() - 1));

	return history;
};
