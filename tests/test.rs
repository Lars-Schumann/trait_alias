use trait_alias::trait_alias;

trait_alias! { pub trait SendSyncStatic = Send + Sync + 'static }

fn _simple<T: SendSyncStatic>(value: T) -> impl Send + Sync + 'static {
    value
}
