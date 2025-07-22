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

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:------------------------------- |
|        | `1.21 ns` (✅ **1.00x**)    | `1.21 ns` (✅ **1.00x slower**) | `1.21 ns` (✅ **1.00x slower**)        | `1.21 ns` (✅ **1.00x slower**)  |

### very_short_string

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:------------------------------- |
|        | `3.31 ns` (✅ **1.00x**)    | `3.02 ns` (✅ **1.10x faster**) | `3.49 ns` (✅ **1.05x slower**)        | `2.63 ns` (✅ **1.26x faster**)  |

### very_short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:------------------------------- |
|        | `3.31 ns` (✅ **1.00x**)    | `2.96 ns` (✅ **1.12x faster**) | `2.90 ns` (✅ **1.14x faster**)        | `2.47 ns` (✅ **1.34x faster**)  |

### short_string_close_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:------------------------------- |
|        | `8.07 ns` (✅ **1.00x**)    | `8.38 ns` (✅ **1.04x slower**) | `8.49 ns` (✅ **1.05x slower**)        | `6.86 ns` (✅ **1.18x faster**)  |

### short_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`             | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`              |
|:-------|:---------------------------|:-------------------------------|:--------------------------------------|:------------------------------- |
|        | `9.55 ns` (✅ **1.00x**)    | `3.38 ns` (🚀 **2.82x faster**) | `3.14 ns` (🚀 **3.04x faster**)        | `3.38 ns` (🚀 **2.82x faster**)  |

### long_string_no_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`               |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:-------------------------------- |
|        | `46.96 us` (✅ **1.00x**)   | `2.64 us` (🚀 **17.78x faster**) | `2.22 us` (🚀 **21.17x faster**)       | `2.30 us` (🚀 **20.45x faster**)  |

### long_string_close_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`               |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:-------------------------------- |
|        | `35.38 us` (✅ **1.00x**)   | `73.55 us` (❌ *2.08x slower*)   | `73.16 us` (❌ *2.07x slower*)         | `63.06 us` (❌ *1.78x slower*)    |

### long_string_sparse_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`              | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`               |
|:-------|:---------------------------|:--------------------------------|:--------------------------------------|:-------------------------------- |
|        | `44.93 us` (✅ **1.00x**)   | `30.97 us` (✅ **1.45x faster**) | `30.88 us` (✅ **1.45x faster**)       | `26.87 us` (✅ **1.67x faster**)  |

### long_string_all_matches

|        | `scalar_baseline`          | `memchr_sse2_loop`                | `memchr_sse2_loop_amortized`          | `memchr_sse2_iter`                 |
|:-------|:---------------------------|:----------------------------------|:--------------------------------------|:---------------------------------- |
|        | `47.51 us` (✅ **1.00x**)   | `627.58 us` (❌ *13.21x slower*)   | `624.56 us` (❌ *13.15x slower*)       | `540.72 us` (❌ *11.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

