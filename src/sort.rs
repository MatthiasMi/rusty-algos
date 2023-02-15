#![allow(non_snake_case)]
/// `insertion_sort` sorts the list partly `L[0..p]` by swapping
/// elements from unsorted parts to current correct position `p`.
pub(crate) fn insertion_sort<T: Ord>(list: &mut [T]) {
    for p in 1..list.len() {
        let mut j = p;
        while j > 0 && list[j] < list[j - 1] {
            list.swap(j, j - 1);
            j -= 1;
        }
    }
}


/// `merge_sort` divides in the middle, conquers the left list
/// then the right list, and merges the result into a new list.
pub(crate) fn merge_sort<T: Ord + Copy>(list: &[T]) -> Vec<T> {
    if list.len() < 2 { return list.to_vec(); }
    let mid = list.len() / 2;

    let mut L = merge_sort(&list[..mid]);
    let mut R = merge_sort(&list[mid..]);
    merge(&mut L, &mut R)
}

fn merge<T: Ord>(L: &mut Vec<T>, R: &mut Vec<T>) -> Vec<T> {
    let mut sorted = Vec::new();
    for _ in 0..L.len() + R.len() {
        if L.is_empty() {
            sorted.append(R);
            break;
        } else if R.is_empty() {
            sorted.append(L);
            break;
        } else if L[0] <= R[0] {
            sorted.push(L.remove(0));
        } else {
            sorted.push(R.remove(0));
        }
    }
    sorted
}

/// `quick_sort` follows the "divide-and-conquer"-strategy.
/// It picks a pivot element `p` which splits the list in two.
/// Then put p at its correct position in `L`, swap all
/// smaller elements before, and all those greater after `p`.
pub(crate) fn quick_sort<T: Ord>(list: &mut [T]) {
    if list.len() < 2 { return; }  //  Leave recursion

    let (p, L) = list.split_first_mut().unwrap();
    let (mut left, mut right) = (0, L.len() - 1);
    while left <= right {
        if &L[left] <= p {
            left += 1;
        } else if &L[right] > p {
            if right == 0 { break; }
            right -= 1;
        } else {
            L.swap(left, right);
            left += 1;
            if right == 0 { break; }
            right -= 1;
        }
    }
    list.swap(0, left);
    // Divide `list` and conquer both parts
    let (left_list, right_list) = list.split_at_mut(left);
    quick_sort(left_list);
    quick_sort(&mut right_list[1..]);
}

#[cfg(test)]
mod tests {
    use super::{insertion_sort, merge_sort, quick_sort};

    #[test]
    fn sorting() {
        let unsorted = [3, 1, 2, 0usize];
        let sorted = [0usize, 1, 2, 3];

        let mut list = unsorted.clone();
        insertion_sort(&mut list);
        assert_eq!(&sorted, &list);
        let mut list = unsorted.clone();
        let list = &merge_sort(&mut list)[..];  // as slice
        assert_eq!(&sorted, &list);
        let mut list = unsorted.clone();
        quick_sort(&mut list);
        assert_eq!(&sorted, &list);


        let descending = [4, 3, 2, 1, 0usize];
        let mut sorted = descending.clone();
        sorted.sort();
        let mut list = descending.clone();
        insertion_sort(&mut list);
        assert_eq!(&sorted, &list);
        let mut list = descending.clone();
        let list = &merge_sort(&mut list)[..];  // as slice
        assert_eq!(&sorted, &list);
        let mut list = descending.clone();
        quick_sort(&mut list);
        assert_eq!(&sorted, &list);


        let repeated = [0usize, 3, 1, 0, 2];
        let mut sorted = repeated.clone();
        sorted.sort();
        let mut list = repeated.clone();
        insertion_sort(&mut list);
        assert_eq!(&sorted, &list);
        let mut list = repeated.clone();
        let list = &merge_sort(&mut list)[..];  // as slice
        assert_eq!(&sorted, &list);
        let mut list = repeated.clone();
        quick_sort(&mut list);
        assert_eq!(&sorted, &list);
    }
}
