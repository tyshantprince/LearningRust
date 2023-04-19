
pub struct Sorter{
    pub sortName: String,
}

impl Sorter{


pub fn insertion_sort(arr: &mut Vec<isize>){
    for i in 1..arr.len(){
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1]{
            arr.swap(j, j-1);
            j = j-1;
        }
    }
}
}