use super::Sorter;

pub struct MergeSort;

fn mergesort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    mergesort(&mut slice[..mid]);
    mergesort(&mut slice[mid..]);

    let mut left_idx = 0;
    let mut right_idx = mid;

    while left_idx <= mid && right_idx < len {
        if slice[left_idx] <= slice[right_idx] {
            left_idx += 1;
        } else {
            let mut current_idx = right_idx;

            while current_idx > left_idx {
                slice.swap(current_idx, current_idx - 1);
                current_idx -= 1;
            }

            left_idx += 1;
            right_idx += 1;
        }
    }
}

impl<T> Sorter<T> for MergeSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        mergesort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
