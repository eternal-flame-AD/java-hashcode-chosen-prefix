Java hashcode structured chosen-prefix/splicing collision finder using tableless SIMD meet-in-the-middle.


Benchmark:

Time = time to find one collision.
Thrpt = number of equivalent hash trials per second.

```
> RUSTFLAGS="-Ctarget-feature=+simd128" cargo bench --target=wasm32-wasipi # wasmtime SIMD128
collision/meet_in_the_middle
                        time:   [12.012 ms 13.202 ms 14.405 ms]
                        thrpt:  [149.08 Gelem/s 162.67 Gelem/s 178.78 Gelem/s]
> cargo bench # baseline SSE2
collision/meet_in_the_middle
                        time:   [11.428 ms 12.514 ms 13.633 ms]
                        thrpt:  [157.52 Gelem/s 171.61 Gelem/s 187.92 Gelem/s]
> RUSTFLAGS="-Ctarget-feature=+avx2" cargo bench # AVX2
collision/meet_in_the_middle
                        time:   [4.6007 ms 4.8741 ms 5.1513 ms]
                        thrpt:  [416.88 Gelem/s 440.59 Gelem/s 466.77 Gelem/s]
> RUSTFLAGS="-Ctarget-feature=+avx512f" cargo bench # AVX-512
collision/meet_in_the_middle
                        time:   [2.4931 ms 2.5985 ms 2.7054 ms]
                        thrpt:  [793.77 Gelem/s 826.43 Gelem/s 861.38 Gelem/s]
```

Demo:

```
> RUSTFLAGS="-Ctarget-cpu=native" cargo run --release 

{"uid":0,"account_balance":0 ~ {"uid":0,"account_balance":99999,"_fixup":"j50000G[O_4122787310" (481b9c93)
Found within 15174 iters in 262.791µs
{"uid":1,"account_balance":0 ~ {"uid":1,"account_balance":99999,"_fixup":"g58100LGLq6403136037" (54b49214)
Found within 601291 iters in 10.429732ms
{"uid":2,"account_balance":0 ~ {"uid":2,"account_balance":99999,"_fixup":"n50000^RAO4376563586" (614d8795)
Found within 573 iters in 10.159µs
{"uid":3,"account_balance":0 ~ {"uid":3,"account_balance":99999,"_fixup":"i52200`NZa5038451205" (6de67d16)
Found within 746943 iters in 12.749041ms
{"uid":4,"account_balance":0 ~ {"uid":4,"account_balance":99999,"_fixup":"f57100WVOf0249477215" (7a7f7297)
Found within 572086 iters in 9.75622ms
{"uid":5,"account_balance":0 ~ {"uid":5,"account_balance":99999,"_fixup":"e5556000K[[z4830083526" (87186818)
Found within 224074 iters in 3.828896ms
{"uid":6,"account_balance":0 ~ {"uid":6,"account_balance":99999,"_fixup":"g56000M_Ad4903300127" (93b15d99)
Found within 197580 iters in 3.377776ms
{"uid":7,"account_balance":0 ~ {"uid":7,"account_balance":99999,"_fixup":"d5552000LCMd4903300127" (a04a531a)
Found within 77899 iters in 1.314344ms
{"uid":8,"account_balance":0 ~ {"uid":8,"account_balance":99999,"_fixup":"a5552100YYBp5038451205" (ace3489b)
Found within 395032 iters in 6.758407ms
{"uid":9,"account_balance":0 ~ {"uid":9,"account_balance":99999,"_fixup":"a553000\\Cp4012507866" (b97c3e1c)
Found within 101243 iters in 1.707984ms
{"uid":10,"account_balance":0 ~ {"uid":10,"account_balance":99999,"_fixup":"d559000VJAE5038451034" (9485d7a4)
Found within 295221 iters in 5.024804ms
{"uid":11,"account_balance":0 ~ {"uid":11,"account_balance":99999,"_fixup":"e5557000TUTy4012507734" (a11ecd25)
Found within 249491 iters in 4.208198ms
{"uid":12,"account_balance":0 ~ {"uid":12,"account_balance":99999,"_fixup":"k5551100^GBv6403136037" (adb7c2a6)
Found within 361693 iters in 6.185626ms
{"uid":13,"account_balance":0 ~ {"uid":13,"account_balance":99999,"_fixup":"h0000UVNz4020643147" (ba50b827)
Found within 14004 iters in 236.23µs
{"uid":14,"account_balance":0 ~ {"uid":14,"account_balance":99999,"_fixup":"h53000DYMm0249477215" (c6e9ada8)
Found within 111363 iters in 1.899259ms
{"uid":15,"account_balance":0 ~ {"uid":15,"account_balance":99999,"_fixup":"c1000KJGg2622951835" (d382a329)
Found within 39210 iters in 680.226µs
```