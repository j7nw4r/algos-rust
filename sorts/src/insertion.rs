mod tests {

    #[test]
    fn run_tests() {
        let mut input = [3u32, 2, 1, 4, 26, 17];
        let output = [1u32, 2, 3, 4, 17, 26];

        super::sort(&mut input);

        assert_eq!(input, output)
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    insertion_sort(arr)
}

fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }

    for x in 0..arr.len() {
        for y in (1..=x).rev() {
            if arr[y] < arr[y - 1] {
                arr.swap(y, y - 1);
            } else {
                break;
            }
        }
    }
}
