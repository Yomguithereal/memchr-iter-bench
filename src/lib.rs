use memchr::arch::x86_64::sse2::memchr::One as SSE2One;

pub fn scalar_baseline(needle: u8, haystack: &[u8]) -> usize {
    let mut count: usize = 0;

    for byte in haystack {
        if *byte == needle {
            count += 1;
        }
    }

    count
}

unsafe fn memchr_sse2(needle: u8, haystack: &[u8]) -> Option<usize> {
    SSE2One::new_unchecked(needle).find(haystack)
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub fn memchr_sse2_loop(needle: u8, haystack: &[u8]) -> usize {
    let mut count: usize = 0;
    let mut i: usize = 0;

    while i < haystack.len() {
        unsafe {
            if let Some(o) = memchr_sse2(needle, &haystack[i..]) {
                count += 1;
                i += o + 1;
            } else {
                break;
            }
        }
    }

    count
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub fn memchr_sse2_loop_amortized(needle: u8, haystack: &[u8]) -> usize {
    let one = unsafe { SSE2One::new_unchecked(needle) };

    let mut count: usize = 0;
    let mut i: usize = 0;

    while i < haystack.len() {
        if let Some(o) = one.find(&haystack[i..]) {
            count += 1;
            i += o + 1;
        } else {
            break;
        }
    }

    count
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub fn memchr_sse2_iter(needle: u8, haystack: &[u8]) -> usize {
    let one = unsafe { SSE2One::new_unchecked(needle) };

    let mut count: usize = 0;

    // NOTE: not using the iterator `count` method because it is optimized
    for _ in one.iter(haystack) {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(scalar_baseline(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_loop(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_loop_amortized(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_iter(b',', b"name,surname,age"), 2);
    }
}
