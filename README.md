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
                        time:   [2.3941 ms 2.4884 ms 2.5847 ms]
                        thrpt:  [830.85 Gelem/s 862.98 Gelem/s 896.98 Gelem/s]
```

Demo:

```
> RUSTFLAGS="-Ctarget-cpu=native" cargo run --release 

{"uid":0,"account_balance":0 ~ {"uid":0,"account_balance":99999,"_fixup":"l5555550000OVH|4830083526" (481b9c93)
Found within 7854 iters in 263.712µs
{"uid":1,"account_balance":0 ~ {"uid":1,"account_balance":99999,"_fixup":"p55555558000EMPz4020643147" (54b49214)
Found within 277892 iters in 9.409708ms
{"uid":2,"account_balance":0 ~ {"uid":2,"account_balance":99999,"_fixup":"n50000^RAO4376563586" (614d8795)
Found within 573 iters in 19.227µs
{"uid":3,"account_balance":0 ~ {"uid":3,"account_balance":99999,"_fixup":"f5555557000JTNt5038451034" (6de67d16)
Found within 243305 iters in 8.24595ms
{"uid":4,"account_balance":0 ~ {"uid":4,"account_balance":99999,"_fixup":"f55552100TEE|5038451034" (7a7f7297)
Found within 397459 iters in 13.483199ms
{"uid":5,"account_balance":0 ~ {"uid":5,"account_balance":99999,"_fixup":"f555554000_MG}4012507866" (87186818)
Found within 137630 iters in 4.690486ms
{"uid":6,"account_balance":0 ~ {"uid":6,"account_balance":99999,"_fixup":"g56000M_Ad4903300127" (93b15d99)
Found within 197580 iters in 6.699344ms
{"uid":7,"account_balance":0 ~ {"uid":7,"account_balance":99999,"_fixup":"d5552000LCMd4903300127" (a04a531a)
Found within 77899 iters in 2.643386ms
{"uid":8,"account_balance":0 ~ {"uid":8,"account_balance":99999,"_fixup":"a5555553000ETAA4012507734" (ace3489b)
Found within 98916 iters in 3.361726ms
{"uid":9,"account_balance":0 ~ {"uid":9,"account_balance":99999,"_fixup":"c555550000F_G{2622951835" (b97c3e1c)
Found within 7109 iters in 238.174µs
{"uid":10,"account_balance":0 ~ {"uid":10,"account_balance":99999,"_fixup":"i5555551000QLVn4122787310" (9485d7a4)
Found within 54640 iters in 1.839315ms
{"uid":11,"account_balance":0 ~ {"uid":11,"account_balance":99999,"_fixup":"m555553000FU]^4012507563" (a11ecd25)
Found within 127621 iters in 4.332454ms
{"uid":12,"account_balance":0 ~ {"uid":12,"account_balance":99999,"_fixup":"g55551000QDBz4376563586" (adb7c2a6)
Found within 33904 iters in 1.14407ms
{"uid":13,"account_balance":0 ~ {"uid":13,"account_balance":99999,"_fixup":"h0000UVNz4020643147" (ba50b827)
Found within 14004 iters in 475.526µs
{"uid":14,"account_balance":0 ~ {"uid":14,"account_balance":99999,"_fixup":"l555552000N[Ep2622951835" (c6e9ada8)
Found within 70477 iters in 2.386747ms
{"uid":15,"account_balance":0 ~ {"uid":15,"account_balance":99999,"_fixup":"c1000KJGg2622951835" (d382a329)
Found within 39210 iters in 1.325436ms
```