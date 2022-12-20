const getCurrentPodAmount = (pod, transactions) =>
	transactions
		.map((transaction) =>
			transaction.receiver === pod
				? transaction.amount
				: transaction.sender === pod
				? -transaction.amount
				: 0
		)
		.reduce((a, b) => a + b, 0);

export default async (filter, transactions, fetch) => {
	let pods = await (await fetch(`/pods.json`)).json();

	pods.forEach((pod) => (pod.amount = getCurrentPodAmount(pod.name, transactions)));

	if (filter.type) {
		pods = pods.filter((pod) => pod.type === filter.type);
	}

	return pods;
};
