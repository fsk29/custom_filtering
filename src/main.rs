struct FilterCondition<T> {
    field: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.field == item
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone + PartialEq,
{
    collection
        .iter()
        .filter(|item| condition.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    // Create a collection with some elements
    let collection = vec![1, 2, 3, 4, 5];

    // Initialize a FilterCondition object
    let filter_condition = FilterCondition { field: 3 };

    // Call custom_filter with the collection and filter condition
    let filtered_result = custom_filter(&collection, &filter_condition);

    // Print the filtered result
    println!("Filtered Result: {:?}", filtered_result);
}

