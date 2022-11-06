use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Permission scanner",
    about = "Scan a directory for files that match permission criteria. \nVisit https://github.com/Pythack/permscan#readme for more information. "
)]
pub struct Opt {
    #[structopt(
        long,
        help = "Specify permissions that the user who owns the file or directory needs to have on the item in the format @rwx"
    )]
    pub user: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that the group who owns the file or directory needs to have on the item in the format @rwx"
    )]
    pub group: Option<String>,

    #[structopt(
        long,
        help = "Specify permissions that users who does not own the file or directory needs to have on the item in the format @rwx"
    )]
    pub other: Option<String>,

    #[structopt(long, help = "Specify the owner of the file in the format user:group")]
    pub owner: Option<String>,

    #[structopt(long = "type", help = "Specify the type of the object")]
    pub item_type: Option<String>,

    #[structopt(short, help = "If present, permscan will recursively traverse the folder")]
    pub recursive: bool,

    #[structopt(
        short,
        help = "Return files that match at least one criteria, instead of those that match all criteria"
    )]
    pub merge: bool,

    #[structopt(short, help = "Return the list of files that don't match with the criteria")]
    pub invert: bool,

    #[structopt(short, help = "Parse hidden files as well")]
    pub all: bool,

    #[structopt(short = "u", long = "update", help = "Check for a newer version of permscan")]
    pub check_update: bool,

    #[structopt(
        short = "b",
        long = "build",
        help = "If the update flag is also present and the user decide to update, the update will be built from source"
    )]
    pub build: bool,

    #[structopt(default_value = "./", help = "The path of the directory your want to look into.")]
    pub path: String,
}
