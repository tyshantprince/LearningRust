

fn main() {
    let mut v = vec![2, 34, 4, 19, -5, 22, 99];

    insertion_sort(&mut v); 
    
    println!("{:?}", v);

    let mut arr = [5, 4, 3, 2, 1];
    merge_sort(&mut arr);
    println!("{:?}", arr);

}

pub fn insertion_sort(arr: &mut Vec<isize>){
    for i in 1..arr.len(){
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1]{
            arr.swap(j, j-1);
            j = j-1;
        }
    }
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        merge_sort(left.as_mut());
        merge_sort(right.as_mut());
        merge(left.as_mut(), right.as_mut(),arr); // we want to write back to arr
    }
}

fn merge(left: &mut [i32], right: &mut [i32], result: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}