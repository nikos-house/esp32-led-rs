pub trait Updatable<T> {
    fn subscribe_to_updates(&self, cb: Box<dyn Fn(&T)>);
}

pub trait Pollable<T> {
    fn poll(&self);
}

#[warn(unused_macros)]
#[macro_export]
macro_rules! rc {
    ($val:tt) => {
        Rc::new(RefCell::new($val))
    };
}
