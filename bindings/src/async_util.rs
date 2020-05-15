use std::sync::{Arc, Condvar, Mutex};

use crate::generated::windows::foundation::{
    AsyncActionCompletedHandler, AsyncActionWithProgressCompletedHandler,
    AsyncOperationCompletedHandler, AsyncOperationWithProgressCompletedHandler, AsyncStatus,
    IAsyncAction, IAsyncActionWithProgress, IAsyncInfo, IAsyncOperation,
    IAsyncOperationWithProgress,
};
use winrt::{Result, RuntimeType, TryInto};

/// Extension for `IAsyncAction` with helper method.
pub trait RtAsyncAction {
    /// Waits for the asynchronous action to complete, blocking the current thread.
    fn blocking_wait(&self);
}

/// Extension for `IAsyncOperation` with helper methods.
pub trait RtAsyncOperation: RtAsyncAction {
    type TResult;

    fn get_results(&self) -> Result<Self::TResult>;

    #[inline]
    /// Waits for the asynchronous operation to complete, blocking the current thread,
    /// then return the result.
    fn blocking_get(&self) -> Result<Self::TResult> {
        self.blocking_wait();
        self.get_results()
    }
}

// The handler type is different for each interface, and the easiest way to share code seems to be a macro
macro_rules! impl_blocking_wait {
    ($handler:ident) => {
        #[inline]
        fn blocking_wait(&self) {
            let info: IAsyncInfo = self.try_into().expect("query_interface failed");
            let status = info.status().expect("get_status failed");

            if status == AsyncStatus::Completed {
                return;
            }

            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            {
                let pair2 = pair.clone();
                let handler = $handler::new(move |_op, _status| {
                    let &(ref lock, ref cvar) = &*pair2;
                    let mut completed = lock.lock().expect("lock failed");
                    *completed = true;
                    cvar.notify_one();
                    Ok(())
                });
                self.set_completed(&handler).expect("set_completed failed");
                // local reference to `handler` is dropped here -> Release() is called
            }

            // use condvar to wait until handler has been called
            let &(ref lock, ref cvar) = &*pair;
            let mut completed = lock.lock().expect("lock failed");
            while !*completed {
                completed = cvar.wait(completed).expect("wait failed");
            }
        }
    };
}

impl RtAsyncAction for IAsyncAction {
    impl_blocking_wait! { AsyncActionCompletedHandler }
}

impl<P: RuntimeType> RtAsyncAction for IAsyncActionWithProgress<P> {
    impl_blocking_wait! { AsyncActionWithProgressCompletedHandler }
}

impl<T: RuntimeType> RtAsyncAction for IAsyncOperation<T> {
    impl_blocking_wait! { AsyncOperationCompletedHandler }
}

impl<T: RuntimeType> RtAsyncOperation for IAsyncOperation<T> {
    type TResult = T;

    #[inline]
    fn get_results(&self) -> Result<Self::TResult> {
        self.get_results()
    }
}

impl<T: RuntimeType, P: RuntimeType> RtAsyncAction for IAsyncOperationWithProgress<T, P> {
    impl_blocking_wait! { AsyncOperationWithProgressCompletedHandler }
}

impl<T: RuntimeType, P: RuntimeType> RtAsyncOperation for IAsyncOperationWithProgress<T, P> {
    type TResult = T;

    #[inline]
    fn get_results(&self) -> Result<T> {
        self.get_results()
    }
}
