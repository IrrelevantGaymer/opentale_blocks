pub struct Table<T: 'static + ?Sized>(pub &'static [&'static T]);

impl<T: 'static + ?Sized> Table<T> {
    pub fn iter(&self) -> impl Iterator<Item = &'static T> + '_ {
        self.0.iter().copied()
    }
}

impl<T: 'static + ?Sized> IntoIterator for &Table<T> {
    type Item = &'static T;

    type IntoIter = std::iter::Copied<std::slice::Iter<'static, &'static T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}