// use std::{cell::RefCell, rc::Rc};

// use super::EngineHandler;

// pub trait NativeManager<ManagedType> {
//     fn destroy(&mut self, p: ManagedType);
// }

// pub struct NativeManaged<ManagerType, ManagedType>
// where
//     ManagerType: NativeManager<ManagedType>,
// {
//     pub(crate) manager: Rc<RefCell<ManagerType>>,
//     pub data: ManagedType,
// }

// impl<ManagerType, PtrType> NativeManaged<ManagerType, *mut PtrType>
// where
//     ManagerType: NativeManager<*mut PtrType>,
// {
//     pub fn try_from_ptr(manager: Rc<RefCell<ManagerType>>, ptr: *mut PtrType) -> Option<Self> {
//         if ptr.is_null() {
//             None
//         } else {
//             Some(Self {
//                 manager: manager,
//                 data: ptr,
//             })
//         }
//     }
// }

// impl<ManagerType, ManagedType> Drop for NativeManaged<ManagerType, ManagedType>
// where
//     ManagerType: NativeManager<ManagedType>,
// {
//     fn drop(&mut self) {
//         self.manager.borrow_mut().destroy(self.data);
//     }
// }

// pub type EngineManaged<ManagedType> = NativeManaged<EngineHandler, ManagedType>;
