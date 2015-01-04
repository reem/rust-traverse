use {Traversal, FromTraversal};

impl<T> FromTraversal<T> for Vec<T> {
    fn collect<I: Traversal<Item=T>>(iter: I) -> Vec<T> {
        let mut vec = Vec::new();
        iter.run(|&mut: elem| vec.push(elem));
        vec
    }
}

