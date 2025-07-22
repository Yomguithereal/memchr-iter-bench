# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [empty_string](#empty_string)
    - [very_short_string](#very_short_string)
    - [very_short_string_no_matches](#very_short_string_no_matches)
    - [short_string_close_matches](#short_string_close_matches)
    - [short_string_no_matches](#short_string_no_matches)
    - [long_string_no_matches](#long_string_no_matches)
    - [long_string_close_matches](#long_string_close_matches)
    - [long_string_sparse_matches](#long_string_sparse_matches)
    - [long_string_all_matches](#long_string_all_matches)

## Benchmark Results

### empty_string

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `1.21 ns` (✅ **1.00x**)    | `1.22 ns` (✅ **1.01x slower**) | `1.21 ns` (✅ **1.00x slower**)        | `1.21 ns` (✅ **1.00x slower**) | `1.21 ns` (✅ **1.00x slower**)        |

### very_short_string

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `3.45 ns` (✅ **1.00x**)    | `3.01 ns` (✅ **1.15x faster**) | `3.45 ns` (✅ **1.00x faster**)        | `2.54 ns` (✅ **1.36x faster**) | `3.90 ns` (❌ *1.13x slower*)          |

### very_short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `3.45 ns` (✅ **1.00x**)    | `2.97 ns` (✅ **1.16x faster**) | `3.62 ns` (✅ **1.05x slower**)        | `2.53 ns` (✅ **1.37x faster**) | `2.92 ns` (✅ **1.18x faster**)        |

### short_string_close_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `8.11 ns` (✅ **1.00x**)    | `8.46 ns` (✅ **1.04x slower**) | `8.54 ns` (✅ **1.05x slower**)        | `6.97 ns` (✅ **1.16x faster**) | `6.79 ns` (✅ **1.19x faster**)        |

### short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `9.55 ns` (✅ **1.00x**)    | `3.15 ns` (🚀 **3.03x faster**) | `2.92 ns` (🚀 **3.27x faster**)        | `3.15 ns` (🚀 **3.03x faster**) | `5.24 ns` (🚀 **1.82x faster**)        |

### long_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `43.45 us` (✅ **1.00x**)   | `2.22 us` (🚀 **19.54x faster**) | `2.22 us` (🚀 **19.55x faster**)       | `2.33 us` (🚀 **18.64x faster**) | `3.19 us` (🚀 **13.62x faster**)       |

### long_string_close_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `35.41 us` (✅ **1.00x**)   | `73.58 us` (❌ *2.08x slower*)   | `73.28 us` (❌ *2.07x slower*)         | `63.11 us` (❌ *1.78x slower*)   | `10.49 us` (🚀 **3.38x faster**)       |

### long_string_sparse_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `45.07 us` (✅ **1.00x**)   | `30.91 us` (✅ **1.46x faster**) | `31.11 us` (✅ **1.45x faster**)       | `26.85 us` (✅ **1.68x faster**) | `6.01 us` (🚀 **7.50x faster**)        |

### long_string_all_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`                | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`                | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:----------------------------------|:--------------------------------------|:----------------------------------|:------------------------------------- |
|        | `45.02 us` (✅ **1.00x**)   | `630.40 us` (❌ *14.00x slower*)   | `623.20 us` (❌ *13.84x slower*)       | `529.96 us` (❌ *11.77x slower*)   | `99.56 us` (❌ *2.21x slower*)         |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

