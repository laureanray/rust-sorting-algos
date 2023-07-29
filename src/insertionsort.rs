use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice [unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice[..unsorted]
            if !self.smart {
                let mut i = unsorted;

                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                // then use .insert to splice in i
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };

                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

#[test]
fn it_works_not_smart() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

