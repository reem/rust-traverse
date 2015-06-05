use std::collections::*;
use std::hash::Hash;
use super::*;

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

impl<T> FromTraversal<T> for VecDeque<T> {
	fn from_traversal<I: IntoTraversal<Item=T>>(traversable: I) -> Self {
		let trav = traversable.into_traversal();
		let mut new = Self::with_capacity(trav.size_hint().0);
		trav.run(|elem| {
			new.push_back(elem);
		});
		new
	}
}

impl<T> FromTraversal<T> for LinkedList<T> {
	fn from_traversal<I: IntoTraversal<Item=T>>(traversable: I) -> Self {
		let trav = traversable.into_traversal();
		let mut new = Self::new();
		trav.run(|elem| {
			new.push_back(elem);
		});
		new
	}
}

impl<K: Hash + Eq, V> FromTraversal<(K, V)> for HashMap<K, V> {
	fn from_traversal<I: IntoTraversal<Item=(K, V)>>(traversable: I) -> Self {
		let trav = traversable.into_traversal();
		let mut new = Self::with_capacity(trav.size_hint().0);
		trav.run(|(k, v)| {
			new.insert(k, v);
		});
		new
	}
}

impl<K: Ord, V> FromTraversal<(K, V)> for BTreeMap<K, V> {
	fn from_traversal<I: IntoTraversal<Item=(K, V)>>(traversable: I) -> Self {
		let trav = traversable.into_traversal();
		let mut new = Self::new();
		trav.run(|(k, v)| {
			new.insert(k, v);
		});
		new
	}
}

