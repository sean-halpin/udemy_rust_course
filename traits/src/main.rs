mod drop_example;
mod dynamic_dispatch;
mod into_example;
mod op_overloading;
mod static_dispatch;
mod traits;
mod vectors_of_diff_objects;

fn main() {
    traits::traits();
    into_example::into_example();
    drop_example::drop_example();
    op_overloading::op_overloading();
    static_dispatch::static_dispatch();
    dynamic_dispatch::dynamic_dispatch();
    vectors_of_diff_objects::vectors_of_diff_objects();
}
