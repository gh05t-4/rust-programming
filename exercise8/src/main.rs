struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
	    Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    // TODO: Write the rest of this implementation.
    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty then return None
        if self.inner.is_empty() {
            return None;
        }

        // Check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        Some(items)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
	    Groups::new(data.clone()).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![4],
    	    vec![1, 1],
	        vec![2],
    	    vec![1],
	        vec![3, 3],
	        vec![-2, -2, -2],
    	    vec![5, 5],
	    ]
    );
    println!("{:?}", Groups::new(data).into_iter().collect::<Vec<Vec<_>>>());

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
	    Groups::new(data2.clone()).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![1],
    	    vec![2, 2],
	        vec![1, 1],
	        vec![2, 2],
    	    vec![3],
	        vec![4, 4],
	        vec![3],
	    ]
    );
    println!("{:?}", Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>());
}