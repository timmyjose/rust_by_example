#![allow(dead_code)]

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let current_age = current_age?;
    Some(format!("{}", current_age))
}

struct Person {
    job: Option<Job>,
}

struct Job {
    phone_number: Option<PhoneNumber>,
}

struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job.as_ref()?.phone_number.as_ref()?.area_code
    }
}

fn main() {
    let valid_current_age = Some(12);
    println!("{:?}", next_birthday(valid_current_age));

    let invalid_age = None;
    assert!(next_birthday(invalid_age).is_none());

    let bob = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(123),
                number: 12345,
            }),
        }),
    };

    assert_eq!(bob.work_phone_area_code(), Some(123));

    let sam = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: None,
                number: 54321,
            }),
        }),
    };

    assert!(sam.work_phone_area_code().is_none());
}
