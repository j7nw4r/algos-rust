mod tests {
    #[test]
    fn run_tests() {
        let input = [3u32, 2, 1, 4, 26, 17];
        let output = [1u32, 2, 3, 4, 17, 26];
        let result = super::sort(input.to_vec());
        assert_eq!(result[..], output[..])
    }
}

pub fn sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    merge_sort(arr)
}

fn merge_sort<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let l = merge_sort(arr[..mid].to_vec());
    let r = merge_sort(arr[mid..].to_vec());
    let result = merge(l, r);

    return result;
}

fn merge<T: Ord + Copy>(mut l: Vec<T>, mut r: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    while l.len() > 0 && r.len() > 0 {
        if r.first().unwrap() > l.first().unwrap() {
            let elem = l[0];
            l.remove(0);
            result.push(elem);
        } else {
            let elem = r[0];
            r.remove(0);
            result.push(elem);
        }
    }

    while l.len() > 0 {
        let elem = l[0];
        l.remove(0);
        result.push(elem);
    }

    while r.len() > 0 {
        let elem = r[0];
        r.remove(0);
        result.push(elem);
    }

    result
}
