#![feature(adt_const_params)]
#![feature(const_trait_impl)]

pub mod blocks;

#[const_trait]
pub trait Buildable {
    fn new_with_name(name: &'static str) -> Self;
    fn get_texture_size() -> usize;
    /// This is useful for blocks with multiple blockstates
    /// such as stairs, slabs, waterloggable blocks, rotateable blocks,
    /// connecting blocks, etc.  Most blocks don't have blockstates,
    /// so the default is 1.
    fn get_id_size() -> usize { 1usize }
    fn with_index(self, idx: usize) -> Self;
    fn set_index(&mut self, idx: usize);
    fn with_id(self, id: usize) -> Self;
    fn set_id(&mut self, id: usize);
}

#[macro_export]
macro_rules! table {
    (
        $slice:path, 
        enum $enum:ident,
        static $table:ident = {
            $(let $block_name:ident : $block_type:ty = $block_expr:expr ;)*
        }
    ) => {
        $crate::assert_items_define!(
            $slice, 
            $($block_name : $block_type),*
        );
        $crate::items_define!(
            1, 1,
            $($block_name : $block_type = $block_expr),*
        );

        $crate::table_define!(
            $table, $slice, 
            $($block_name),*
        );

        $crate::enum_define!(
            $enum,
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
        $id:expr, $idx:expr,
        $first_name:ident : $first_type:ty = $first_expr:expr 
        $(, $rest_name:ident : $rest_type:ty = $rest_expr:expr)*
    ) => {
        #[allow(non_upper_case_globals)]
        static $first_name: $first_type = 
            <$first_type as $crate::Buildable>::with_id(
                <$first_type as $crate::Buildable>::with_index(
                    $first_expr, 
                    $idx
                ),
                $id
            );
        $crate::items_define!(
            $id + <$first_type as $crate::Buildable>::get_id_size(),
            $idx + <$first_type as $crate::Buildable>::get_texture_size(),
            $($rest_name : $rest_type = $rest_expr),*
        );
    };
    ($id:expr, $idx:expr,) => {
        
    };
}

#[macro_export]
macro_rules! table_define {
    (
        $table_name:ident, $slice:path, 
        $($block_name:ident),*
    ) => {
        pub static $table_name : &'static [&'static (dyn $slice)] = &[
            $(& $block_name),*
        ];
    }
}

#[macro_export]
macro_rules! enum_define {
    ($enum_name:ident, $($block_name:ident),*) => {
        pub enum $enum_name {
            $($block_name),*
        }
    }
}