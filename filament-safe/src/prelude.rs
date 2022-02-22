use std::{
    cell::UnsafeCell,
    rc::{Rc, Weak},
};

use crate::{
    filament::{Engine, WeakEngine},
    utils::EntityIdentify,
};

#[derive(Debug)]
pub struct RcHandle<T> {
    data: Rc<UnsafeCell<T>>,
}

impl<T> AsRef<T> for RcHandle<T> {
    fn as_ref(&self) -> &T {
        unsafe { &(*self.data.get()) }
    }
}

impl<T> AsMut<T> for RcHandle<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.data.get()) }
    }
}

impl<T> RcHandle<T> {
    pub fn new(data: T) -> Self {
        RcHandle {
            data: Rc::new(UnsafeCell::new(data)),
        }
    }

    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle {
            data: Rc::downgrade(&self.data),
        }
    }
}

impl<T> Clone for RcHandle<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

#[derive(Debug)]
pub struct WeakHandle<T> {
    data: Weak<UnsafeCell<T>>,
}

impl<T> WeakHandle<T> {
    pub fn new() -> Self {
        WeakHandle { data: Weak::new() }
    }

    pub fn upgrade(&self) -> Option<RcHandle<T>> {
        self.data.upgrade().map(|data| RcHandle { data })
    }
}

impl<T> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

pub trait NativeHandle<N> {
    fn native(&self) -> *const N;
    fn native_mut(&mut self) -> *mut N;
}

/// Data type, weak reference with engine, generally included in the reference count, automatically destroyed.
pub struct EngineData<T>
where
    T: EngineDrop,
{
    data: T,
    engine: WeakEngine,
}

impl<T> EngineData<T>
where
    T: EngineDrop,
{
    pub fn new(data: T, engine: WeakEngine) -> Self {
        EngineData { data, engine }
    }
}

impl<T> RcHandle<EngineData<T>>
where
    T: EngineDrop,
{
    pub fn data(&self) -> &T {
        &self.as_ref().data
    }
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.as_mut().data
    }

    pub fn engine(&self) -> &WeakEngine {
        &self.as_ref().engine
    }
}

impl<T> Drop for EngineData<T>
where
    T: EngineDrop,
{
    fn drop(&mut self) {
        if let Ok(mut engine) = self.engine.upgrade_engine() {
            self.data.drop(&mut engine);
        }
    }
}

/// Component type, no reference counting, Manually create or destroy on parent entity.
pub struct EngineComponent<T> {
    pub(crate) data: T,
    engine: WeakEngine,
    parent_identify: EntityIdentify,
}

impl<T> EngineComponent<T> {
    pub fn new(data: T, engine: WeakEngine, parent_identify: EntityIdentify) -> Self {
        EngineComponent {
            data,
            engine,
            parent_identify,
        }
    }

    pub fn engine(&self) -> Engine {
        self.engine.upgrade_engine().ok().unwrap()
    }
}

pub struct EngineSystem<T>
where
    T: EngineDrop,
{
    data: T,
    engine: Engine,
}

impl<T> EngineSystem<T>
where
    T: EngineDrop,
{
    pub fn new(data: T, engine: Engine) -> Self {
        EngineSystem { data, engine }
    }
}

impl<T> RcHandle<EngineSystem<T>>
where
    T: EngineDrop,
{
    pub fn data(&self) -> &T {
        &self.as_ref().data
    }
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.as_mut().data
    }

    pub fn engine(&self) -> &Engine {
        &self.as_ref().engine
    }
}

impl<T> Drop for EngineSystem<T>
where
    T: EngineDrop,
{
    fn drop(&mut self) {
        self.data.drop(&mut self.engine)
    }
}

pub trait EngineDrop {
    fn drop(&mut self, engine: &mut Engine);
}

#[derive(Debug)]
pub enum EngineError {
    EngineDestroyed,
    Unknown,
}

pub type EngineResult<T> = Result<T, EngineError>;
