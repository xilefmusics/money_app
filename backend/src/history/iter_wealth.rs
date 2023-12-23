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

    pub fn into_shift_in_out_iter(self) -> ShiftInOutIterator<'a> {
        ShiftInOutIterator::new(self)
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

pub struct ShiftInOutIterator<'a> {
    wealth_iterator: WealthIterator<'a>,
    last: Option<Wealth>,
}

impl<'a> ShiftInOutIterator<'a> {
    pub fn new(mut wealth_iterator: WealthIterator<'a>) -> Self {
        let last = wealth_iterator.next().map(|mut last| {
            last.income = 0.into();
            last.out = 0.into();
            last.change = 0.into();
            last
        });
        Self {
            wealth_iterator,
            last,
        }
    }
}

impl<'a> Iterator for ShiftInOutIterator<'a> {
    type Item = Wealth;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.wealth_iterator.next();
        if next.is_none() {
            return self.last.take();
        }
        let mut next = next.unwrap();
        let mut result = self.last.take().unwrap();

        result.income = next.income.clone();
        next.income = 0.into();
        result.out = next.out.clone();
        next.out = 0.into();
        result.change = next.change.clone();
        next.change = 0.into();

        self.last = Some(next);
        Some(result)
    }
}
