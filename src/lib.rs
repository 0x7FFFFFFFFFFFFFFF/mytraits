use std::path::Path;
use eyre::WrapErr;

pub trait MyTraitForString {
    fn to_vec_char(&self) -> Vec<char>;
    fn to_vec_u8(&self) -> Vec<u8>;
}

impl MyTraitForString for String {
    fn to_vec_char(&self) -> Vec<char> {
        self.chars().collect::<Vec<_>>()
    }

    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

///
/// This trait is intended to be implemented by `String` and `&str`.
pub trait MyTraitForStringSlice<'a> {
    /// Test if a string slice equals to two other string slices.
    fn equals_to_one_of_2(self, a: &str, b: &str) -> bool;

    /// Test if a string slice equals to three other string slices.
    fn equals_to_one_of_3(self, a: &str, b: &str, c: &str) -> bool;

    /// Using the `regex` to find the first match in the current string. Then return the `n`th capture group.
    /// Note that group 0 corresponds to the entire matched part (of the left most first match).
    fn get_regex_matched_group_n(self, regex: &str, n: usize) -> eyre::Result<String>;

    /// This function assumes `self` denotes a file path. It then append an index to it. If the new path exists,
    /// it increases the index, and check again, until it get a path that doesn't exist.
    fn name_increase_width_2(self) -> String;

    /// Convert the string slice to a `Path` object.
    fn to_path(self) -> &'a Path;

    /// Convert the string slice to a `Vec<char>`.
    fn to_vec_char(self) -> Vec<char>;

    /// Convert the string slice to a `Vec<u8>`.
    fn to_vec_u8(self) -> Vec<u8>;

    /// Check if the string slice ends with one of the elements of the vector.
    fn ends_with_vec(self, v: Vec<&str>) -> bool;

    /// Assume `self` denotes a file path and returns the file name part as a string slice.
    fn get_file_name(self) -> &'a str;

    /// Assume `self` denotes a file path and returns the directory name part as a string slice.
    fn get_folder_name(self) -> &'a str;
}


impl<'a> MyTraitForStringSlice<'a> for &'a str {
    fn equals_to_one_of_2(self, a: &str, b: &str) -> bool {
        if self == a || self == b {
            return true;
        }
        false
    }

    fn equals_to_one_of_3(self, a: &str, b: &str, c: &str) -> bool {
        if self == a || self == b || self == c {
            return true;
        }
        false
    }

    fn get_regex_matched_group_n(self, regex: &str, n: usize) -> eyre::Result<String> {
        let re = regex::Regex::new(regex).wrap_err_with(|| format!("Failed to create regex object for {}", regex))?;
        let result = re.captures(self).ok_or(eyre::eyre!("Can't get capture groups"))?;
        Ok(result[n].to_string())
    }

    fn name_increase_width_2(self) -> String {
        fn inc(s: &str) -> String {
            let mut i = s.parse::<i32>().unwrap();
            i = i + 1;
            let width = s.chars().count();
            format!("[{:0>w$}]", i, w = width)
        }

        fn get_default(w: usize) -> String {
            format!("[{}]", "0".repeat(w))
        }

        let width: usize = 2;
        let file = Path::new(self);
        let re = regex::Regex::new(&format!(r"(?i)^(.+?)(\[({})\])?(\.\w+)?$", r"\d".repeat(width))).unwrap();
        let result = re.captures(file.to_str().unwrap()).unwrap();

        let mut t;
        if result.get(2).is_none() {
            t = format!("{}{}{}", &result.get(1).map_or("", |e| e.as_str()), get_default(width), &result.get(4).map_or("", |e| e.as_str()));
        } else {
            t = format!("{}{}{}", &result.get(1).map_or("", |e| e.as_str()), inc(&result.get(3).map(|e| e.as_str()).unwrap()), &result.get(4).map_or("", |e| e.as_str()));
        }

        while t.as_str().to_path().is_file() || t.as_str().to_path().is_dir() {
            t = t.name_increase_width_2();
        }
        t
    }

    fn to_path(self) -> &'a Path {
        Path::new(self)
    }

    fn to_vec_char(self) -> Vec<char> {
        self.chars().collect::<Vec<_>>()
    }

    fn to_vec_u8(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn ends_with_vec(self, v: Vec<&str>) -> bool {
        for e in v {
            if self.ends_with(e) {
                return true;
            }
        }
        false
    }

    fn get_file_name(self) -> &'a str {
        let x = Path::new(self);
        x.file_name().unwrap().to_str().unwrap()
    }

    fn get_folder_name(self) -> &'a str {
        let x = Path::new(self);
        x.parent().map_or("", |e| e.to_str().unwrap())
    }
}
