use std::{
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
    ops::DerefMut,
    sync::{LazyLock, Mutex},
    thread::{self, ThreadId},
};

use super::{into_lifetime::IntoLifeTime, into_variant::IntoOption, leak_box::LeakBox};

static THREADS: LazyLock<Mutex<HashMap<ThreadId, Context>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static GLOBAL: LazyLock<Mutex<Context>> = LazyLock::new(|| Mutex::new(Context::new()));

pub struct Context {
    values: HashMap<TypeId, LeakBox>,
}

impl Context {
    pub fn current<'r>() -> &'r mut Self {
        let mut mx = THREADS.lock().unwrap();
        let thread = thread::current().id();
        mx.entry(thread).or_insert(Self::new()).into_lifetime()
    }

    pub fn global<'r>() -> &'r mut Self {
        let mut mx = GLOBAL.lock().unwrap();
        mx.deref_mut().into_lifetime()
    }

    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, val: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let leak = LeakBox::from(Box::new(val));
        let out = self
            .values
            .insert(type_id, leak.cast_to())?
            .cast_to::<T>()
            .into_box();
        Some(*out)
    }

    pub fn take<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let out = self.values.remove(&type_id)?.cast_to::<T>().into_box();
        Some(*out)
    }

    pub fn get<'r, T: 'static>(&mut self) -> Option<&'r mut T> {
        let type_id = TypeId::of::<T>();
        self.values
            .get_mut(&type_id)?
            .assert::<T>()
            .must_mut()
            .Some()
    }

    pub fn set_then_get<'r, T: 'static>(&mut self, val: T) -> &'r mut T {
        self.set(val);
        self.get::<T>().unwrap().into_lifetime()
    }

    /// get<T>() if not exist then get<&mut T>
    pub fn get_cow<T: 'static>(&mut self) -> Option<&mut T> {
        if let Some(owned) = self.get::<T>() {
            return owned.into_lifetime().Some();
        };
        let brrow = &mut **self.get::<&mut T>()?;
        brrow.into_lifetime().Some()
    }

    pub fn set_mut<T>(&mut self, val: &mut T) -> MutGuard<T> {
        self.set(val.into_lifetime());
        MutGuard {
            ctx: self.into_lifetime(),
            borrow_of: PhantomData,
        }
    }
}

unsafe impl Send for Context {}

pub struct MutGuard<T: 'static> {
    ctx: &'static mut Context,
    borrow_of: PhantomData<T>,
}

impl<T> Drop for MutGuard<T> {
    fn drop(&mut self) {
        self.ctx.take::<&mut T>();
    }
}
