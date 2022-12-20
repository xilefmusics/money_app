import podNameToPodType from './podNameToPodType';

export default (transactions, pods, podType) =>
	transactions
		.map((transaction) => {
			const senderType = podNameToPodType(transaction.sender, pods);
			const receiverType = podNameToPodType(transaction.receiver, pods);
			let result = 0;
			if (senderType === podType) {
				result -= transaction.amount;
			}
			if (receiverType === podType) {
				result += transaction.amount;
			}
			return result;
		})
		.reduce((a, b) => a + b, 0);
