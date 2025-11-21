use pqc_nostd::run_post;

fn main() {
    println!("Starting FIPS 140-3 POST...");
    match run_post() {
        Ok(_) => {
            println!("POST Passed! Module is Operational.");
        }
        Err(e) => {
            println!("POST Failed: {:?}", e);
            std::process::exit(1);
        }
    }
}
