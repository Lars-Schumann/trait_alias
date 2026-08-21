use trait_alias::trait_alias;

trait_alias! { pub trait SendSyncStatic = Send + Sync + 'static }

fn _simple<T: SendSyncStatic>(value: T) -> impl Send + Sync + 'static {
    value
}

trait_alias! { pub trait FromInto<T> = From<T> + Into<T> }

fn _generic<T: FromInto<i32>>(value: i32) -> i32 {
    let t: T = value.into();
    t.into()
}
