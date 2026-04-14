/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::PartialOrd;

fn sort<T: PartialOrd + Copy>(array: &mut [T]){
    let len = array.len();

    // Insertion Sort
    for i in 1..len {
        let temp = array[i];
        let mut done = false;

        for j in 1..=i {
            let index = i - j;
            if array[index] <= temp {
                array[index + 1] = temp;
                done = true;
                break;
            }
            array[index + 1] = array[index];
        }

        if !done {
            array[0] = temp;
        }
    }
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
