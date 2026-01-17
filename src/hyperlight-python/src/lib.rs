use hyperlight_host::func::HostFunction;

pub mod sandbox;

pub type HostPrintFn = HostFunction<i32, (String,)>;
