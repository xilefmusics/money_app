import arrayToObject from '../../lib/general/arrayToObject';
import getTransactions from '../../lib/transaction/getTransactions';
import getDebts from '../../lib/debt/getDebts';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);
	const debts = getDebts(transactions);

	return { debts };
}
