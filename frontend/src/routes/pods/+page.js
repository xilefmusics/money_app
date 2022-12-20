import arrayToObject from '../../lib/general/arrayToObject';
import getTransactions from '../../lib/transaction/getTransactions';
import getPods from '../../lib/pod/getPods';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);
	const pods = await getPods(filter, transactions, fetch);

	return { pods };
}
