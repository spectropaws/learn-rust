use regex::{Regex, RegexBuilder};

pub struct Matcher {
    regex: Option<Regex>,
    pattern: String,
    ignore_case: bool,
}

impl Matcher {
    pub fn new(
        mut pattern: String,
        use_regex: bool,
        ignore_case: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let regex = if use_regex {
            Some(
                RegexBuilder::new(&pattern)
                    .case_insensitive(ignore_case)
                    .build()?,
            )
        } else {
            None
        };

        if ignore_case {
            pattern = pattern.to_lowercase();
        }

        Ok(Self {
            regex,
            pattern,
            ignore_case,
        })
    }

    pub fn matches(&self, line: &str) -> bool {
        if let Some(re) = &self.regex {
            return re.is_match(line);
        }

        if self.ignore_case {
            line.to_lowercase().contains(&self.pattern)
        } else {
            line.contains(&self.pattern)
        }
    }
}
