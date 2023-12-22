use super::{Diff, TransactionsIterator, Wealth};

pub struct WealthIterator<'a> {
    transactions_iterator: TransactionsIterator<'a>,
    last: Wealth,
}

impl<'a> WealthIterator<'a> {
    pub fn new(transactions_iterator: TransactionsIterator<'a>) -> Self {
        Self {
            transactions_iterator,
            last: Wealth::default(),
        }
    }
}

impl<'a> Iterator for WealthIterator<'a> {
    type Item = Wealth;

    fn next(&mut self) -> Option<Self::Item> {
        let (transactions, date) = self.transactions_iterator.next()?;

        self.last = (transactions
            .iter()
            .map(|transaction| Wealth {
                date,
                income: transaction.income().into(),
                out: transaction.out().into(),
                change: (transaction.income() - transaction.out()).into(),
                real: (transaction.signed_amount() - transaction.signed_debt_sum()).into(),
                debt: transaction.signed_debt_sum().into(),
                sum: transaction.signed_amount().into(),
            })
            .sum::<Wealth>()
            + Wealth {
                date,
                income: 0.into(),
                out: 0.into(),
                change: 0.into(),
                real: self.last.real.clone(),
                debt: self.last.debt.clone(),
                sum: self.last.sum.clone(),
            })
        .diff(&self.last);

        Some(self.last.clone())
    }
}
