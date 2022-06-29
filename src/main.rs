use std::env;

fn main() {

    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();
    if &args[1] == "-h" {
        print!("{}", version);
    } else {
        let _from = &args[1];
        let ldd = &args[2];
        let _file_path = _get_file_path(ldd.to_string());
        // COPY --from=base /tmp/tagparam /bin/tagparam
        //println!("COPY from={} - {}", from, ldd);
    }
}

fn _get_file_path(_input:String){

    let input = _input.trim();
    let delim = "=>";
    // let delim2 = " ";
    let _result: Vec<&str> = input.split(delim).collect();
    //let _next_result: Vec<String> = _result.split(delim2).collect();
    // let _filter1 = _result[1].trim();
    //let _filter2 = _filter1.to_string().replace_range(0..4, "");

    // let removed: String = foo
    // .chars()
    // .take(start)
    // .chain(foo.chars().skip(stop))
    // .collect();


    print!("{}",_result[1].trim());
    // turn this:
    //libtheoraenc.so.1 => /usr/lib/x86_64-linux-gnu/libtheoraenc.so.1 (0x00007f7f2a453000)
    // into this:
    // /usr/lib/x86_64-linux-gnu/libtheoraenc.so.1 /usr/lib/x86_64-linux-gnu/libtheoraenc.so.1
    // and return
}