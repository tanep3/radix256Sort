const BIT_WIDTH: usize = 8;
const BUCKETS: usize = 1 << BIT_WIDTH; // 256

/// Sorts a `Vec<u32>` using Radix Sort (base 256).
///
/// This function consumes the input vector and returns a sorted vector.
/// It allocates a temporary buffer of the same size as the input.
///
/// # Examples
///
/// ```
/// use radix256_sort::radix256_sort_vec;
///
/// let v = vec![5, 2, 9, 1, 5, 6];
/// let sorted = radix256_sort_vec(v);
/// assert_eq!(sorted, vec![1, 2, 5, 5, 6, 9]);
/// ```
pub fn radix256_sort_vec(mut list: Vec<u32>) -> Vec<u32> {
    let len = list.len();
    if len <= 1 {
        return list;
    }

    let mut buffer = vec![0u32; len];
    let mut from = &mut list;
    let mut to = &mut buffer;

    for shift in 0..(32 / BIT_WIDTH) {
        let mut counts = [0usize; BUCKETS];

        // Count
        for &num in from.iter() {
            let byte = ((num >> (shift * BIT_WIDTH)) & ((BUCKETS - 1) as u32)) as usize;
            counts[byte] += 1;
        }

        // Accumulate
        let mut positions = [0usize; BUCKETS];
        for i in 1..BUCKETS {
            positions[i] = positions[i - 1] + counts[i - 1];
        }

        // Reorder
        for &num in from.iter() {
            let byte = ((num >> (shift * BIT_WIDTH)) & ((BUCKETS - 1) as u32)) as usize;
            to[positions[byte]] = num;
            positions[byte] += 1;
        }

        std::mem::swap(&mut from, &mut to);
    }

    // After 4 passes (even number), the result is in the original 'list' (which 'from' points to after 4 swaps? No wait.)
    // Let's re-verify the swap logic.
    // Start: from=&list, to=&buffer.
    // Pass 0: write to buffer. swap -> from=&buffer, to=&list.
    // Pass 1: write to list. swap -> from=&list, to=&buffer.
    // Pass 2: write to buffer. swap -> from=&buffer, to=&list.
    // Pass 3: write to list. swap -> from=&list, to=&buffer.
    // End loop.
    // 'from' is now &list. 'to' is &buffer.
    // The last write was to 'list' (in Pass 3).
    // So the sorted data is in 'list'.
    
    // However, we need to return the Vec that contains the data.
    // 'list' variable owns the memory of the original vector.
    // 'buffer' variable owns the memory of the allocated buffer.
    // Since we just swapped references, the actual Vec structs are unchanged.
    // The data is physically in 'list'.
    
    list
}

/// Sorts a slice of `u32` in-place using Radix Sort (base 256).
///
/// This function allocates a temporary buffer of the same size as the input slice.
///
/// # Examples
///
/// ```
/// use radix256_sort::radix256_sort_inplace;
///
/// let mut v = [5, 2, 9, 1, 5, 6];
/// radix256_sort_inplace(&mut v);
/// assert_eq!(v, [1, 2, 5, 5, 6, 9]);
/// ```
pub fn radix256_sort_inplace(list: &mut [u32]) {
    let len = list.len();
    if len <= 1 {
        return;
    }

    let mut buffer = vec![0u32; len];
    
    // We ping-pong between list and buffer for each pass
    
    // Wait, we can't easily swap references to a slice and a vec's slice if one is borrowed from the argument.
    // We need two mutable slices.
    // 'list' is &mut [u32].
    // 'buffer' is Vec<u32>, so &mut buffer[..] is &mut [u32].
    
    // We can't swap `from` and `to` references in a way that changes what `list` points to, 
    // because `list` is a borrow given to us.
    // But we can swap *which slice we read from and write to* in the loop.
    
    // Let's use a slightly different approach for inplace to satisfy borrow checker.
    // We will copy back and forth.
    // Actually, since we know it's 4 passes, we can just be explicit or use a bool flag.
    
    // But to keep it zero-copy (except the necessary moves), we want to ping-pong.
    // Since we have `list` (A) and `buffer` (B).
    // Pass 0: A -> B
    // Pass 1: B -> A
    // Pass 2: A -> B
    // Pass 3: B -> A
    // Result is in A.
    
    // We can implement a helper that takes source and dest.
    
    for shift in 0..(32 / BIT_WIDTH) {
        let (src, dst) = if shift % 2 == 0 {
            (&*list, &mut *buffer)
        } else {
            (&*buffer, &mut *list)
        };
        
        let mut counts = [0usize; BUCKETS];

        // Count
        for &num in src.iter() {
            let byte = ((num >> (shift * BIT_WIDTH)) & ((BUCKETS - 1) as u32)) as usize;
            counts[byte] += 1;
        }

        // Accumulate
        let mut positions = [0usize; BUCKETS];
        for i in 1..BUCKETS {
            positions[i] = positions[i - 1] + counts[i - 1];
        }

        // Reorder
        for &num in src.iter() {
            let byte = ((num >> (shift * BIT_WIDTH)) & ((BUCKETS - 1) as u32)) as usize;
            dst[positions[byte]] = num;
            positions[byte] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    use rand::prelude::*;

    #[test]
    fn test_radix256_sort_vec() {
        let mut rng = rand::thread_rng();
        let v: Vec<u32> = (0..1000).map(|_| rng.gen()).collect();
        let mut expected = v.clone();
        expected.sort();
        
        let sorted = radix256_sort_vec(v);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_radix256_sort_inplace() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u32> = (0..1000).map(|_| rng.gen()).collect();
        let mut expected = v.clone();
        expected.sort();
        
        radix256_sort_inplace(&mut v);
        assert_eq!(v, expected);
    }
    
    #[test]
    fn test_empty() {
        let v: Vec<u32> = vec![];
        let sorted = radix256_sort_vec(v);
        assert_eq!(sorted, vec![]);
        
        let mut v: Vec<u32> = vec![];
        radix256_sort_inplace(&mut v);
        assert_eq!(v, vec![]);
    }
    
    #[test]
    fn test_single() {
        let v = vec![42];
        let sorted = radix256_sort_vec(v);
        assert_eq!(sorted, vec![42]);
        
        let mut v = vec![42];
        radix256_sort_inplace(&mut v);
        assert_eq!(v, vec![42]);
    }
}
