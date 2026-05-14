use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    sync::{LazyLock, Mutex},
    thread::{self, ThreadId},
};

use super::into_lifetime::IntoLifeTime;

static THREADS: LazyLock<Mutex<HashMap<ThreadId, Context>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct Context {
    values: HashMap<TypeId, Box<dyn Any>>,
}

unsafe impl Send for Context {}

impl Context {
    pub fn current<'r>() -> &'r mut Self {
        let mut mx = THREADS.lock().unwrap();
        let thread = thread::current().id();
        mx.entry(thread).or_insert(Self::new()).into_lifetime()
    }

    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, val: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let leak = Box::new(val);
        self.values
            .insert(type_id, leak)
            .map(|leak| *(leak.downcast::<T>().unwrap()))
    }

    pub fn take<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.values
            .remove(&type_id)
            .map(|leak| *(leak.downcast::<T>().unwrap()))
    }

    pub fn get<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.values
            .get_mut(&type_id)
            .map(|leak| leak.downcast_mut::<T>().unwrap().into_lifetime())
    }

    pub fn set_mut<T>(&mut self, val: &mut T) -> MutGuard<T> {
        self.set(val.into_lifetime());
        MutGuard {
            ctx: self.into_lifetime(),
            borrow_of: PhantomData,
        }
    }
}

pub struct MutGuard<T: 'static> {
    ctx: &'static mut Context,
    borrow_of: PhantomData<T>,
}

impl<T> Drop for MutGuard<T> {
    fn drop(&mut self) {
        self.ctx.take::<&mut T>();
    }
}
