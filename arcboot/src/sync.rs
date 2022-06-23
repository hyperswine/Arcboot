use core::cell::UnsafeCell;

pub trait Mutex {
    /// The type of the data that is wrapped by this mutex
    type Data;

    /// Locks the mutex and grants the closure temporary mutable access to the wrapped data
    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
}

/// A reader-writer exclusion type
pub trait ReadWriteEx {
    /// The type of encapsulated data
    type Data;

    /// Grants temporary mutable access to the encapsulated data
    fn write<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;

    /// Grants temporary immutable access to the encapsulated data
    fn read<'a, R>(&'a self, f: impl FnOnce(&'a Self::Data) -> R) -> R;
}

/// A pseudo-lock for when executing on a single core
pub struct IRQSafeNullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

/// A pseudo-lock that is RW during the single-core kernel init phase and RO afterwards
pub struct InitStateLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for IRQSafeNullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for IRQSafeNullLock<T> where T: ?Sized + Send {}

impl<T> IRQSafeNullLock<T> {
    /// Create an instance.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

unsafe impl<T> Send for InitStateLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for InitStateLock<T> where T: ?Sized + Send {}

impl<T> InitStateLock<T> {
    /// Create an instance.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

// PUT INTO KERNEL?
// ARCBOOT ONLY NEEDS TO SETUP PAGING AND RETURN THE BASE PHYS ADDR OR VIRT ADDR OF THE PAGE TABLES
// AND MAYBE WRAP IT

// but uh... whats a way of doing that?

// impl<T> Mutex for IRQSafeNullLock<T> {
//     type Data = T;

//     fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
//         // NOTE: not a real lock
//         let data = unsafe { &mut *self.data.get() };

//         // Execute the closure while IRQs are masked.
//         exception::asynchronous::exec_with_irq_masked(|| f(data))
//     }
// }

// impl<T> interface::ReadWriteEx for InitStateLock<T> {
//     type Data = T;

//     fn write<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
//         assert!(
//             state::state_manager().is_init(),
//             "InitStateLock::write called after kernel init phase"
//         );
//         assert!(
//             exception::asynchronous::is_local_irq_masked(),
//             "InitStateLock::write called with IRQs unmasked"
//         );

//         let data = unsafe { &mut *self.data.get() };

//         f(data)
//     }

//     fn read<'a, R>(&'a self, f: impl FnOnce(&'a Self::Data) -> R) -> R {
//         let data = unsafe { &*self.data.get() };

//         f(data)
//     }
// }
