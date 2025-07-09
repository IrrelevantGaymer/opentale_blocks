#![feature(const_trait_impl)]

pub mod block_types;

#[const_trait]
pub trait Buildable {
    fn get_size() -> usize;
    fn with_index(self, idx: usize) -> Self;
}

#[macro_export]
macro_rules! table {
    (
        $slice:path, 
        static $table:ident = {
            $(let $block_name:ident : $block_type:ty = $block_expr:expr ;)*
        }
    ) => {
        $crate::assert_items_define!(
            $slice, 
            $($block_name : $block_type),*
        );
        $crate::items_define!(
            1, 
            $($block_name : $block_type = $block_expr),*
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
        $slice:path, 
        $first_name:ident : $first_type:ty
        $(, $rest_name:ident : $rest_type:ty)*
    ) => {
        static_assertions::assert_impl_all!($first_type: $crate::Buildable, $slice);
        $crate::assert_items_define!(
            $slice, 
            $($rest_name : $rest_type),*
        );
    };
    ($slice:path, ) => {

    }
}

#[macro_export]
macro_rules! items_define {
    (
        $idx:expr, 
        $first_name:ident : $first_type:path = $first_expr:expr 
        $(, $rest_name:ident : $rest_type:path = $rest_expr:expr)*
    ) => {
        static $first_name: $first_type = <$first_type as $crate::Buildable>::with_index($first_expr, $idx);
        $crate::items_define!(
            $idx + <$first_type as $crate::Buildable>::get_size(), 
            $($rest_name : $rest_type = $rest_expr),*
        );
    };
    ($idx:expr, ) => {
        
    };
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