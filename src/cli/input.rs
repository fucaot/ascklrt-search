use clap::{App, Arg, ArgMatches};

pub fn build_cli_param() -> ArgMatches {
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
                .short('f')
                .long("file")
                .takes_value(true)
                .value_parser(clap::value_parser!(String))
        ).get_matches();
    matches
}
