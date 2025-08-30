Java hashcode structured chosen-prefix/splicing collision finder using tableless SIMD meet-in-the-middle.

Supports baseline x86 (SSE2), AVX-512 and SIMD128.

Benchmark:

```
> cargo bench # baseline SSE2
collision/meet_in_the_middle
                        time:   [14.329 ms 15.713 ms 17.128 ms]
                        thrpt:  [125.38 Gelem/s 136.67 Gelem/s 149.87 Gelem/s]
> RUSTFLAGS="-Ctarget-feature=+simd128" cargo bench --target=wasm32-wasipi # wasmtime SIMD128
collision/meet_in_the_middle
                        time:   [12.012 ms 13.202 ms 14.405 ms]
                        thrpt:  [149.08 Gelem/s 162.67 Gelem/s 178.78 Gelem/s]
> RUSTFLAGS="-Ctarget-feature=+avx512f" cargo bench
collision/meet_in_the_middle
                        time:   [2.6456 ms 2.7643 ms 2.8838 ms]
                        thrpt:  [744.68 Gelem/s 776.86 Gelem/s 811.72 Gelem/s]


```

Demo:

```
> RUSTFLAGS="-Ctarget-cpu=native" cargo run --release 
 
{"uid":0,"account_balance":0 ~ {"uid":0,"account_balance":99999,"_fixup":"j50000G[O_4122787310" (481b9c93)
Found within 15174 iters in 151.979µs
{"uid":1,"account_balance":0 ~ {"uid":1,"account_balance":99999,"_fixup":"g58100LGLq6403136037" (54b49214)
Found within 29467 iters in 5.856019ms
{"uid":2,"account_balance":0 ~ {"uid":2,"account_balance":99999,"_fixup":"n50000^RAO4376563586" (614d8795)
Found within 573 iters in 6.162µs
{"uid":3,"account_balance":0 ~ {"uid":3,"account_balance":99999,"_fixup":"i52200`NZa5038451205" (6de67d16)
Found within 48047 iters in 7.455819ms
{"uid":4,"account_balance":0 ~ {"uid":4,"account_balance":99999,"_fixup":"f57100WVOf0249477215" (7a7f7297)
Found within 32030 iters in 5.628967ms
{"uid":5,"account_balance":0 ~ {"uid":5,"account_balance":99999,"_fixup":"d50300QRWo3460899734" (87186818)
Found within 53088 iters in 9.96526ms
{"uid":6,"account_balance":0 ~ {"uid":6,"account_balance":99999,"_fixup":"g56000M_Ad4903300127" (93b15d99)
Found within 6972 iters in 1.989993ms
{"uid":7,"account_balance":0 ~ {"uid":7,"account_balance":99999,"_fixup":"f9000[FJ{4830083526" (a04a531a)
Found within 18402 iters in 3.007671ms
{"uid":8,"account_balance":0 ~ {"uid":8,"account_balance":99999,"_fixup":"g6100_NPr4012507563" (ace3489b)
Found within 31806 iters in 5.324156ms
{"uid":9,"account_balance":0 ~ {"uid":9,"account_balance":99999,"_fixup":"p7300N^]|2622951835" (b97c3e1c)
Found within 66613 iters in 12.078007ms
{"uid":10,"account_balance":0 ~ {"uid":10,"account_balance":99999,"_fixup":"f50100WXO}0249477215" (9485d7a4)
Found within 25094 iters in 3.294137ms
{"uid":11,"account_balance":0 ~ {"uid":11,"account_balance":99999,"_fixup":"p2100WHYp3460899734" (a11ecd25)
Found within 36822 iters in 4.192169ms
{"uid":12,"account_balance":0 ~ {"uid":12,"account_balance":99999,"_fixup":"k4100V[Mg0249477215" (adb7c2a6)
Found within 27141 iters in 4.629692ms
{"uid":13,"account_balance":0 ~ {"uid":13,"account_balance":99999,"_fixup":"h0000UVNz4020643147" (ba50b827)
Found within 14004 iters in 145.648µs
{"uid":14,"account_balance":0 ~ {"uid":14,"account_balance":99999,"_fixup":"h53000DYMm0249477215" (c6e9ada8)
Found within 16059 iters in 1.144301ms
{"uid":15,"account_balance":0 ~ {"uid":15,"account_balance":99999,"_fixup":"c1000KJGg2622951835" (d382a329)
Found within 7442 iters in 376.307µs
```