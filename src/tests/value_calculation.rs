use tests::helpers::histo64;

#[test]
fn equivalent_range_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(1, h.equivalent_range(0));
    assert_eq!(1, h.equivalent_range(1));
    assert_eq!(1, h.equivalent_range(1023));
    // first in top half
    assert_eq!(1, h.equivalent_range(1024));
    // last in top half
    assert_eq!(1, h.equivalent_range(2047));
    // first in 2nd bucket
    assert_eq!(2, h.equivalent_range(2048));
    assert_eq!(2, h.equivalent_range(2049));
    // end of 2nd bucket
    assert_eq!(2, h.equivalent_range(4095));

    assert_eq!(7, h.bucket_count);
    // in 7th bucket
    assert_eq!(1 << 6, h.equivalent_range(100_000));
    // max value in top bucket
    assert_eq!(1 << 6, h.equivalent_range((1 << 17) - 1));
    // even bigger
    assert_eq!(1 << 7, h.equivalent_range((1 << 17)));
}

#[test]
fn equivalent_range_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    assert_eq!(4, h.equivalent_range(0));
    assert_eq!(4, h.equivalent_range(1));
    assert_eq!(4, h.equivalent_range(3));
    assert_eq!(4, h.equivalent_range(4));
    assert_eq!(4, h.equivalent_range(4095));
    // first in top half
    assert_eq!(4, h.equivalent_range(4096));
    // last in top half
    assert_eq!(4, h.equivalent_range(8188));
    // first in 2nd bucket
    assert_eq!(8, h.equivalent_range(8192));
    // end of 2nd bucket
    assert_eq!(8, h.equivalent_range(16384 - 7));

    assert_eq!(5, h.bucket_count);
    // in 5th bucket: same ranges as the unit magnitude 0 case because we're 2 buckets earlier, but
    // with magnitude 2 larger.
    assert_eq!(1 << 6, h.equivalent_range(100_000));
    // max value in top bucket
    assert_eq!(1 << 6, h.equivalent_range((1 << 17) - 1));
    // even bigger
    assert_eq!(1 << 7, h.equivalent_range((1 << 17)));
}

#[test]
fn equivalent_range_unit_magnitude_0_max_buckets() {
    let h = histo64(1, u64::max_value(), 3);

    assert_eq!(1, h.equivalent_range(0));
    assert_eq!(1, h.equivalent_range(1));
    assert_eq!(1, h.equivalent_range((1 << 11) - 1));
    // start of second bucket
    assert_eq!(1 << 1, h.equivalent_range(1 << 11));
    // third
    assert_eq!(1 << 2, h.equivalent_range(1 << 12));
    assert_eq!(1 << 3, h.equivalent_range(1 << 13));
    assert_eq!(1 << 4, h.equivalent_range(1 << 14));
    assert_eq!(1 << 5, h.equivalent_range(1 << 15));
    // ...

    assert_eq!(1 << 53, h.equivalent_range(1 << 63));
    assert_eq!(1 << 53, h.equivalent_range(u64::max_value()));
}

#[test]
fn equivalent_range_unit_magnitude_0_min_precision_max_buckets() {
    let h = histo64(1, u64::max_value(), 0);

    assert_eq!(1, h.equivalent_range(0));
    assert_eq!(1, h.equivalent_range(1));
    // start of second bucket
    assert_eq!(1 << 1, h.equivalent_range(1 << 1));
    // third
    assert_eq!(1 << 2, h.equivalent_range(1 << 2));
    // ...

    assert_eq!(1 << 63, h.equivalent_range(1 << 63));
    assert_eq!(1 << 63, h.equivalent_range(u64::max_value()));
}

#[test]
fn equivalent_range_unit_magnitude_0_max_precision_max_buckets() {
    let h = histo64(1, u64::max_value(), 5);

    assert_eq!(1, h.equivalent_range(0));
    assert_eq!(1, h.equivalent_range(1));
    assert_eq!(1, h.equivalent_range((1 << 18) - 1));
    // start of second bucket
    assert_eq!(1 << 1, h.equivalent_range(1 << 18));
    // third
    assert_eq!(1 << 2, h.equivalent_range(1 << 19));
    assert_eq!(1 << 3, h.equivalent_range(1 << 20));
    assert_eq!(1 << 4, h.equivalent_range(1 << 21));
    assert_eq!(1 << 5, h.equivalent_range(1 << 22));
    // ...

    assert_eq!(1 << 46, h.equivalent_range(1 << 63));
    assert_eq!(1 << 46, h.equivalent_range(u64::max_value()));
}

#[test]
fn equivalent_range_unit_magnitude_2_max_buckets() {
    let h = histo64(4, u64::max_value(), 3);

    assert_eq!(1 << 2, h.equivalent_range(0));
    assert_eq!(1 << 2, h.equivalent_range(1));
    assert_eq!(1 << 2, h.equivalent_range(4));
    assert_eq!(1 << 2, h.equivalent_range(1 << 12));
    assert_eq!(1 << 2, h.equivalent_range((1 << 13) - 1));
    // above lowest value, same ranges at all values
    // start of second bucket
    assert_eq!(1 << 3, h.equivalent_range(1 << 13));
    // third
    assert_eq!(1 << 4, h.equivalent_range(1 << 14));
    assert_eq!(1 << 5, h.equivalent_range(1 << 15));
    // ...

    assert_eq!(1 << 53, h.equivalent_range(1 << 63));
    assert_eq!(1 << 53, h.equivalent_range(u64::max_value()));
}

#[test]
fn equivalent_range_unit_magnitude_50_max_buckets() {
    let h = histo64(1 << 50, u64::max_value(), 3);

    // 11-bit sub buckets
    assert_eq!(2048, h.sub_bucket_count);

    assert_eq!(1 << 50, h.equivalent_range(0));
    assert_eq!(1 << 50, h.equivalent_range(1));
    assert_eq!(1 << 50, h.equivalent_range(4));
    assert_eq!(1 << 50, h.equivalent_range((1 << 61) - 1));
    // above lowest value, same ranges at all values
    // start of second bucket
    assert_eq!(1 << 51, h.equivalent_range(1 << 61));
    // third
    assert_eq!(1 << 52, h.equivalent_range(1 << 62));
    assert_eq!(1 << 53, h.equivalent_range(1 << 63));
    assert_eq!(1 << 53, h.equivalent_range(u64::max_value()));
}


#[test]
fn highest_equivalent_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(0, h.highest_equivalent(0));
    assert_eq!(1, h.highest_equivalent(1));
    assert_eq!(1023, h.highest_equivalent(1023));
    // first in top half
    assert_eq!(1024, h.highest_equivalent(1024));
    // last in top half
    assert_eq!(2047, h.highest_equivalent(2047));
    // first in 2nd bucket
    assert_eq!(2049, h.highest_equivalent(2048));
    assert_eq!(2049, h.highest_equivalent(2049));
    // end of 2nd bucket
    assert_eq!(4095, h.highest_equivalent(4095));
}

#[test]
fn highest_equivalent_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    assert_eq!(3, h.highest_equivalent(0));
    assert_eq!(3, h.highest_equivalent(1));
    assert_eq!(3, h.highest_equivalent(3));
    assert_eq!(7, h.highest_equivalent(4));
    assert_eq!(4095, h.highest_equivalent(4095));
    // first in top half
    assert_eq!(4099, h.highest_equivalent(4096));
    // last in top half
    assert_eq!(8191, h.highest_equivalent(8188));
    // first in 2nd bucket
    assert_eq!(8192 + 7, h.highest_equivalent(8192));
    // 2nd bucket has a scale of 8
    assert_eq!(8192 + 7, h.highest_equivalent(8192 + 7));
    // end of 2nd bucket
    assert_eq!(16384 - 1, h.highest_equivalent(16384 - 7));
}

#[test]
fn highest_equivalent_u64_max_value_saturates() {
    let h = histo64(1, u64::max_value(), 3);

    assert_eq!(u64::max_value() - 1, h.highest_equivalent(u64::max_value() - 1));

    assert_eq!(u64::max_value(), h.highest_equivalent(u64::max_value()));
}


#[test]
fn next_non_equivalent_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(1, h.next_non_equivalent(0));
    assert_eq!(2, h.next_non_equivalent(1));
    assert_eq!(1024, h.next_non_equivalent(1023));
    // first in top half
    assert_eq!(1025, h.next_non_equivalent(1024));
    // last in top half
    assert_eq!(2048, h.next_non_equivalent(2047));
    // first in 2nd bucket
    assert_eq!(2050, h.next_non_equivalent(2048));
    // but 2nd bucket has a scale of 2, so next value is same
    assert_eq!(2050, h.next_non_equivalent(2049));
    // end of 2nd bucket
    assert_eq!(4096, h.next_non_equivalent(4095));
}

#[test]
fn next_non_equivalent_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    assert_eq!(4, h.next_non_equivalent(0));
    assert_eq!(4, h.next_non_equivalent(1));
    assert_eq!(4, h.next_non_equivalent(3));
    assert_eq!(8, h.next_non_equivalent(4));
    assert_eq!(4096, h.next_non_equivalent(4095));
    // first in top half
    assert_eq!(4100, h.next_non_equivalent(4096));
    // last in top half
    assert_eq!(8192, h.next_non_equivalent(8188));
    // first in 2nd bucket
    assert_eq!(8192 + 8, h.next_non_equivalent(8192));
    // 2nd bucket has a scale of 8
    assert_eq!(8192 + 8, h.next_non_equivalent(8192 + 7));
    // end of 2nd bucket
    assert_eq!(16384, h.next_non_equivalent(16384 - 7));
}

#[test]
fn next_non_equivalent_u64_max_value_saturates() {
    let h = histo64(1, u64::max_value(), 3);

    // the next number would be quite a lot higher...
    assert_eq!(1_u64 << 53, h.equivalent_range(u64::max_value()));

    // ... but it's capped.
    assert_eq!(u64::max_value(), h.next_non_equivalent(u64::max_value() - 1));
    assert_eq!(u64::max_value(), h.next_non_equivalent(u64::max_value()));
}

#[test]
fn lowest_equivalent_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(0, h.lowest_equivalent(0));
    assert_eq!(1, h.lowest_equivalent(1));
    assert_eq!(1023, h.lowest_equivalent(1023));
    // first in top half
    assert_eq!(1024, h.lowest_equivalent(1024));
    // last in top half
    assert_eq!(2047, h.lowest_equivalent(2047));
    // first in 2nd bucket
    assert_eq!(2048, h.lowest_equivalent(2048));
    // but 2nd bucket has a scale of 2, so next value is same
    assert_eq!(2048, h.lowest_equivalent(2049));
    // end of 2nd bucket
    assert_eq!(4094, h.lowest_equivalent(4095));
}

#[test]
fn lowest_equivalent_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    assert_eq!(0, h.lowest_equivalent(0));
    assert_eq!(0, h.lowest_equivalent(1));
    assert_eq!(0, h.lowest_equivalent(3));
    assert_eq!(4, h.lowest_equivalent(4));
    // last in bottom half
    assert_eq!(1024 * 4 - 4, h.lowest_equivalent(1024 * 4 - 1));
    // first in top half
    assert_eq!(1024 * 4, h.lowest_equivalent(1024 * 4));
    // last in top half
    assert_eq!(2048 * 4 - 4, h.lowest_equivalent(2048 * 4 - 4));
    assert_eq!(2048 * 4 - 4, h.lowest_equivalent(2048 * 4 - 1));
    // first in 2nd bucket
    assert_eq!(8192, h.lowest_equivalent(8192));
    // 2nd bucket has a scale of 8
    assert_eq!(8192, h.lowest_equivalent(8192 + 7));
    // end of 2nd bucket
    assert_eq!(16384 - 8, h.lowest_equivalent(16384 - 8));
    assert_eq!(16384 - 8, h.lowest_equivalent(16384 - 1));
}

#[test]
fn value_from_loc_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(0, h.value_from_loc(0, 0));
    // end of first bucket
    assert_eq!(2048 - 1, h.value_from_loc(0, 2047));
    // start of second bucket
    assert_eq!(2048, h.value_from_loc(1, 1024));
    // scale is 2
    assert_eq!(4096 - 2, h.value_from_loc(1, 2047));
    assert_eq!(4096, h.value_from_loc(2, 1024));
}

#[test]
fn value_from_loc_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    assert_eq!(0, h.value_from_loc(0, 0));
    // end of first bucket
    assert_eq!((2048 - 1) * 4, h.value_from_loc(0, 2047));
    // start of second bucket
    assert_eq!(2048 * 4, h.value_from_loc(1, 1024));
    // scale is 8
    assert_eq!((4096 - 2) * 4, h.value_from_loc(1, 2047));
    assert_eq!(4096 * 4, h.value_from_loc(2, 1024));
}

#[test]
fn value_for_unit_magnitude_0() {
    let h = histo64(1, 100_000, 3);

    // first bucket
    assert_eq!(0, h.value_for(0));
    assert_eq!(1023, h.value_for(1023));
    assert_eq!(1024, h.value_for(1024));
    assert_eq!(2047, h.value_for(2047));
    // second bucket
    assert_eq!(2048, h.value_for(2048));
    assert_eq!(4096 - 2, h.value_for(3071));
}

#[test]
fn value_for_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);

    // first bucket
    assert_eq!(0, h.value_for(0));
    assert_eq!(1023 * 4, h.value_for(1023));
    assert_eq!(1024 * 4, h.value_for(1024));
    assert_eq!(2047 * 4, h.value_for(2047));
    // second bucket
    assert_eq!(2048 * 4, h.value_for(2048));
    assert_eq!((4096 - 2) * 4, h.value_for(3071));
}
