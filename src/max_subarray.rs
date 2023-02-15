/// `max_subarray` returns the `start`, maximum `sum` and `end` of a sliced `isize`-list.
pub(crate) fn max_subarray(list: &[isize]) -> (usize, isize, usize) {
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    let mut best = std::isize::MIN;
    let mut current = 0;

    for i in 0..list.len() {
        sum += list[i];
        if sum > best {
            best = sum;
            start = current;
            end = i;
        }
        if sum < 0 {
            sum = 0;
            current = i + 1;
        }
    }
    (start, best, end)
}

/// `test_max_subarray` runs in `O(n)`, where `n` is the length of the list.
#[test]
fn test_max_subarray() {
    let list = [1, 2, 3, -2, 5];
    assert_eq!(max_subarray(&list), (0, 9, 4));

    let list = [1, 2, 3, -2, -5];
    assert_eq!(max_subarray(&list), (0, 6, 2));

    let list = [-1, -2, -3, -4];
    assert_eq!(max_subarray(&list), (0, -1, 0));

    let list = [1, 2, -1, 2, 3];
    assert_eq!(max_subarray(&list), (0, 7, 4));
}