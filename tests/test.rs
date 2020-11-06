#[cfg(test)]
mod tests {

    use binary_search::binary_search;

    #[test]
    fn test_case_01() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 33;
       assert_eq!(binary_search(array, target), 3);
    }
    
    #[test]
    fn test_case_02() {
       let array = vec![1, 5, 23, 111];
       let target = 111;
       assert_eq!(binary_search(array, target), 3);
    }

    #[test]
    fn test_case_03() {
       let array = vec![1, 5, 23, 111];
       let target = 5;
       assert_eq!(binary_search(array, target), 1);
    }

    #[test]
    fn test_case_04() {
       let array = vec![1, 5, 23, 111];
       let target = 35;
       assert_eq!(binary_search(array, target), -1);
    }
    
    #[test]
    fn test_case_05() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 0;
       assert_eq!(binary_search(array, target), 0);
    }

    #[test]
    fn test_case_06() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 1;
       assert_eq!(binary_search(array, target), 1);
    }

    #[test]
    fn test_case_07() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 21;
       assert_eq!(binary_search(array, target), 2);
    }

    #[test]
    fn test_case_08() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 45;
       assert_eq!(binary_search(array, target), 4);
    }

    #[test]
    fn test_case_09() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 61;
       assert_eq!(binary_search(array, target), 6);
    }

    #[test]
    fn test_case_10() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 71;
       assert_eq!(binary_search(array, target), 7);
    }

    #[test]
    fn test_case_11() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 72;
       assert_eq!(binary_search(array, target), 8);
    }

    #[test]
    fn test_case_12() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 73;
       assert_eq!(binary_search(array, target), 9);
    }

    #[test]
    fn test_case_13() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
       let target = 70;
       assert_eq!(binary_search(array, target), -1);
    }

    #[test]
    fn test_case_14() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73, 355];
       let target = 355;
       assert_eq!(binary_search(array, target), 10);
    }

    #[test]
    fn test_case_15() {
       let array = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73, 355];
       let target = 354;
       assert_eq!(binary_search(array, target), -1);
    }
    
    #[test]
    fn test_case_16() {
       let array = vec![1, 5, 23, 111];
       let target = 120;
       assert_eq!(binary_search(array, target), -1);
    }

    #[test]
    fn test_case_17() {
       let array = vec![5, 23, 111];
       let target = 3;
       assert_eq!(binary_search(array, target), -1);
    }
}
