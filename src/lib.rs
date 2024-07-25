pub mod to_tuple_element {
    pub fn mutable_reference_to_tuple_element<T>(tuple: &mut (T, T), flag: bool) -> &mut T {
        if flag {
            &mut tuple.1
        } else {
            &mut tuple.0
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_mutable_reference_to_tuple_element() {
            let mut tuple = (1, 2);
            {
                let elem = mutable_reference_to_tuple_element(&mut tuple, false);
                *elem = 10;
            }
            assert_eq!(tuple.0, 10);
            assert_eq!(tuple.1, 2);
            {
                let elem = mutable_reference_to_tuple_element(&mut tuple, true);
                *elem = 20;
            }
            assert_eq!(tuple.0, 10);
            assert_eq!(tuple.1, 20);
        }
    }
}
pub mod slice_element {
    pub fn mutable_reference_to_slice_element<T>(slice: &mut [T], num: usize) -> &mut T {
        &mut slice[num]
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_mutable_reference_to_slice_element() {
            let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];
            {
                let elem = mutable_reference_to_slice_element(&mut slice, 3);
                *elem = 20;
            }
            assert_eq!(slice[3], 20);
        }
        #[test]
        #[should_panic]
        fn test_mutable_reference_to_slice_element_out_of_bounds() {
            let mut slice = [1, 2, 3];
            let _ = mutable_reference_to_slice_element(&mut slice, 10); // Выход за границу
        }

    }
}
pub mod element_from_end {
    pub fn slice_element_from_end<T>(slice: &[T], n: usize) -> &T {
        &slice[slice.len() - n - 1]
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_slice_element_from_end() {
            let slice = [1, 2, 3, 4, 5, 6, 7, 8];
            let elem = slice_element_from_end(&slice, 0);
            assert_eq!(*elem, 8);
        }
        #[test]
        #[should_panic]
        fn test_slice_element_from_end_out_of_bounds() {
            let slice = [1, 2, 3];
            let _ = slice_element_from_end(&slice, 4); // Выход за границу
        }

        #[test]
        fn test_slice_element_from_empty_slice() {
            let slice: [i32; 0] = [];
            let result = std::panic::catch_unwind(|| slice_element_from_end(&slice, 0));
            assert!(result.is_err()); // Ожидаем панику
        }
    }
}
pub mod split_slice {
    pub fn split_slice_at<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
        slice.split_at(n)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_split_slice_at() {
            let slice = [1, 2, 3, 4, 5];
            let (left, right) = split_slice_at(&slice, 2);
            assert_eq!(left, &[1, 2]);
            assert_eq!(right, &[3, 4, 5]);
        }
        #[test]
        fn test_split_slice_at_bounds() {
            let slice = [1, 2, 3, 4, 5];
            let (left, right) = split_slice_at(&slice, 5);
            assert_eq!(left, &[1, 2, 3, 4, 5]);
            assert_eq!(right, &[]);
        }

        #[test]
        fn test_split_empty_slice() {
            let slice: [i32; 0] = [];
            let (left, right) = split_slice_at(&slice, 0);
            assert_eq!(left, &[]);
            assert_eq!(right, &[]);
        }
    }
}
pub mod slice_into_quarters {

    // pub fn split_slice_into_quarters<T>(slice: &[T]) -> Vec<&[T]> {
    //     let len = slice.len();
    //     let quater = (len + 3) / 4;
    //     let mut result = Vec::new();
    //     for i in 0..4 {
    //         let start = i * quater;
    //         let end = ((i + 1) * quater).min(len);
    //         result.push(&slice[start..end]);
    //     }
    //     result

    pub fn split_slice_into_quarters<T>(slice: &[T]) -> Vec<&[T]> {
        let len = slice.len();
        let mut quaters = Vec::with_capacity(4);
        let mut start = 0;
        for i in 0..4 {
            let end = if i < len % 4 {
                start + (len / 4) + 1
            } else {

                start + (len / 4)
            };
            quaters.push(&slice[start..end]);
            start = end;
        }
        quaters
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_split_slice_into_quarters() {
            let slice = [1, 2, 3, 4, 5, 6, 7, 8];
            let elem_vec = split_slice_into_quarters(&slice);
            assert_eq!(elem_vec[0], &[1, 2]);
            assert_eq!(elem_vec[1], &[3, 4]);
            assert_eq!(elem_vec[2], &[5, 6]);
            assert_eq!(elem_vec[3], &[7, 8]);
        }
        #[test]
        fn test_split_slice_into_quarters_empty() {
            let slice: [i32; 0] = [];
            let quarters = split_slice_into_quarters(&slice);
            assert_eq!(quarters[0], &[]);
            assert_eq!(quarters[1], &[]);
            assert_eq!(quarters[2], &[]);
            assert_eq!(quarters[3], &[]);
        }
        #[test]
        fn test_split_slice_into_quarters_single_element() {
            let slice = [1];
            let quarters = split_slice_into_quarters(&slice);
            assert_eq!(quarters[0], &[1]);
            assert_eq!(quarters[1], &[]);
            assert_eq!(quarters[2], &[]);
            assert_eq!(quarters[3], &[]);
        }
        #[test]
        fn test_split_slice_into_quarters_seventeen_elements() {
            let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
            let quarters = split_slice_into_quarters(&slice);
            assert_eq!(quarters[0], &[1, 2, 3, 4, 5]);
            assert_eq!(quarters[1], &[6, 7, 8, 9]);
            assert_eq!(quarters[2], &[10, 11, 12, 13]);
            assert_eq!(quarters[3], &[14, 15, 16, 17]);
        }
        #[test]
        fn test_split_slice_into_quarters_various_lengths() {
            let slice_5 = [1, 2, 3, 4, 5];
            let quarters_5 = split_slice_into_quarters(&slice_5);
            assert_eq!(quarters_5[0], &[1, 2]);
            assert_eq!(quarters_5[1], &[3]);
            assert_eq!(quarters_5[2], &[4]);
            assert_eq!(quarters_5[3], &[5]);

            let slice_6 = [1, 2, 3, 4, 5, 6];
            let quarters_6 = split_slice_into_quarters(&slice_6);
            assert_eq!(quarters_6[0], &[1, 2]);
            assert_eq!(quarters_6[1], &[3, 4]);
            assert_eq!(quarters_6[2], &[5]);
            assert_eq!(quarters_6[3], &[6]);

            let slice_7 = [1, 2, 3, 4, 5, 6, 7];
            let quarters_7 = split_slice_into_quarters(&slice_7);
            assert_eq!(quarters_7[0], &[1, 2]);
            assert_eq!(quarters_7[1], &[3, 4]);
            assert_eq!(quarters_7[2], &[5, 6]);
            assert_eq!(quarters_7[3], &[7]);
            
            let slice_8 = [1, 2, 3, 4, 5, 6, 7, 8];
            let quarters_8 = split_slice_into_quarters(&slice_8);
            assert_eq!(quarters_8[0], &[1, 2]);
            assert_eq!(quarters_8[1], &[3, 4]);
            assert_eq!(quarters_8[2], &[5, 6]);
            assert_eq!(quarters_8[3], &[7, 8]);
        }
    }
}
