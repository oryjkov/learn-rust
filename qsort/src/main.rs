fn qsort(v: &mut [i32]) { //&mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    }

    let pivot = v[0];

    let mut smaller = 0;

    for i in 1..v.len() {
        if v[i] < pivot {
            smaller += 1;
        }
    }
    v.swap(0, smaller);
    let (mut left, mut right) = (0, smaller+1);
    while left < smaller && right < v.len() {
        while v[left] < pivot {
            left += 1;
        }
        while right < v.len() && v[right] >= pivot {
            right += 1;
        }
        if left >= smaller || right >= v.len() {
            break;
        }
        v.swap(left, right);
        left += 1;
        right += 1;
    }

    qsort(&mut v[..smaller]);

    let v_len = v.len();
    qsort(&mut v[(smaller+1).max(v_len)..]);
}

#[test]
fn qsort_test() {
    let mut v = vec![1,0,3,-9];
    qsort(&mut v);
    let sorted_v = vec![-9,0,1,3];
    assert_eq!(v, sorted_v);
}

fn main() {
    println!("Hello, world!");
}
