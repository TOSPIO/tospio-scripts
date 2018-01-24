extern crate libc;

mod affinity;
mod wrapper_facility;

fn main() {
    let cpu_set = affinity::get_affinity();
    affinity::assign_cpu(0);
    println!("{:?}", 123);
}
