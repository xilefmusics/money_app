import arrayToObject from '../../../lib/general/arrayToObject';
import getTransactions from '../../../lib/transaction/getTransactions';
import getPodHistory from '../../../lib/pod/getPodHistory';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);

	const history = getPodHistory(params.pod, transactions);

	return { history };
}
