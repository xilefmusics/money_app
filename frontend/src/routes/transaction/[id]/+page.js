import arrayToObject from '../../../lib/general/arrayToObject';
import getTransactions from '../../../lib/transaction/getTransactions';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);

	const transaction = transactions.filter((transaction) => transaction.id == params.id)[0];

	return { transaction };
}
