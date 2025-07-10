#[const_trait]
pub trait ConstInto<T>: Sized {
    fn const_into(self) -> T;
}

impl<T: Sized> const ConstInto<T> for T {
    fn const_into(self) -> T {
        self
    }
}

