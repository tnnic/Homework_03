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
    }
}
pub mod slice_into_quarters {
    pub fn split_slice_into_quarters<T>(slice: &[T]) -> Vec<&[T]> {
        let len = slice.len();
        let quater = (len + 3) / 4;
        let mut result = Vec::new();
        for i in 0..4 {
            let start = i * quater;
            let end = ((i + 1) * quater).min(len);
            result.push(&slice[start..end]);
        }
        result
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
    }
}
