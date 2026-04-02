use std::{
    any::TypeId,
    collections::HashMap,
    sync::{LazyLock, Mutex},
    thread::{self, ThreadId},
};

use super::context_func::TypedInto;
use super::into_lifetime::IntoLifeTime;

use super::leak_box::LeakBox;

static THREADS: LazyLock<Mutex<HashMap<ThreadId, ThreadContext>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct ThreadContext {
    values: HashMap<TypeId, LeakBox<isize>>,
}

impl ThreadContext {
    pub fn current() -> &'static mut Self {
        let mut mx = THREADS.lock().unwrap();
        let thread = thread::current().id();
        mx.entry(thread)
            .or_insert(Self {
                values: HashMap::new(),
            })
            .into_lifetime()
    }

    pub fn set<T: 'static>(&mut self, val: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let leak = Box::new(val).inTo::<LeakBox<T>>();
        let old = self.get::<T>();
        self.values
            .insert(type_id, leak.cast_to())
            .map(|leak| *(leak.cast_to::<T>().into_box()));
        old
    }

    pub fn get<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.values
            .remove(&type_id)
            .map(|leak| *(leak.cast_to::<T>().into_box()))
    }
}
