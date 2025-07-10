#![feature(const_trait_impl)]

pub mod block_types;

#[const_trait]
pub trait Buildable<T: Indexable> {
    fn get_size() -> usize;
    fn with_index(self, idx: T) -> Self;
}

#[const_trait]
pub trait Indexable: Clone + Copy + Send + Sync {
    fn value(&self) -> usize;
    fn default() -> Self;
}

#[macro_export]
macro_rules! table {
    (
        $slice:path, 
        enum $enum_name:ident,
        static $table:ident = {
            $(let $block_name:ident : $block_type:ident = $block_expr:expr ;)*
        }
    ) => {
        $crate::assert_items_define!(
            $slice, $enum_name,
            $($block_name : $block_type),*
        );
        $crate::items_define!(
            $enum_name,
            $($block_name : $block_type = $block_expr),*
        );
        $crate::items_enum!(
            $enum_name,
            $($block_name : $block_type),*
        );
        $crate::table_define!(
            $table, $slice, 
            $($block_name),*
        );
    };
}

#[macro_export]
macro_rules! assert_items_define {
    (
        $slice:path, $enum_name:ident,
        $first_name:ident : $first_type:ident
        $(, $rest_name:ident : $rest_type:ident)*
    ) => {
        static_assertions::assert_impl_all!($crate::insert_indexable!($enum_name, $first_type): $crate::Buildable<$enum_name>, $slice);
        $crate::assert_items_define!(
            $slice, $enum_name,
            $($rest_name : $rest_type),*
        );
    };
    ($slice:path, $enum_name:ident, ) => {

    }
}

#[macro_export]
macro_rules! items_define {
    (
        $enum_name:ident,
        $first_name:ident : $first_type:ident = $first_expr:expr 
        $(, $rest_name:ident : $rest_type:ident = $rest_expr:expr)*
    ) => {
        #[allow(non_upper_case_globals)]
        static $first_name: $crate::insert_indexable!($enum_name, $first_type) = 
            <$crate::insert_indexable!($enum_name, $first_type) as $crate::Buildable<$enum_name>>::with_index(
                $first_expr, 
                $enum_name::$first_name
            );
        $crate::items_define!(
            $enum_name,
            $($rest_name : $rest_type = $rest_expr),*
        );
    };
    ($enum_name:ident,) => {
        
    };
}

#[macro_export]
macro_rules! insert_indexable {
    ($enum_name:ident, $ty:ident) => {
        $ty<$enum_name>
    };
    ($enum_name:ident, $ty:ident < $($args:ty),+ $(,)? >) => {
        $ty<$enum_name, $($args),*>
    };
}

#[macro_export]
macro_rules! items_enum {
    (
        $enum_name:ident,
        $($block_name:ident : $block_type:ident),*
    ) => {
        #[derive(Clone, Copy)]
        pub enum $enum_name {
            AIR,
            $($block_name),*
        }

        impl const $crate::Indexable for $enum_name {
            fn default() -> Self {
                $enum_name::AIR
            }
            
            fn value(&self) -> usize {
                $crate::items_enum_inner!(1; self, $enum_name, $($block_name : $block_type),*)
            }
        }
    }
}

#[macro_export]
macro_rules! items_enum_inner {
    (
        $idx:expr; $self:ident, $enum_name:ident,
        $first_name:ident : $first_type:ident
        $(, $rest_name:ident : $rest_type:ident)*
    ) => {
        if matches!($self, <$enum_name>::$first_name) {
            $idx
        } else {
            $crate::items_enum_inner!(
                $idx + <$crate::insert_indexable!($enum_name, $first_type) as $crate::Buildable<$enum_name>>::get_size();
                $self,
                $enum_name,
                $($rest_name : $rest_type),*
            )
        }
    };
    ($idx:expr; $self:ident, $enum_name:ident, ) => {
        0
    }
}

#[macro_export]
macro_rules! table_define {
    (
        $table:ident, $slice:path, 
        $($block_name:ident),*
    ) => {
        pub static $table : &'static [&'static (dyn $slice)] = &[
            $(& $block_name),*
        ];
    }
}