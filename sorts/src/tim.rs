mod tests {
    #[test]
    fn run_tests() {
        let mut input = [3u32, 2, 1, 4, 26, 17].to_vec();
        let output = [1u32, 2, 3, 4, 17, 26].to_vec();
        super::sort(&mut input);
        assert_eq!(input, output)
    }
}

pub fn sort<T: Ord>(arr: &mut Vec<T>) {
    tim_sort(arr)
}

fn tim_sort<T: Ord>(_arr: &mut Vec<T>) {}
