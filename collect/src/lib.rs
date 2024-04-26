pub fn bubble_sort(vec: &mut Vec<i32>) {
    let len = vec.len();
    for _ in 0..len {
        let mut swapped = false;
        for j in 0..len - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = true;
            }
        }
        // If no two elements were swapped by inner loop, then the list is sorted.
        if !swapped {
            break;
        }
    }
}
