mod data;
mod graph;
mod search;
mod sort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q() {
        let q = data::Queue::<usize>::new();
        assert!(q.mty(), "is not empty");
    }

    #[test]
    fn test_g() {
        let g = graph::Graph::new(vec![],vec![]);
        assert!(g.V.is_empty() == g.E.is_empty(), "is not empty");
    }

    #[test]
    fn test_s() {
        let s = search::linear_search(&0, &vec![]);
        assert_eq!(s, None, "is not None");
    }

    #[test]
    fn test_sort() {
        let mut list = vec![];
        sort::insertion_sort::<usize>(&mut list);        
                assert_eq!(list, vec![], "is not empty");
    }
}