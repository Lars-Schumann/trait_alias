#[macro_export]
macro_rules! trait_alias {
    ($vis:vis trait $alias_name:ident$(<$($generics:ident),*>)? = $($super_traits:tt)+) => {

        $vis trait $alias_name$(<$($generics),*>)?: $($super_traits)+ {}

        impl<__ඞඞT: $($super_traits)+, $($($generics),*)?> $alias_name$(<$($generics),*>)? for __ඞඞT {}
    };
}
