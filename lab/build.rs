fn main() {
    println!("cargo:rerun-if-changed=src/parser/l1.lalrpop");
    lalrpop::process_root().unwrap();
}
