use std::{any::TypeId, collections::HashSet};

use sutils::{Context, IntoLifeTime};

#[test]
fn context_mut() {
    let context = Context::current();
    let mut x = 1u8;
    context.set((&mut x).into_lifetime());
    
    **context.get::<&mut u8>().unwrap() += 1;
    **context.get::<&'static mut u8>().unwrap() += 1;
    assert!(x == 3);

    assert!(
        HashSet::from([
            type_id_assert::<u8>(),
            type_id_assert::<&mut u8>(),
            type_id_assert::<&()>(),
            type_id_assert::<()>(),
        ])
        .len()
            == 4
    );

    fn type_id_assert<T: 'static>() -> TypeId {
        use std::any::TypeId;
        assert!(
            HashSet::from([
                TypeId::of::<T>(),
                TypeId::of::<&T>(),
                TypeId::of::<&mut T>(),
            ])
            .len()
                == 3
        );
        assert!(TypeId::of::<&T>() == TypeId::of::<&'static T>());
        assert!(TypeId::of::<&mut T>() == TypeId::of::<&'static mut T>());
        TypeId::of::<T>()
    }
}
