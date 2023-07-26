use super::Sorter;

pub struct InsertionSort;

// A really stupid sort
impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {

    }

}


// #[test]
// fn it_works() {
//     let mut things = vec![4, 2, 3, 1];
//     super::sort::<_, Insertionsort>(&mut things);
//     assert_eq!(things, &[1, 2, 3, 4]);
// }
