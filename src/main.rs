use clap::{App, Args, Arg, value_parser};

/**
 *
 * our expect effect:   rsc -r {root_path} -f {file_name}
 *
 * 01. 首先实现一个简单的目标，根据文件名当前目录寻找文件并打印到屏幕上
 * input:
 *  rsc {filename}
 * output:
 *  /Users/uuuu/develop/ascklrt-search/{filename}...
 *
 */
fn main() {
    let matches = App::new("ascklrt-search")
        .version("1.0.1")
        .author("rast")
        .arg(
            // 指定root_path
            Arg::new("rootpath")
                .short('r')
                .long("rootpath")
                .takes_value(true)
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            // 指定文件名参数
            Arg::new("file")
                .short('n')
                .long("file")
                .takes_value(true)
                .value_parser(clap::value_parser!(String))
        ).get_matches();


    // 使用get_one函数必须保证类型和上面的 .value_parser一致
    // Users have to match the type from Arg::value_parser with ArgMatches::get_one::<T>
    // https://github.com/clap-rs/clap/issues/3792
    let filename = match matches.get_one::<String>("file") {
        Some(f) => f,
        None => panic!("param -f filename must exist，you can see --help"),
    };

    println!("filename: {}", filename);
}
