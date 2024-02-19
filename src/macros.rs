#[macro_export]
macro_rules! only_privileged {
    ($trait_self: expr, $error_msg:expr) => {
        let caller = $trait_self.blockchain().get_caller();
        if !$trait_self.is_privileged(&caller) {
            sc_panic!($error_msg);
        }
    };
}
