macro_rules! trait_alias {
    ($vis:vis trait $alias_name:ident = $($super_traits:tt)+) => {

        $vis trait $alias_name: $($super_traits)+ {}

        impl<T: $($super_traits)+> $alias_name for T {}
    };
}

trait_alias! { pub trait SendSyncStatic = Send + Sync + 'static }

#[cfg(test)]
mod tests {
    fn _goob<T: SendSyncStatic>(value: T) -> impl Send + Sync + 'static {
        value
    }
    use super::*;
}
