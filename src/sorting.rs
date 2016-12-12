pub fn quicksort<T: Ord>(b: &mut [T]) {
    if b.len() > 1 {
        let pivot = partition(b);
        let (init, rest) = b.split_at_mut(pivot);
        let tail = &mut rest[1..]; // skip the pivot
        quicksort(init);
        quicksort(tail);
    }
}

fn partition<T: Ord>(b: &mut [T]) -> usize {
    assert!(b.len() >= 1);
    let mut pivot = 0;
    let mut hi    = b.len() - 1;
    for _ in 0 .. b.len() - 1 {
        if b[pivot] < b[pivot + 1] {
            b.swap(pivot + 1, hi);
            hi -= 1;
        } else {
            b.swap(pivot, pivot + 1);
            pivot += 1;
        }
    }
    pivot
}

pub fn bubblesort<T: Ord>(b: &mut [T]) {
    for i in 0..b.len() {
        for j in i..b.len() {
            if b[i] > b[j] {
                b.swap(i, j);
            }
        }
    }
}

pub fn mergesort<T: Ord + Clone>(b: &mut [T]) {
    let length = b.len();
    if length > 1 {
        let v: Vec<T>;
        {
            let (init, tail) = b.split_at_mut(length / 2);
            mergesort(init);
            mergesort(tail);
            v = merge(init, tail);
        }
        b.clone_from_slice(&v[..]);
    }
}

fn merge<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut res = vec![];
    let mut xs = a.iter();
    let mut ys = b.iter().peekable();

    while let Some(x) = xs.next() {
        loop {
            if let Some(y) = ys.peek() { if x < y { break; } }
            else { break; }
            res.push(ys.next()
                     .expect("Shouldn't ever happen!")
                     .clone());
        }
        res.push(x.clone());
    }

    while let Some(y) = ys.next() {
        res.push(y.clone());
    }
    res
}
