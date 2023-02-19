#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SlideNumber(u32);

impl SlideNumber {
    pub fn next(&self, slide_count: u32) -> SlideNumber {
        let next_slide_number = self.0 % slide_count + 1;
        log::debug!("Next SlideNumber after {} is {}", self.0, next_slide_number);
        SlideNumber(next_slide_number)
    }
}

impl From<u32> for SlideNumber {
    fn from(value: u32) -> Self {
        SlideNumber(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod next {
        use super::*;

        #[test]
        fn returns_slide_number_with_incremented_number() {
            let slide_count = 3;
            let slide_number = SlideNumber(1);

            let next_slide_number = slide_number.next(slide_count);

            assert_eq!(next_slide_number, SlideNumber(2));
        }

        #[test]
        fn returns_slide_number_with_number_equal_to_slide_count_when_slide_number_is_equal_to_slide_count(
        ) {
            let slide_count = 3;
            let slide_number = SlideNumber(slide_count - 1);

            let next_slide_number = slide_number.next(slide_count);

            assert_eq!(next_slide_number, SlideNumber(slide_count));
        }

        #[test]
        fn returns_slide_number_with_number_1_when_slide_number_is_larger_than_slide_count() {
            let slide_count = 3;
            let slide_number = SlideNumber(slide_count);

            let next_slide_number = slide_number.next(slide_count);

            assert_eq!(next_slide_number, SlideNumber(1));
        }
    }
}
