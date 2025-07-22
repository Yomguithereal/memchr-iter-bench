use memchr::arch::x86_64::sse2::memchr::One as SSE2One;

/// A trait for adding some helper routines to pointers.
pub(crate) trait Pointer {
    /// Returns the distance, in units of `T`, between `self` and `origin`.
    ///
    /// # Safety
    ///
    /// Same as `ptr::offset_from` in addition to `self >= origin`.
    unsafe fn distance(self, origin: Self) -> usize;

    /// Casts this pointer to `usize`.
    ///
    /// Callers should not convert the `usize` back to a pointer if at all
    /// possible. (And if you believe it's necessary, open an issue to discuss
    /// why. Otherwise, it has the potential to violate pointer provenance.)
    /// The purpose of this function is just to be able to do arithmetic, i.e.,
    /// computing offsets or alignments.
    fn as_usize(self) -> usize;
}

impl<T> Pointer for *const T {
    unsafe fn distance(self, origin: *const T) -> usize {
        // TODO: Replace with `ptr::sub_ptr` once stabilized.
        usize::try_from(self.offset_from(origin)).unwrap_unchecked()
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

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

use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi8, _mm_load_si128, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8,
};

#[inline(always)]
fn get_for_offset(mask: u32) -> u32 {
    #[cfg(target_endian = "big")]
    {
        mask.swap_bytes()
    }
    #[cfg(target_endian = "little")]
    {
        mask
    }
}

#[inline(always)]
fn first_offset(mask: u32) -> usize {
    get_for_offset(mask).trailing_zeros() as usize
}

#[inline(always)]
fn clear_least_significant_bit(mask: u32) -> u32 {
    mask & (mask - 1)
}

struct OneMatches<'h> {
    start: *const u8,
    end: *const u8,
    current: *const u8,
    mask: Option<(*const u8, u32)>,
    needle: u8,
    splat: __m128i,
    haystack: core::marker::PhantomData<&'h [u8]>,
}

const BYTES: usize = 16;
const ALIGN: usize = 15;

impl<'h> OneMatches<'h> {
    unsafe fn new(needle: u8, haystack: &[u8]) -> Self {
        let ptr = haystack.as_ptr();

        Self {
            start: ptr,
            end: ptr.wrapping_add(haystack.len()),
            current: ptr,
            mask: None,
            needle,
            splat: _mm_set1_epi8(needle as i8),
            haystack: core::marker::PhantomData,
        }
    }

    unsafe fn next(&mut self) -> Option<usize> {
        'main: loop {
            // Processing current move mask
            if let Some((from, mask)) = &mut self.mask {
                debug_assert!(*mask != 0);

                let offset = from.add(first_offset(*mask));
                let next_mask = clear_least_significant_bit(*mask);

                if next_mask != 0 {
                    *mask = next_mask;
                } else {
                    self.mask = None;
                }

                return Some(offset.distance(self.start));
            }

            // Initial unaligned load
            if self.current == self.start {
                let chunk = _mm_loadu_si128(self.current as *const __m128i);
                let cmp = _mm_cmpeq_epi8(chunk, self.splat);
                let mask = _mm_movemask_epi8(cmp) as u32;

                let next = self.start.add(BYTES - (self.start.as_usize() & ALIGN));

                if mask != 0 {
                    self.mask = Some((self.start, mask));
                    self.current = next;
                    continue 'main;
                } else {
                    self.current = next;
                }
            }

            // Main loop of aligned loads
            while self.current <= self.end.sub(BYTES) {
                debug_assert_eq!(0, self.current.as_usize() % BYTES);

                let chunk = _mm_load_si128(self.current as *const __m128i);
                let cmp = _mm_cmpeq_epi8(chunk, self.splat);
                let mask = _mm_movemask_epi8(cmp) as u32;

                let next = self.current.add(BYTES);

                if mask != 0 {
                    self.mask = Some((self.current, mask));
                    self.current = next;
                    continue 'main;
                } else {
                    self.current = next;
                }
            }

            // debug_assert!(self.end.distance(self.current) < BYTES);

            // Processing remaining bytes linearly
            while self.current < self.end {
                if *self.current == self.needle {
                    let offset = self.current.distance(self.start);
                    self.current = self.current.add(1);
                    return Some(offset);
                } else {
                    self.current = self.current.add(1);
                }
            }

            return None;
        }
    }
}

struct OneMatchesIter<'h>(OneMatches<'h>);

impl<'h> OneMatchesIter<'h> {
    fn new(needle: u8, haystack: &[u8]) -> Self {
        unsafe { OneMatchesIter(OneMatches::new(needle, haystack)) }
    }
}

impl<'h> Iterator for OneMatchesIter<'h> {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { self.0.next() }
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub fn memoized_memchr_sse2_iter(needle: u8, haystack: &[u8]) -> usize {
    OneMatchesIter::new(needle, haystack).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        dbg!(OneMatchesIter::new(b',', b"name,surname,age").collect::<Vec<_>>());
        dbg!(memchr::memchr_iter(b',', b"name,surname,age").collect::<Vec<_>>());

        assert_eq!(scalar_baseline(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_loop(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_loop_amortized(b',', b"name,surname,age"), 2);
        assert_eq!(memchr_sse2_iter(b',', b"name,surname,age"), 2);
        assert_eq!(memoized_memchr_sse2_iter(b',', b"name,surname,age"), 2);
    }

    // TODO: test from wide_split
    // TODO: test from bench itself
}
