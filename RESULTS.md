# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [empty_string](#empty_string)
    - [very_short_string](#very_short_string)
    - [very_short_string_no_matches](#very_short_string_no_matches)
    - [short_string_dense_matches](#short_string_dense_matches)
    - [short_string_no_matches](#short_string_no_matches)
    - [long_string_no_matches](#long_string_no_matches)
    - [long_string_dense_matches](#long_string_dense_matches)
    - [long_string_sparse_matches](#long_string_sparse_matches)
    - [long_string_very_sparse_matches](#long_string_very_sparse_matches)
    - [long_string_all_matches](#long_string_all_matches)

## Benchmark Results

### empty_string

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `1.22 ns` (✅ **1.00x**)    | `1.21 ns` (✅ **1.01x faster**) | `1.21 ns` (✅ **1.01x faster**)        | `1.21 ns` (✅ **1.00x faster**) | `1.21 ns` (✅ **1.00x faster**)        |

### very_short_string

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `2.42 ns` (✅ **1.00x**)    | `3.56 ns` (❌ *1.47x slower*)   | `3.19 ns` (❌ *1.32x slower*)          | `3.26 ns` (❌ *1.35x slower*)   | `3.93 ns` (❌ *1.62x slower*)          |

### very_short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `2.42 ns` (✅ **1.00x**)    | `2.92 ns` (❌ *1.21x slower*)   | `2.86 ns` (❌ *1.18x slower*)          | `2.99 ns` (❌ *1.23x slower*)   | `2.73 ns` (❌ *1.13x slower*)          |

### short_string_dense_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `8.30 ns` (✅ **1.00x**)    | `9.74 ns` (❌ *1.17x slower*)   | `9.62 ns` (❌ *1.16x slower*)          | `6.66 ns` (✅ **1.25x faster**) | `7.28 ns` (✅ **1.14x faster**)        |

### short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`             | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:-------------------------------|:------------------------------------- |
|        | `9.56 ns` (✅ **1.00x**)    | `3.14 ns` (🚀 **3.05x faster**) | `3.17 ns` (🚀 **3.02x faster**)        | `2.67 ns` (🚀 **3.59x faster**) | `5.34 ns` (✅ **1.79x faster**)        |

### long_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `43.69 us` (✅ **1.00x**)   | `2.23 us` (🚀 **19.60x faster**) | `2.23 us` (🚀 **19.64x faster**)       | `2.22 us` (🚀 **19.68x faster**) | `3.20 us` (🚀 **13.67x faster**)       |

### long_string_dense_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `35.46 us` (✅ **1.00x**)   | `73.47 us` (❌ *2.07x slower*)   | `73.26 us` (❌ *2.07x slower*)         | `62.55 us` (❌ *1.76x slower*)   | `10.65 us` (🚀 **3.33x faster**)       |

### long_string_sparse_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `45.16 us` (✅ **1.00x**)   | `31.10 us` (✅ **1.45x faster**) | `31.00 us` (✅ **1.46x faster**)       | `27.45 us` (✅ **1.65x faster**) | `9.08 us` (🚀 **4.97x faster**)        |

### long_string_very_sparse_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:--------------------------------|:------------------------------------- |
|        | `122.43 us` (✅ **1.00x**)  | `38.93 us` (🚀 **3.14x faster**) | `38.78 us` (🚀 **3.16x faster**)       | `34.61 us` (🚀 **3.54x faster**) | `15.18 us` (🚀 **8.07x faster**)       |

### long_string_all_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`                | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`                | `memoized_memchr_sse2_iter`           |
|:-------|:---------------------------|:----------------------------------|:--------------------------------------|:----------------------------------|:------------------------------------- |
|        | `45.11 us` (✅ **1.00x**)   | `627.89 us` (❌ *13.92x slower*)   | `624.44 us` (❌ *13.84x slower*)       | `520.24 us` (❌ *11.53x slower*)   | `99.86 us` (❌ *2.21x slower*)         |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

