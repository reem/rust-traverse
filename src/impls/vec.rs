use std::vec;
use {Traversal, IntoTraversal, FromTraversal, Internal};

impl<T> IntoTraversal for Vec<T> {
    type IntoTrav = Internal<vec::IntoIter<T>>;
    type Item = T;

    fn into_traversal(self) -> Self::IntoTrav {
        Internal::new(self.into_iter())
    }
}

impl<T> FromTraversal<T> for Vec<T> {
	fn from_traversal<I: IntoTraversal<Item=T>>(traversable: I) -> Self {
		let trav = traversable.into_traversal();
		let mut new = Self::with_capacity(trav.size_hint().0);
		trav.run(|elem| {
			new.push(elem);
		});
		new
	}
}

#[cfg(test)]
mod test {
    use {Traversal, IntoTraversal};

    #[test]
    fn test_basic() {
        let data = vec![1, 2, 5, 4, 6, 7];
        let traversal: Vec<usize> = data.clone().into_traversal().collect();
        assert_eq!(traversal, data);
    }

    #[test]
    fn test_zero_size() {
        let data = vec![(), (), ()];
        let traversal: Vec<()> = data.clone().into_traversal().collect();
        assert_eq!(traversal, data);
    }
}
