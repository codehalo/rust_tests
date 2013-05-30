fn main() {
    let path = path::PosixPath("/Users/rj46465/Development/rust/accounts.rsf");

    let fr_res:Result<@Reader, ~str> = io::file_reader(&path);

    if fr_res.is_ok() {
		let rfile = fr_res.get();
        println("\n\n--------------------- READ BACK.");
        println(fmt!("Account: %u", rfile.read_be_uint()));
        println(fmt!("Name: %s", rfile.read_until('\u0000', false)));
        //rfile.read_u8();
        println(fmt!("Balance: %s", rfile.read_be_f64().to_str()));
        println("---------------------");
    } else {
        println(fmt!("Cant open %s", path.to_str()));
    }    

}
