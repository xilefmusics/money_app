import saveFile from './saveFile'

export default transactions => {
    let csv = "Date,Amount,Type,Sender,Receiver,Budgets,Inbudgets,Debts,Tags\n"

	const getDate = transaction => `${transaction.date.getFullYear()}-${transaction.date.getMonth()+1}-${transaction.date.getDate()}`
	const getAmount = transaction => `${(transaction.amount/100).toFixed(2)}`
	const getArray = array => {
		let result = ""
		for (const [key, value] of Object.entries(array)) {
  			result += `\'${key}\'=\'${value}\' `
		}
		if (result.length > 0) {
			result = result.slice(0, -1)
		}
		return result
	}
	
	transactions.forEach(transaction => {
		csv += `${getDate(transaction)},${getAmount(transaction)},${transaction.type},${transaction.sender},${transaction.receiver},${getArray(transaction.budgets)},${getArray(transaction.inbudgets)},${getArray(transaction.debts)},${getArray(transaction.tags)}\n`
	});
	var blob = new Blob([csv], {type: "octet/stream"});
	saveFile(blob, "transactions.csv");
}