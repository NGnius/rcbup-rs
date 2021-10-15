use argh::{FromArgs, from_env};
use libfj::robocraft::{FactoryAPI, AuthenticatedTokenProvider};

#[derive(FromArgs, Clone)]
#[argh(description = "Download robots from robocraft's factory.

MIT License
https://github.com/NGnius/rcbup-rs")]
pub struct CliArguments {
    #[argh(positional, arg_name = "search", description = "search text")]
    pub search: Option<String>,
    #[argh(option, arg_name = "password", short = 'p', description = "password for robocraft (non-steam)")]
    pub password: Option<String>,
    #[argh(option, arg_name = "username", short = 'u', description = "username for robocraft (non-steam)")]
    pub username: Option<String>,
    #[argh(option, arg_name = "out", description = "output folder for downloads")]
    pub out: Option<String>,
    #[argh(switch, arg_name = "batch", short = 'l', description = "slow down and handle chores for longer batches")]
    pub batch: bool,
    #[argh(switch, arg_name = "player", description = "search by player usernames only")]
    pub player: bool,
    #[argh(option, arg_name = "extension", description = "file extension for robot JSON files")]
    pub extension: Option<String>,
    #[argh(option, arg_name = "max", short = 'm', description = "maximum robots to download")]
    pub max: Option<usize>,
    #[argh(switch, arg_name = "thumbnail", short = 'i', description = "download robot thumbnail images too")]
    pub thumbnail: bool,
}

impl CliArguments {
    pub fn from_env() -> Self {
        let args: Self = from_env();
        args.validate();
        args
    }
    
    pub fn empty(&self) -> bool {
        self.search.is_none()
        && self.password.is_none()
        && self.username.is_none()
        && self.out.is_none()
        && !self.batch
        && self.extension.is_none()
        && self.max.is_none()
        && !self.thumbnail
    }
    
    fn validate(&self) {
        // validate args and panic on invalid input
        if self.password.is_some() != self.username.is_some() {
            eprint_then_panic("password and username must be specified together");
        }
    }
    
    pub fn configure_api(&self) -> Result<FactoryAPI, ()> {
        if self.username.is_some() && self.password.is_some() {
            // configure with auth
            let auth_result = AuthenticatedTokenProvider::with_username(&self.username.clone().unwrap(), &self.password.clone().unwrap());
            if let Ok(auth) = auth_result {
                Ok(FactoryAPI::with_auth(Box::new(auth)))
            } else {
                eprintln!("Authentication error: {}", auth_result.err().unwrap());
                Err(())
            }
        } else {
            // configure with default auth
            Ok(FactoryAPI::new())
        }
    }
}

fn eprint_then_panic(msg: &str) -> !{
    eprintln!("{}", msg);
    panic!("{}", msg);
}
