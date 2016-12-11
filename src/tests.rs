extern crate rand;

use sorting::quicksort;
use sorting::bubblesort;
use sorting::mergesort;
use tests::rand::Rng;

fn base_case_test<F>(sort: F)
where F: Fn(&mut [i32]) {
    // zero element list
    let mut z = [];
    sort(&mut z);
    assert_eq!(z, []);

    // one element list
    let mut o = [0];
    sort(&mut o);
    assert_eq!(o, [0]);

    // single rearrangement
    let mut t = [1, 0];
    sort(&mut t);
    assert_eq!(t, [0, 1]);
}

fn simple_test<F>(sort: F)
where F: Fn(&mut [i32]) {
    let mut b = vec![1, 5, 6, 4, 3, 7, 2, 0];
    sort(&mut b);
    assert_eq!(b, (0..8).collect::<Vec<i32>>());
}

fn random_test<F>(sort: F)
where F: Fn(&mut [i32]) {
    let mut b = (0..999).collect::<Vec<i32>>();
    rand::thread_rng().shuffle(&mut b);
    sort(&mut b);
    assert_eq!(b, (0..999).collect::<Vec<i32>>());
}

fn bad_case_test<F>(sort: F)
where F: Fn(&mut [i32]) {
    let mut b = (0..99).collect::<Vec<i32>>();
    b.reverse();
    sort(&mut b);
    assert_eq!(b, (0..99).collect::<Vec<i32>>());
}

macro_rules! test {
    ($name:ident, $tester:ident, $fun:ident) => {
        #[test]
        fn $name() {
            $tester($fun);
        }
    }
}

// test quicksort
test!(quicksort_base_case, base_case_test, quicksort);
test!(quicksort_simple, simple_test, quicksort);
test!(quicksort_random, random_test, quicksort);
test!(quicksort_bad_case, bad_case_test, quicksort);

// test bubblesort
test!(bubblesort_base_case, base_case_test, bubblesort);
test!(bubblesort_simple, base_case_test, bubblesort);
test!(bubblesort_random, random_test, bubblesort);
test!(bubblesort_bad_case, bad_case_test, bubblesort);

// test mergesort
test!(mergesort_base_case, base_case_test, mergesort);
test!(mergesort_simple, base_case_test, mergesort);
test!(mergesort_random, random_test, mergesort);
test!(mergesort_bad_case, bad_case_test, mergesort);
