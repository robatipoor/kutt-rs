use clap::{App, Arg, ArgMatches};

#[derive(Debug, Default)]
pub struct AppArgs {
    pub target_url: Option<String>,
    pub custom_url: Option<String>,
    pub password: Option<String>,
    pub reuse: bool,
    pub login: Option<String>,
    pub delete: Option<String>,
}

impl AppArgs {
    pub fn get_app_args() -> AppArgs {
        let matches: ArgMatches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::with_name("target-url")
                    .short("t")
                    .long("target")
                    .value_name("URL")
                    .help("Set a url")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("custom-url")
                    .short("c")
                    .long("create")
                    .value_name("PATHFILE")
                    .help("Set a path text file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("delete")
                    .short("d")
                    .long("delete")
                    .value_name("URL")
                    .help("Set a url")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("password")
                    .short("p")
                    .long("password")
                    .value_name("PASSWORD")
                    .help("Set a password")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("login")
                    .short("l")
                    .long("login")
                    .value_name("API-KEY")
                    .help("Set a API-KEY")
                    .takes_value(true),
            )
            .get_matches();

        let mut app_args: AppArgs = AppArgs::default();
        if let Some(l) = matches.value_of("login") {
            app_args.login = Some(l.to_owned());
            return app_args;
        }
        if let Some(u) = matches.value_of("target-url") {
            app_args.target_url = Some(u.to_owned());
            if let Some(u) = matches.value_of("custom-url") {
                app_args.custom_url = Some(u.to_owned());
            }
        } else if let Some(u) = matches.value_of("delete") {
            app_args.delete = Some(u.to_owned());
        }
        if let Some(p) = matches.value_of("password") {
            app_args.password = Some(p.to_owned());
        }
        return app_args;
    }
}
