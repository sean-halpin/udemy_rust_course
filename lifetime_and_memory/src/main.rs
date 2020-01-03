mod borrowing;
mod lifetimes;
mod lifetimes_in_structure_impl;
mod ownership;
mod ref_counted_variables;

fn main() {
    ownership::ownership();
    borrowing::borrowing();
    lifetimes::lifetimes();
    lifetimes_in_structure_impl::lifetimes_in_structure_impl();
    ref_counted_variables::ref_counted_variables();
}
