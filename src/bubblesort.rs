use super::Sorted;

pub struct Bubblesort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // TODO
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[j] {
                    slice.swap(i, j + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    super::Sort::<_, Bubblesort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
