pub struct PairIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    inner: I,
    cache: Option<T>,
}

impl<I, T> PairIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    pub fn new(mut inner: I) -> Self {
        let cache = inner.next();
        Self { inner, cache }
    }
}

impl<I, T> Iterator for PairIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (T, Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.cache.take()?;
        let next = self.inner.next();
        self.cache = next.clone();
        Some((current, next))
    }
}

pub trait IntoPairIterator<T>
where
    Self: Iterator<Item = T> + Sized,
    T: Clone,
{
    fn pairs(self) -> PairIterator<Self, T> {
        PairIterator::new(self)
    }
}

impl<I, T> IntoPairIterator<T> for I
where
    I: Iterator<Item = T> + Sized,
    T: Clone,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_iterator_vec() {
        assert_eq!(
            PairIterator::new(vec![0, 1, 2, 3, 4].into_iter())
                .collect::<Vec<(usize, Option<usize>)>>(),
            vec![
                (0, Some(1)),
                (1, Some(2)),
                (2, Some(3)),
                (3, Some(4)),
                (4, None),
            ]
        );
    }

    #[test]
    fn pair_iterator_empty() {
        assert_eq!(
            PairIterator::new(vec![].into_iter()).collect::<Vec<(usize, Option<usize>)>>(),
            vec![]
        );
    }
}
