mod traits;
mod into_example;
mod drop_example;
mod op_overloading;
mod static_dispatch;

fn main() {
    traits::traits();
    into_example::into_example();
    drop_example::drop_example();
    op_overloading::op_overloading();
    static_dispatch::static_dispatch();
}
