#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod half_open {
    #[cfg(test)]
    mod explicit {
        use bitmask::bitmask;

        #[test]
        fn all() {
            const A: u32 = bitmask!(0..32);
            assert_eq!(A, 0b11111111111111111111111111111111u32);
        }

        #[test]
        fn arbitrary() {
            const A: u32 = bitmask!(2..9);
            assert_eq!(A, 0b111111100u32);
        }

        #[test]
        fn first() {
            const A: u32 = bitmask!(0..1);
            assert_eq!(A, 0b1u32);
        }

        #[test]
        fn last() {
            const A: u32 = bitmask!(31..32);
            assert_eq!(A, 0b10000000000000000000000000000000u32);
        }
    }

    #[cfg(test)]
    mod implicit {
        use bitmask::bitmask;

        #[test]
        fn all_to() {
            const A: u32 = bitmask!(..32);
            assert_eq!(A, 0b11111111111111111111111111111111u32);
        }

        #[test]
        fn all_from() {
            const A: u32 = bitmask!(0u32..);
            assert_eq!(A, 0b11111111111111111111111111111111u32);
        }

        #[test]
        fn arbitrary() {
            const A: u32 = bitmask!(..9);
            assert_eq!(A, 0b111111111u32);
        }

        #[test]
        fn first() {
            const A: u32 = bitmask!(..1);
            assert_eq!(A, 0b1u32);
        }

        #[test]
        fn last() {
            const A: u32 = bitmask!(31u32..);
            assert_eq!(A, 0b10000000000000000000000000000000u32);
        }
    }
}

#[cfg(test)]
mod closed {
    #[cfg(test)]
    mod explicit {
        use bitmask::bitmask;

        #[test]
        fn all() {
            const A: u32 = bitmask!(0..=31);
            assert_eq!(A, 0b11111111111111111111111111111111u32);
        }

        #[test]
        fn arbitrary() {
            const A: u32 = bitmask!(2..=9);
            assert_eq!(A, 0b1111111100u32);
        }

        #[test]
        fn first() {
            const A: u32 = bitmask!(0..=0);
            assert_eq!(A, 0b1u32);
        }

        #[test]
        fn last() {
            const A: u32 = bitmask!(31..=31);
            assert_eq!(A, 0b10000000000000000000000000000000u32);
        }
    }

    #[cfg(test)]
    mod implicit {
        use bitmask::bitmask;

        #[test]
        fn all() {
            const A: u32 = bitmask!(..=31);
            assert_eq!(A, 0b11111111111111111111111111111111u32);
        }

        #[test]
        fn arbitrary() {
            const A: u32 = bitmask!(..=9);
            assert_eq!(A, 0b1111111111u32);
        }

        #[test]
        fn first() {
            const A: u32 = bitmask!(..=0);
            assert_eq!(A, 0b1u32);
        }
    }
}
