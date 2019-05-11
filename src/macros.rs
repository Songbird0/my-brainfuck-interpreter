/// A light macro debugging.
///
/// The implementation isn't magic, it simply
/// uses `dbg!` on `debug` build only.
///
#[macro_export]
macro_rules! ldbg {
    ($val:expr) => {
        if cfg!(debug_assertions) {
            let lock = &$val;
            ::std::dbg!(lock);
        }
    }
}
