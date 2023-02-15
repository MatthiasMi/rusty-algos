/// `binary_search` requires `PartialEq` for checking `<`, and
///  returns index of `t` in (asc) sorted list `L = [L_1,...]`, else `None`.
pub(crate) fn binary_search<T: Ord>(t: &T, list: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len();

    while left <= right {
        let ind = (left + right) / 2; // Index is set to the mid of the search space.
        if list.len() == ind {
            return None;
        }
        if list[ind] == *t {
            return Some(ind);
        } else if list[ind] < *t {
            left = ind + 1;
        } else {
            right = ind - 1;
        }
    }
    None
}

/// `linear_search` requires `PartialEq` for checking `==` and returns 
/// the index of `t` in list, else `None`.
pub(crate) fn linear_search<T: PartialEq>(t: &T, list: &[T]) -> Option<usize> {
    list.iter().position(|x| x == t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_int_not_found() {
        assert_eq!(binary_search(&5, &vec![0, 1, 2, 3]), None);
        assert_eq!(linear_search(&5, &vec![0, 1, 2, 3]), None);
    }
    #[test]
    fn search_int_one() {
        assert_eq!(binary_search(&0, &vec![0]), Some(0));
        assert_eq!(linear_search(&0, &vec![0]), Some(0));
    }

    #[test]
    fn search_ints_asc_odd() {
        assert_eq!(binary_search(&2, &vec![0, 1, 2]), Some(2));
        assert_eq!(binary_search(&1, &vec![0, 1, 2]), Some(1));
        assert_eq!(binary_search(&0, &vec![0, 1, 2]), Some(0));
        assert_eq!(linear_search(&2, &vec![0, 1, 2]), Some(2));
        assert_eq!(linear_search(&1, &vec![0, 1, 2]), Some(1));
        assert_eq!(linear_search(&0, &vec![0, 1, 2]), Some(0));
    }

    #[test]
    fn search_ints_asc_even() {
        assert_eq!(binary_search(&3, &vec![0, 1, 2, 3]), Some(3));
        assert_eq!(binary_search(&2, &vec![0, 1, 2, 3]), Some(2));
        assert_eq!(binary_search(&1, &vec![0, 1, 2, 3]), Some(1));
        assert_eq!(binary_search(&0, &vec![0, 1, 2, 3]), Some(0));
        assert_eq!(linear_search(&3, &vec![0, 1, 2, 3]), Some(3));
        assert_eq!(linear_search(&2, &vec![0, 1, 2, 3]), Some(2));
        assert_eq!(linear_search(&1, &vec![0, 1, 2, 3]), Some(1));
        assert_eq!(linear_search(&0, &vec![0, 1, 2, 3]), Some(0));
    }
}
