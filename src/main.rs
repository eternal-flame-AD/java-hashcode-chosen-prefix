use java_hashcode_chosen_prefix::{find_collision, hashcode};

fn main() {
    for uid in 0..16 {
        let common_prefix = format!(r#"{{"uid":{},"account_balance":"#, uid);
        let midstate_base = hashcode(common_prefix.as_bytes());

        let start = std::time::Instant::now();
        let result = find_collision(midstate_base, b"0", b"99999");
        let elapsed = start.elapsed();

        let msg_base = format!("{}0", common_prefix);
        let msg_new = format!(
            "{}99999{}",
            common_prefix,
            String::from_utf8_lossy(result.msg()),
        );

        let code = hashcode(msg_base.as_bytes());
        assert_eq!(code, hashcode(msg_new.as_bytes()));

        println!("{} ~ {} ({:08x})", msg_base, msg_new, code);

        eprintln!("Found within {} iters in {:?}", result.iters, elapsed);
    }
}
