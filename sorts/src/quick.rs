mod tests {
    #[test]
    fn run_tests() {
        let mut input = [3u32, 2, 1, 4, 26, 17];
        let output = [1u32, 2, 3, 4, 17, 26];

        super::sort(&mut input);

        assert_eq!(input[..], output[..])
    }
}

pub fn sort<T: Ord>(arr: &mut [T]) {
    quick_sort(arr, 0, arr.len() - 1);
}

fn quick_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if arr.len() == 0 || arr.len() == 1 {
        return;
    }

    if start >= end {
        return;
    }

    let mut pos = start;
    for i in start..end {
        if arr[i] < arr[end] {
            arr.swap(i, pos);
            pos += 1;
        }
    }
    arr.swap(end, pos);

    // Gaurding against negative usize value.
    if pos >= 1 {
        quick_sort(arr, start, pos - 1);
    }
    quick_sort(arr, pos, end);
}
