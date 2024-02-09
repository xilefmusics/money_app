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

    pub fn next_with_key(&mut self, key: K) -> Option<Vec<T>> {
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

        Some(cluster)
    }

    pub fn next_without_key(&mut self) -> Option<Vec<T>> {
        let mut cluster = if let Some(cache) = self.cache.take() {
            vec![cache]
        } else {
            vec![]
        };
        while let Some(item) = self.inner.next() {
            cluster.push(item);
        }

        if cluster.len() > 0 {
            Some(cluster)
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
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(key) = self.keys.next() {
            self.next_with_key(key)
        } else {
            self.next_without_key()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_iterator() {
        assert_eq!(
            ClusterIterator::new(vec![1, 2, 3, 4].into_iter(), vec![2, 3].into_iter(), |n| *n)
                .collect::<Vec<Vec<usize>>>(),
            vec![vec![1], vec![2], vec![3, 4]]
        );
    }

    #[test]
    fn cluster_iterator_empty_data() {
        assert_eq!(
            ClusterIterator::new(vec![].into_iter(), vec![42].into_iter(), |n| *n)
                .collect::<Vec<Vec<usize>>>(),
            vec![vec![]]
        );
    }

    #[test]
    fn cluster_iterator2() {
        assert_eq!(
            ClusterIterator::new(vec![1].into_iter(), vec![3].into_iter(), |n| *n)
                .collect::<Vec<Vec<usize>>>(),
            vec![vec![1]]
        );
    }
}
