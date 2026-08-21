#[macro_export]
macro_rules! trait_alias {
    ($vis:vis trait $alias:ident$(<$($generics:ident),*>)? = $($super_traits:tt)+) => {

        $vis trait $alias$(<$($generics),*>)?: $($super_traits)+ {}

        impl<__ඞඞT: $($super_traits)+, $($($generics),*)?> $alias$(<$($generics),*>)? for __ඞඞT {}
    };
}
