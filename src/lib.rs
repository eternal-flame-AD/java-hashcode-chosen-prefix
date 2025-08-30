#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "wasm32")]
use std::arch::wasm32::*;

use std::io::Write;

pub const fn hashcode(bytes: &[u8]) -> u32 {
    let mut h: u32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        h = h.wrapping_mul(31).wrapping_add(bytes[i] as u32);
        i += 1;
    }
    h
}

pub const fn hash_update(state: &mut u32, bytes: &[u8]) {
    let mut i = 0;
    while i < bytes.len() {
        *state = state.wrapping_mul(31).wrapping_add(bytes[i] as u32);
        i += 1;
    }
}

const SPLICE_TARGETS: [[u8; 12]; 16] = const {
    let mut buf = [*br#","_fixup":"?"#; 16];

    let mut i = 0;
    while i < 16 {
        buf[i][11] = b'a' + i as u8;
        i += 1;
    }

    buf
};

const fn hash_rewind(final_state: u32, msg: &[u8]) -> u32 {
    let mut new_state = final_state;
    let mut i = msg.len();
    while i > 0 {
        new_state = new_state.wrapping_sub(msg[i - 1] as u32);
        new_state = new_state.wrapping_mul(MOD_INVERSE_FOR_31);
        i -= 1;
    }
    new_state
}

// precomputed modular inverse for 31
const MOD_INVERSE_FOR_31: u32 = 3186588639;

// preimages that hash to a prime number for good feng-shui and has unique residue modulo 31 to meet in the middle
const BACK_SPLICES: [[u8; 10]; 16] = [
    *b"5038451034",
    *b"0249477215",
    *b"4012507563",
    *b"4122787310",
    *b"3460899734",
    *b"4376563586",
    *b"3101342757",
    *b"5038451205",
    *b"4903300127",
    *b"4012507734",
    *b"3101342881",
    *b"6403136037",
    *b"4830083526",
    *b"4012507866",
    *b"4020643147",
    *b"2622951835",
];

pub struct CollisionResult {
    buf: [u8; 64],
    len: usize,
    pub iters: u64,
}

impl CollisionResult {
    pub fn msg(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

const NONCE_START: u8 = b'\"' + 1;
const NONCE_END: u8 = b'~';

fn check_json_string(s: &[u8]) -> bool {
    let mut escaped = false;
    for &c in s {
        if !escaped {
            if c == b'\\' {
                escaped = true;
            }
        } else {
            match c {
                b'b' | b'f' | b'n' | b'r' | b't' | b'\\' | b'/' => escaped = false,
                _ => return false,
            }
        }
    }
    !escaped
}

#[cfg(target_arch = "wasm32")]
pub fn find_collision(
    midstate: u32,
    original_prefix: &[u8],
    target_prefix: &[u8],
) -> CollisionResult {
    let mut goal = midstate;
    hash_update(&mut goal, original_prefix);

    macro_rules! u32x4_mulloi {
        ($x:expr, 31) => {
            u32x4_sub(u32x4_shl($x, 5), $x)
        };
    }

    let mut target_midstate = midstate;
    hash_update(&mut target_midstate, target_prefix);

    let mut target_contribs = [target_midstate; 8];
    for i in 0..8 {
        hash_update(&mut target_contribs[i], &SPLICE_TARGETS[i]);
    }

    unsafe {
        let goals: [u32; 8] =
            core::array::from_fn(|i| hash_rewind(hash_rewind(goal, b"\""), &BACK_SPLICES[i]));
        let goals: [_; 8] =
            core::array::from_fn(|i| u32x4_splat(goals[i].wrapping_sub(NONCE_START as u32)));
        let nonce_range = i32x4_splat((NONCE_END - NONCE_START) as i32);
        let target_contribs_04_base = u32x4(
            target_contribs[0],
            target_contribs[1],
            target_contribs[2],
            target_contribs[3],
        );
        let target_contribs_48_base = u32x4(
            target_contribs[4],
            target_contribs[5],
            target_contribs[6],
            target_contribs[7],
        );
        let mut cmp_04_mask = u32x4_splat(!0);
        let mut cmp_48_mask = u32x4_splat(!0);

        for x0 in 0..10000u64 {
            let mut target_contribs_04 = target_contribs_04_base;
            let mut target_contribs_48 = target_contribs_48_base;
            let mut t0 = x0;
            for _ in 0..4 {
                let d = (t0 % 10) as u32 + (b'0' as u32);
                let dv = u32x4_splat(d);
                target_contribs_04 = u32x4_mulloi!(target_contribs_04, 31);
                target_contribs_04 = u32x4_add(target_contribs_04, dv);
                target_contribs_48 = u32x4_mulloi!(target_contribs_48, 31);
                target_contribs_48 = u32x4_add(target_contribs_48, dv);
                t0 /= 10;
            }
            for x1 in 0..(32 * 32 * 32) {
                let mut final_hashes_04 = target_contribs_04;
                // hide some latency from comparisons
                let mut final_hashes_48 = target_contribs_48;
                let mut t = x1;

                for _ in 0..3 {
                    let d = (t % 32) as u32 + (b'A' as u32);
                    let dv = u32x4_splat(d);
                    final_hashes_04 = u32x4_mulloi!(final_hashes_04, 31);
                    final_hashes_04 = u32x4_add(final_hashes_04, dv);
                    final_hashes_48 = u32x4_mulloi!(final_hashes_48, 31);
                    final_hashes_48 = u32x4_add(final_hashes_48, dv);
                    t /= 32;
                }
                final_hashes_04 = u32x4_mulloi!(final_hashes_04, 31);
                final_hashes_48 = u32x4_mulloi!(final_hashes_48, 31);

                for g in goals {
                    let diff_04 = u32x4_sub(g, final_hashes_04);
                    let diff_48 = u32x4_sub(g, final_hashes_48);
                    let cmp_04 = u32x4_ge(diff_04, nonce_range);
                    let cmp_48 = u32x4_ge(diff_48, nonce_range);
                    cmp_04_mask = v128_and(cmp_04_mask, cmp_04);
                    cmp_48_mask = v128_and(cmp_48_mask, cmp_48);
                }
                let cmp_04_match = !u32x4_all_true(cmp_04_mask);
                let cmp_48_match = !u32x4_all_true(cmp_48_mask);

                if cmp_04_match | cmp_48_match {
                    let final_hashes = if cmp_48_match {
                        final_hashes_48
                    } else {
                        final_hashes_04
                    };

                    let match_idx_back = goals
                        .iter()
                        .position(|&g| {
                            let diff = u32x4_sub(g, final_hashes);
                            let cmp = u32x4_ge(diff, nonce_range);
                            !u32x4_all_true(cmp)
                        })
                        .unwrap();

                    let mut goal_diffs = [0u32; 4];
                    v128_store(
                        goal_diffs.as_mut_ptr().cast(),
                        u32x4_sub(goals[match_idx_back], final_hashes),
                    );

                    let match_idx_front = goal_diffs
                        .iter()
                        .position(|&g| g < (NONCE_END - NONCE_START) as _)
                        .unwrap();

                    let mut final_msg = std::io::Cursor::new([0u8; 64]);
                    final_msg
                        .write_all(
                            &SPLICE_TARGETS[match_idx_front + if cmp_48_match { 4 } else { 0 }],
                        )
                        .unwrap();
                    let mut t = x0;
                    for _ in 0..4 {
                        final_msg.write_all(&[(t % 10) as u8 + b'0']).unwrap();
                        t /= 10;
                    }
                    t = x1;
                    for _ in 0..3 {
                        final_msg.write_all(&[(t % 32) as u8 + b'A']).unwrap();
                        t /= 32;
                    }
                    if !check_json_string(&final_msg.get_ref()[final_msg.position() as usize - 3..])
                    {
                        cmp_04_mask = u32x4_splat(!0);
                        cmp_48_mask = u32x4_splat(!0);
                        continue;
                    }
                    final_msg
                        .write_all(&[NONCE_START + (goal_diffs[match_idx_front] as u8)])
                        .unwrap();
                    final_msg.write_all(&BACK_SPLICES[match_idx_back]).unwrap();
                    final_msg.write_all(&[b'"']).unwrap();

                    let count = x0 * 1000 + x1;

                    return CollisionResult {
                        len: final_msg.position() as usize,
                        buf: final_msg.into_inner(),
                        iters: count,
                    };
                }
            }
        }
    }

    panic!("No collision found within search space");
}

#[cfg(not(target_feature = "avx512f"))]
#[cfg(target_feature = "sse2")]
pub fn find_collision(
    midstate: u32,
    original_prefix: &[u8],
    target_prefix: &[u8],
) -> CollisionResult {
    let mut goal = midstate;
    hash_update(&mut goal, original_prefix);

    macro_rules! _mm_mulloi_epi32 {
        ($x:expr, 31) => {
            _mm_sub_epi32(_mm_slli_epi32($x, 5), $x)
        };
    }

    #[cfg(not(target_feature = "sse4.1"))]
    macro_rules! _mm_check_mask {
        ($x:expr) => {
            _mm_movemask_epi8($x) != 0
        };
    }

    #[cfg(target_feature = "sse4.1")]
    macro_rules! _mm_check_mask {
        ($x:expr) => {
            _mm_test_all_zeros($x, $x) == 0
        };
    }

    let mut target_midstate = midstate;
    hash_update(&mut target_midstate, target_prefix);

    let mut target_contribs = [target_midstate; 8];
    for i in 0..8 {
        hash_update(&mut target_contribs[i], &SPLICE_TARGETS[i]);
    }

    unsafe {
        let goals: [u32; 8] =
            core::array::from_fn(|i| hash_rewind(hash_rewind(goal, b"\""), &BACK_SPLICES[i]));
        let goals: [_; 8] =
            core::array::from_fn(
                |i| _mm_set1_epi32(goals[i].wrapping_sub(NONCE_START as u32) as _),
            );
        let nonce_range = _mm_set1_epi32(((NONCE_END - NONCE_START) as i32).wrapping_add(i32::MIN));
        let target_contribs_04_base = _mm_loadu_si128(target_contribs.as_ptr().cast());
        let target_contribs_48_base = _mm_loadu_si128(target_contribs.as_ptr().add(4).cast());

        for x0 in 0..10000u64 {
            let mut target_contribs_04 = target_contribs_04_base;
            let mut target_contribs_48 = target_contribs_48_base;
            let mut t0 = x0;
            for _ in 0..4 {
                let d = (t0 % 10) as u32 + (b'0' as u32);
                let dv = _mm_set1_epi32(d as _);
                target_contribs_04 = _mm_mulloi_epi32!(target_contribs_04, 31);
                target_contribs_04 = _mm_add_epi32(target_contribs_04, dv);
                target_contribs_48 = _mm_mulloi_epi32!(target_contribs_48, 31);
                target_contribs_48 = _mm_add_epi32(target_contribs_48, dv);
                t0 /= 10;
            }
            for x1 in 0..(32 * 32 * 32) {
                let mut final_hashes_04 = target_contribs_04;
                // hide some latency from comparisons
                let mut final_hashes_48 = target_contribs_48;
                let mut t = x1;

                for _ in 0..3 {
                    let d = (t % 32) as u32 + (b'A' as u32);
                    let dv = _mm_set1_epi32(d as _);
                    final_hashes_04 = _mm_mulloi_epi32!(final_hashes_04, 31);
                    final_hashes_04 = _mm_add_epi32(final_hashes_04, dv);
                    final_hashes_48 = _mm_mulloi_epi32!(final_hashes_48, 31);
                    final_hashes_48 = _mm_add_epi32(final_hashes_48, dv);
                    t /= 32;
                }
                final_hashes_04 = _mm_mulloi_epi32!(final_hashes_04, 31);
                final_hashes_48 = _mm_mulloi_epi32!(final_hashes_48, 31);

                // code path with _mm_min_epu32 support
                #[cfg(target_feature = "sse4.1")]
                let (cmp_04_match, cmp_48_match) = {
                    let min_diff_04 = goals
                        .map(|g| _mm_sub_epi32(g, final_hashes_04))
                        .into_iter()
                        .reduce(|x, y| _mm_min_epu32(x, y))
                        .map(|x| _mm_add_epi32(x, _mm_set1_epi32(i32::MIN)))
                        .unwrap();

                    let min_diff_48 = goals
                        .map(|g| _mm_sub_epi32(g, final_hashes_48))
                        .into_iter()
                        .reduce(|x, y| _mm_min_epu32(x, y))
                        .map(|x| _mm_add_epi32(x, _mm_set1_epi32(i32::MIN)))
                        .unwrap();

                    let cmp_04 = _mm_cmplt_epi32(min_diff_04, nonce_range);

                    let cmp_48 = _mm_cmplt_epi32(min_diff_48, nonce_range);

                    (_mm_check_mask!(cmp_04), _mm_check_mask!(cmp_48))
                };

                // code path without _mm_min_epu32 support (scalar reduction)
                #[cfg(not(target_feature = "sse4.1"))]
                let (cmp_04_match, cmp_48_match) = {
                    let mut cmp_04_mask = _mm_setzero_si128();
                    let mut cmp_48_mask = _mm_setzero_si128();
                    for g in goals {
                        let diff_04 = _mm_sub_epi32(g, final_hashes_04);
                        let diff_48 = _mm_sub_epi32(g, final_hashes_48);
                        let cmp_04 = _mm_cmplt_epi32(
                            _mm_add_epi32(diff_04, _mm_set1_epi32(i32::MIN)),
                            nonce_range,
                        );
                        let cmp_48 = _mm_cmplt_epi32(
                            _mm_add_epi32(diff_48, _mm_set1_epi32(i32::MIN)),
                            nonce_range,
                        );
                        cmp_04_mask = _mm_or_si128(cmp_04_mask, cmp_04);
                        cmp_48_mask = _mm_or_si128(cmp_48_mask, cmp_48);
                    }
                    (_mm_check_mask!(cmp_04_mask), _mm_check_mask!(cmp_48_mask))
                };

                if cmp_04_match | cmp_48_match {
                    let final_hashes = if cmp_48_match {
                        final_hashes_48
                    } else {
                        final_hashes_04
                    };

                    let match_idx_back = goals
                        .iter()
                        .position(|&g| {
                            let diff = _mm_sub_epi32(g, final_hashes);
                            let cmp = _mm_cmplt_epi32(
                                _mm_add_epi32(diff, _mm_set1_epi32(i32::MIN)),
                                nonce_range,
                            );
                            _mm_check_mask!(cmp)
                        })
                        .unwrap();

                    let mut goal_diffs = [0u32; 4];
                    _mm_storeu_si128(
                        goal_diffs.as_mut_ptr().cast(),
                        _mm_sub_epi32(goals[match_idx_back], final_hashes),
                    );

                    let match_idx_front = goal_diffs
                        .iter()
                        .position(|&g| g < (NONCE_END - NONCE_START) as _)
                        .unwrap();

                    let mut final_msg = std::io::Cursor::new([0u8; 64]);
                    final_msg
                        .write_all(
                            &SPLICE_TARGETS[match_idx_front + if cmp_48_match { 4 } else { 0 }],
                        )
                        .unwrap();
                    let mut t = x0;
                    for _ in 0..4 {
                        final_msg.write_all(&[(t % 10) as u8 + b'0']).unwrap();
                        t /= 10;
                    }
                    t = x1;
                    for _ in 0..3 {
                        final_msg.write_all(&[(t % 32) as u8 + b'A']).unwrap();
                        t /= 32;
                    }

                    if !check_json_string(&final_msg.get_ref()[final_msg.position() as usize - 3..])
                    {
                        continue;
                    }

                    final_msg
                        .write_all(&[NONCE_START + (goal_diffs[match_idx_front] as u8)])
                        .unwrap();
                    final_msg.write_all(&BACK_SPLICES[match_idx_back]).unwrap();
                    final_msg.write_all(&[b'"']).unwrap();

                    let count = x0 * 1000 + x1;

                    return CollisionResult {
                        len: final_msg.position() as usize,
                        buf: final_msg.into_inner(),
                        iters: count,
                    };
                }
            }
        }
    }

    panic!("No collision found within search space");
}

#[cfg(target_feature = "avx512f")]
pub fn find_collision(
    midstate: u32,
    original_prefix: &[u8],
    target_prefix: &[u8],
) -> CollisionResult {
    let mut goal = midstate;
    hash_update(&mut goal, original_prefix);

    let mut target_midstate = midstate;
    hash_update(&mut target_midstate, target_prefix);

    let mut target_contribs = [target_midstate; 16];
    for i in 0..16 {
        hash_update(&mut target_contribs[i], &SPLICE_TARGETS[i]);
    }

    macro_rules! _mm512_mulloi_epi32 {
        ($x:expr, 31) => {
            _mm512_sub_epi32(_mm512_slli_epi32($x, 5), $x)
        };
    }

    unsafe {
        let goals: [u32; 16] =
            core::array::from_fn(|i| hash_rewind(hash_rewind(goal, b"\""), &BACK_SPLICES[i]));
        let goals: [_; 16] = core::array::from_fn(|i| {
            _mm512_set1_epi32(goals[i].wrapping_sub(NONCE_START as u32) as _)
        });
        let nonce_range = _mm512_set1_epi32((NONCE_END - NONCE_START) as _);
        let target_contribs_base = _mm512_loadu_si512(target_contribs.as_ptr().cast());
        let mut target_contribs_5_base = target_contribs_base;
        target_contribs_5_base = _mm512_mulloi_epi32!(target_contribs_5_base, 31);
        target_contribs_5_base =
            _mm512_add_epi32(target_contribs_5_base, _mm512_set1_epi32(b'5' as _));

        for x0 in 0..10000u64 {
            let mut target_contribs = target_contribs_base;
            let mut target_contribs_5 = target_contribs_5_base;
            let mut t0 = x0;
            for _ in 0..4 {
                let d = (t0 % 10) as u32 + (b'0' as u32);
                let dv = _mm512_set1_epi32(d as _);
                target_contribs = _mm512_mulloi_epi32!(target_contribs, 31);
                target_contribs = _mm512_add_epi32(target_contribs, dv);
                target_contribs_5 = _mm512_mulloi_epi32!(target_contribs_5, 31);
                target_contribs_5 = _mm512_add_epi32(target_contribs_5, dv);
                t0 /= 10;
            }
            for x1 in 0..(32 * 32 * 32) {
                let mut final_hashes = target_contribs;
                // hide some latency from comparisons
                let mut final_hashes_5 = target_contribs_5;
                let mut t = x1;

                for _ in 0..3 {
                    let d = (t % 32) as u32 + (b'A' as u32);
                    let dv = _mm512_set1_epi32(d as _);
                    final_hashes = _mm512_mulloi_epi32!(final_hashes, 31);
                    final_hashes = _mm512_add_epi32(final_hashes, dv);
                    final_hashes_5 = _mm512_mulloi_epi32!(final_hashes_5, 31);
                    final_hashes_5 = _mm512_add_epi32(final_hashes_5, dv);
                    t /= 32;
                }
                final_hashes = _mm512_mulloi_epi32!(final_hashes, 31);
                final_hashes_5 = _mm512_mulloi_epi32!(final_hashes_5, 31);

                let min_diff = goals
                    .map(|g| {
                        let diff = _mm512_sub_epi32(g, final_hashes);
                        let diff_5 = _mm512_sub_epi32(g, final_hashes_5);
                        _mm512_min_epu32(diff, diff_5)
                    })
                    .into_iter()
                    .reduce(|x, y| _mm512_min_epu32(x, y))
                    .unwrap();

                if _mm512_cmplt_epu32_mask(min_diff, nonce_range) != 0 {
                    let mut mask = 0;

                    for i in 0..16 {
                        let diff = _mm512_sub_epi32(goals[i], final_hashes);
                        mask |= _mm512_cmplt_epu32_mask(diff, nonce_range);
                    }

                    let is_5_match = mask == 0;

                    if is_5_match {
                        final_hashes = final_hashes_5;
                    }

                    let match_idx_back = goals
                        .iter()
                        .position(|&g| {
                            let diff = _mm512_sub_epi32(g, final_hashes);
                            _mm512_cmplt_epu32_mask(diff, nonce_range) != 0
                        })
                        .unwrap();

                    let mut goal_diffs = [0u32; 16];
                    _mm512_storeu_si512(
                        goal_diffs.as_mut_ptr().cast(),
                        _mm512_sub_epi32(goals[match_idx_back], final_hashes),
                    );

                    let match_idx_front = goal_diffs
                        .iter()
                        .position(|&g| g < (NONCE_END - NONCE_START) as _)
                        .unwrap();

                    let mut final_msg = std::io::Cursor::new([0u8; 64]);
                    final_msg
                        .write_all(&SPLICE_TARGETS[match_idx_front])
                        .unwrap();
                    if is_5_match {
                        final_msg.write_all(&[b'5']).unwrap();
                    }
                    let mut t = x0;
                    for _ in 0..4 {
                        final_msg.write_all(&[(t % 10) as u8 + b'0']).unwrap();
                        t /= 10;
                    }
                    t = x1;
                    for _ in 0..3 {
                        final_msg.write_all(&[(t % 32) as u8 + b'A']).unwrap();
                        t /= 32;
                    }
                    if !check_json_string(&final_msg.get_ref()[final_msg.position() as usize - 3..])
                    {
                        continue;
                    }
                    final_msg
                        .write_all(&[NONCE_START + (goal_diffs[match_idx_front] as u8)])
                        .unwrap();
                    final_msg.write_all(&BACK_SPLICES[match_idx_back]).unwrap();
                    final_msg.write_all(&[b'"']).unwrap();

                    let count = x0 * 1000 + x1;

                    return CollisionResult {
                        len: final_msg.position() as usize,
                        buf: final_msg.into_inner(),
                        iters: count,
                    };
                }
            }
        }
    }

    panic!("No collision found within search space");
}
