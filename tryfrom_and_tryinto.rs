use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value % 2 {
            0 => Ok(EvenNumber(value)),
            _ => Err(()),
        }
    }
}

fn main() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(7), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let another_result: Result<EvenNumber, ()> = 7i32.try_into();
    assert_eq!(another_result, Err(()));
}
