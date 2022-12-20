import arrayToObject from '../../../lib/general/arrayToObject';
import getTransactions from '../../../lib/transaction/getTransactions';
import getPods from '../../../lib/pod/getPods';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);
	const pods = await getPods(filter, transactions, fetch);

	const pod = pods.filter((pod) => pod.name === params.pod)[0];

	return { pod };
}
