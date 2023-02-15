mod data;
mod graph;
mod search;
mod sort;
mod max_subarray;

fn main() {
    println!("Demo usage of data structures and algorithms!");

    let q = data::Queue::<usize>::new();
    let g = graph::Graph::new(vec![],vec![]);
    let s = search::linear_search(&0, &vec![]);

    let mut list = vec![];
    sort::insertion_sort::<usize>(&mut list);

    println!("Queue: {:?}, Graph: {:?}, search result: {:?}, sorted list: {:?}", q, g, s, list);
}
