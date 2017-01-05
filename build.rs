extern crate regex;
extern crate svd2rust;
extern crate svd_parser as svd;
extern crate inflections;
#[macro_use]
extern crate quote;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use inflections::Inflect;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("svd.rs");
    let mut f = File::create(&dest_path).unwrap();

    let file = "nrf51.svd";

    println!("cargo:rerun-if-changed={}", file);

    let xml = &mut String::new();
    File::open(file).unwrap().read_to_string(xml).unwrap();

    let d = svd::parse(xml);
    for peripheral in &d.peripherals {
        if peripheral.derived_from.is_some() {
            continue;
        }

        let lower = quote::Ident::new(peripheral.name.to_lowercase());
        let title = quote::Ident::new(peripheral.name.to_pascal_case());
        let upper = quote::Ident::new(peripheral.name.as_str());
        let address = quote::Hex(peripheral.base_address as usize);
        let peripheral = svd2rust::gen_peripheral(peripheral, &d.defaults);

        writeln!(f,
               "{}",
               quote!(
                    pub mod #lower {
                        #(#peripheral)*
                    }

                    const #upper: usize = #address;

                    pub unsafe fn #lower () -> &'static mut #lower::#title {
                        &mut *(#upper as *mut #lower::#title)
                    }
               )
                   .as_str())
            .unwrap();
    }
}
