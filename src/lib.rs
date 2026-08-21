#[macro_export]
macro_rules! trait_alias {
    ($vis:vis trait $alias_name:ident = $($super_traits:tt)+) => {

        $vis trait $alias_name: $($super_traits)+ {}

        impl<T: $($super_traits)+> $alias_name for T {}
    };
}