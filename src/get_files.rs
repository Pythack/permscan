use regex::Regex;

// functions to get files matching permissions criteria

pub fn get_based_on_owner(
    files: &str,
    owner: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{9}[ 0-9]* *")
            + &*owner
            + r" (.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_user(
    files: &str,
    user: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-]") + &user + r"[rwx\-]{6}(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_group(
    files: &str,
    group: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{3}")
            + &group
            + r"[rwx\-]{3}(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_other(
    files: &str,
    other: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();

    let re = Regex::new(
        &(String::from(r"^[dlcbps\-][rwx\-]{6}") + &other + r"(.|\n)*$"),
    )
    .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_based_on_type(
    files: &str,
    file_type: String,
    invert: bool,
    recursive: bool,
) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    let re =
        Regex::new(&(String::from(r"^") + &file_type + r"[rwx\-]{9}(.|\n)*$"))
            .unwrap();
    let sub_dir = Regex::new(&String::from(r"^(.+)/*([^/]+)*:$")).unwrap();
    for line in lines.skip(1) {
        let line = String::from(line);
        if (!invert && re.is_match(&line))
            || (invert && !re.is_match(&line))
            || (recursive && sub_dir.is_match(&line))
        {
            temp_lines.push(line);
        }
    }
    temp_lines
}

pub fn get_all_files(files: &str, invert: bool) -> Vec<String> {
    let lines = files.split('\n');
    let mut temp_lines: Vec<String> = Vec::new();
    if !invert {
        for line in lines.skip(1) {
            let line = String::from(line);
            temp_lines.push(line)
        }
    }
    temp_lines
}
