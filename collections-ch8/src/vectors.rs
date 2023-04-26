use std::collections::HashMap;

pub fn find_median(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut sorted = v.clone();
    sorted.sort();

    Some(sorted[v.len() / 2])
}

pub fn find_mode(v: &Vec<i32>) -> Option<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for val in v.iter() {
        let count = map.entry(*val).or_insert(0);
        *count += 1;
    }

    let max = map.iter().max_by_key(|entry| entry.1);

    match max {
        Some(max) => Some(max.0.to_owned()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_median() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(super::find_median(&v).unwrap(), 3);
    }

    #[test]
    fn test_median_with_empty() {
        let v = vec![];
        assert!(super::find_median(&v).is_none());
    }

    #[test]
    fn test_mode() {
        let v = vec![2, 2, 2, 1, 3, 3];
        assert_eq!(super::find_mode(&v).unwrap(), 2);
    }

    #[test]
    fn test_mode_with_empty() {
        let v = vec![];
        assert!(super::find_mode(&v).is_none());
    }
}
