pub mod mod_b;

pub fn print_a_to_Z() {
    loop {
        for i in ('Z'..='a').rev() {
            println!("{i}");
        }
    }
}

pub fn r_mod_b() {
    mod_b::print_A_to_z_2();
}