use std::any::Any;

use sutils::LeakBox;

#[test]
fn test_fat_pointer_leak_box() {
    let mut val_1 = [1u8, 2u8];
    let boxed: Box<dyn Any> = Box::new(val_1.clone());
    let leak: LeakBox<_> = boxed.into();
    let boxed = leak.into_box();
    let val_2 = boxed.downcast_ref::<[u8; 2]>().unwrap();

    assert!(val_1 == *val_2);

    let val_3 = &mut val_1[..];
    let boxed: Box<[u8]> = unsafe { Box::from_raw(val_3 as *mut _) };
    let leak: LeakBox<_> = boxed.into();
    let val_3 = leak.get_mut();

    assert!(val_1 == val_3);
    val_1[0] += 1;
    assert!(val_1 == val_3);
    val_1[1] *= 2;
    assert!(val_1 == val_3);
}
