import arrayToObject from '../../lib/general/arrayToObject';
import getTransactions from '../../lib/transaction/getTransactions';
import getPods from '../../lib/pod/getPods';
import getDebts from '../../lib/debt/getDebts';
import sumAmount from '../../lib/transaction/sumAmount';
import sumIncome from '../../lib/transaction/sumIncome';
import sumOutcome from '../../lib/transaction/sumOutcome';
import sumAmountForPodType from '../../lib/pod/sumAmountForPodType';
import parseDate from '../../lib/general/parseDate';

const getAmountHistory = (transactions, pods) => {
	const years = [
		...new Set(transactions.map((transaction) => transaction.date.getFullYear()))
	].sort((a, b) => a - b);

	return years.map((year) => {
		const date = '' + (year + 1);
		const tAcc = transactions.filter((transaction) => transaction.date < parseDate(date));
		const tCurrent = transactions.filter(
			(transaction) =>
				transaction.date > parseDate('' + (year + 1)) &&
				transaction.date < parseDate('' + (year + 2))
		);
		const sum = sumAmount(tAcc);
		const solvent = sumAmountForPodType(tAcc, pods, 'solvent');
		const insolvent = sumAmountForPodType(tAcc, pods, 'insolvent');
		const debts = getDebts(tAcc);
		const debt = debts.map((debt) => debt.amount).reduce((a, b) => a + b, 0);
		const real = sum - debt;
		let income = sumIncome(tCurrent);
		let outcome = sumOutcome(tCurrent);
		if (year == new Date().getFullYear()) {
			income = undefined;
			outcome = undefined;
		}
		return { date, sum, solvent, insolvent, debt, real, income, outcome };
	});
};

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url }) {
	const filter = arrayToObject(Array.from(url.searchParams.entries()));
	const transactions = await getTransactions(filter, fetch);
	const pods = await getPods(filter, transactions, fetch);

	const amountHistory = getAmountHistory(transactions, pods);

	return {
		amountHistory: amountHistory
	};
}
