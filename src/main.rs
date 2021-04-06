use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct IssuePosition {
    filename: String,
    offset: u32,
    line: u32,
    column: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Issue {
    from_linter: String,
    text: String,
    severity: String,
    source_lines: Vec<String>,
    replacement: Option<String>,
    pos: IssuePosition,
    expect_no_lint: bool,
    expected_no_lint_linter: String,
}

impl Issue {
    fn to_csv(&self) -> String {
        format!(
            "{},\"{}\",\"{}\",{},{},{}",
            self.from_linter,
            self.text.replace("\"", "\"\""),
            self.source_lines
                .iter()
                .map(|l| l.trim().replace("\"", "\"\""))
                .collect::<Vec<String>>()
                .join("\\\\n"),
            self.pos.filename,
            self.pos.line,
            self.pos.column,
        )
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LinterResults {
    issues: Option<Vec<Issue>>,
}

impl LinterResults {
    fn to_csv(&self) -> String {
        match &self.issues {
            Some(issues) => format!(
                "Linter,Text,SourceLines,Filename,Line,Column\n{}",
                issues
                    .iter()
                    .map(|i| i.to_csv())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            None => String::from(""),
        }
    }
}

fn main() {
    let mut results = String::new();

    std::io::stdin().read_line(&mut results).unwrap();

    let results: LinterResults = serde_json::from_str(&results[..]).unwrap();

    println!("{}", results.to_csv());
}
