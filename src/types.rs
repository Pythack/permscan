// Since all but one of our results are under the form
// Result<T, Box<dyn std::error::Error>>, we create our
// custom type for that
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Permscan stores it's results differently depending on if the merge flag is
// passed or not. This enum allow us to treat those results the same way
pub enum PermscanOutput<'a> {
    Merge(Vec<&'a str>),
    NoMerge(Vec<Vec<&'a str>>),
}
