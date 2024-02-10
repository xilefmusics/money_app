pub struct ClusterIterator<I, T, K, J, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> K,
    J: Iterator<Item = K>,
    K: PartialOrd,
{
    inner: I,
    tokey: F,
    keys: J,
    cache: Option<T>,
}

impl<I, T, K, J, F> ClusterIterator<I, T, K, J, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> K,
    J: Iterator<Item = K>,
    K: PartialOrd,
{
    pub fn new(inner: I, keys: J, tokey: F) -> Self {
        Self {
            inner,
            tokey,
            keys,
            cache: None,
        }
    }

    pub fn next_with_key(&mut self, key: K) -> Option<(Option<K>, Vec<T>)> {
        let mut cluster = if let Some(cache) = self.cache.take() {
            vec![cache]
        } else {
            vec![]
        };
        while let Some(item) = self.inner.next() {
            if (self.tokey)(&item) >= key {
                self.cache = Some(item);
                break;
            }
            cluster.push(item);
        }

        Some((Some(key), cluster))
    }

    pub fn next_without_key(&mut self) -> Option<(Option<K>, Vec<T>)> {
        let mut cluster = if let Some(cache) = self.cache.take() {
            vec![cache]
        } else {
            vec![]
        };
        while let Some(item) = self.inner.next() {
            cluster.push(item);
        }

        if cluster.len() > 0 {
            Some((None, cluster))
        } else {
            None
        }
    }
}

impl<I, T, K, J, F> Iterator for ClusterIterator<I, T, K, J, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> K,
    J: Iterator<Item = K>,
    K: PartialOrd + Clone,
{
    type Item = (Option<K>, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(key) = self.keys.next() {
            self.next_with_key(key)
        } else {
            self.next_without_key()
        }
    }
}

pub trait IntoClusterIterator<T, K, J, F>
where
    Self: Iterator<Item = T> + Sized,
    F: Fn(&T) -> K,
    J: Iterator<Item = K>,
    K: PartialOrd + Clone,
{
    fn cluster(self, keys: J, tokey: F) -> ClusterIterator<Self, T, K, J, F> {
        ClusterIterator::new(self, keys, tokey)
    }
}

impl<T, K, J, F, I> IntoClusterIterator<T, K, J, F> for I
where
    I: Iterator<Item = T> + Sized,
    F: Fn(&T) -> K,
    J: Iterator<Item = K>,
    K: PartialOrd + Clone,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_iterator() {
        assert_eq!(
            ClusterIterator::new(vec![1, 2, 3, 4].into_iter(), vec![2, 3].into_iter(), |n| *n)
                .collect::<Vec<(Option<usize>, Vec<usize>)>>(),
            vec![(Some(2), vec![1]), (Some(3), vec![2]), (None, vec![3, 4])]
        );
    }

    #[test]
    fn cluster_iterator_empty_data() {
        assert_eq!(
            ClusterIterator::new(vec![].into_iter(), vec![42].into_iter(), |n| *n)
                .collect::<Vec<(Option<usize>, Vec<usize>)>>(),
            vec![(Some(42), Vec::<usize>::default())]
        );
    }

    #[test]
    fn cluster_iterator2() {
        assert_eq!(
            ClusterIterator::new(vec![1].into_iter(), vec![3].into_iter(), |n| *n)
                .collect::<Vec<(Option<usize>, Vec<usize>)>>(),
            vec![(Some(3), vec![1])]
        );
    }
}
