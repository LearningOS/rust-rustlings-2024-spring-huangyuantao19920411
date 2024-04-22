/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    //TODO
    if array.len() <= 1 {
        return;
    }
    let pivot_index = partition(array);
    sort(&mut array[0..pivot_index]);
    sort(&mut array[pivot_index..]);
}

fn partition<T: std::cmp::PartialOrd>(array: &mut [T]) -> usize {
    let pivot_index = array.len() / 2;
    let last_index = array.len() - 1;
    array.swap(pivot_index, last_index);
    let mut i = 0;
    for j in 0..last_index {
        if array[j] <= array[last_index] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, last_index);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
