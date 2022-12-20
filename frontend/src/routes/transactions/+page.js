import arrayToObject from '../../lib/general/arrayToObject';
import getTransactions from '../../lib/transaction/getTransactions';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));

	let lastMonth = null;
	const today = new Date();
	const transactions = (await getTransactions(filter, fetch)).map((transaction) => {
		if (lastMonth !== transaction.date.getMonth()) {
			transaction.newBlock = transaction.date.toLocaleString('en-US', { month: 'long' });
			if (transaction.date.getFullYear() < today.getFullYear()) {
				transaction.newBlock = transaction.newBlock + ' ' + transaction.date.getFullYear();
			}
		}
		lastMonth = transaction.date.getMonth();
		return transaction;
	});

	return { transactions };
}
