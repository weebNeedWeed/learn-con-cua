pub fn find_largest(list: &[i32]) -> i32 {
    let largest = &list[0];
    let mut largest = *largest;

    for &v in list {
        if v > largest {
            largest = v;
        }
    }

    largest
}
