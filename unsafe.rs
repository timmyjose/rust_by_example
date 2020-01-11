use std::slice;

fn main() {
    let x = 100;
    let ptr_x = &x as *const i32;

    unsafe {
        assert_eq!(*ptr_x, 100);
    }

    let mut y = 100;
    let ptr_y = &mut y as *mut i32;

    unsafe {
        *ptr_y += 100;
    }

    assert_eq!(unsafe { *ptr_y }, 200);

    let some_vector = vec![1, 2, 3, 4, 5];
    let ptr = some_vector.as_ptr();
    let len = some_vector.len();

    unsafe {
        let my_slice = slice::from_raw_parts(ptr, len);
        assert_eq!(my_slice, some_vector.as_slice());
    }
}
