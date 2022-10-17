use super::{decode, encode};

const ENCODER_TEST_CASES: &[&[u8]] = &[&[], &[0, 0, 0], &[255, 239]];

fn encoder_roundtrips(buf: &[u8]) {
    assert_eq!(&decode(&encode(buf)).unwrap(), buf);
}

#[test]
fn test_encoder_test_cases() {
    for test_case in ENCODER_TEST_CASES {
        encoder_roundtrips(test_case);
    }
}

#[cfg(fuzzing)]
mod fuzztests {
    // Run this test with `cargo fuzzcheck 'tests::fuzztests::test_encoder_roundtrips'.
    // See https://github.com/loiclec/fuzzcheck-rs for more details.
    #[test]
    fn test_encoder_roundtrips() {
        let result = fuzzcheck::fuzz_test(super::encoder_roundtrips)
            .default_options()
            .stop_after_first_test_failure(true)
            .launch();
        assert!(result.found_test_failure);
    }
}

mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_encoder_roundtrips(buf in any::<Vec<u8>>()) {
            super::encoder_roundtrips(&buf);
        }
    }
}
