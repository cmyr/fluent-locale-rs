extern crate fluent_locale;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::fluent_locale::locale::Locale;
use self::fluent_locale::negotiate::negotiate_languages;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct LocaleTestInputData {
    string: String,
    options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocaleTestOutputObject {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
    extensions: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum LocaleTestOutput {
    String(String),
    Object(LocaleTestOutputObject),
}

#[derive(Serialize, Deserialize)]
struct LocaleTestSet {
    input: LocaleTestInputData,
    output: LocaleTestOutput,
}

fn read_locale_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<LocaleTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

#[derive(Serialize, Deserialize)]
struct NegotiateTestSet {
    input: (Vec<String>, Vec<String>),
    output: Vec<String>,
}

fn read_negotiate_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<NegotiateTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

fn test_locale_fixtures(path: &str) {
    let tests = read_locale_testsets(path).unwrap();

    for test in tests {
        let s = test.input.string;

        let loc;
        if let Some(opts) = test.input.options {
            let borrowed: HashMap<&str, &str> =
                opts.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            loc = Locale::new(&s, Some(borrowed)).unwrap();
        } else {
            loc = Locale::from(s);
        }

        match test.output {
            LocaleTestOutput::Object(o) => {
                assert_eq!(loc.get_language(), o.language.as_ref());
                assert_eq!(loc.get_script(), o.script.as_ref());
                assert_eq!(loc.get_region(), o.region.as_ref());
                assert_eq!(loc.get_variants(), o.variants.as_ref());
                assert_eq!(loc.extensions, o.extensions);
            }
            LocaleTestOutput::String(s) => {
                assert_eq!(loc.to_string(), s);
            }
        }
    }
}

fn test_negotiate_fixtures(path: &str) {
    let tests = read_negotiate_testsets(path).unwrap();

    for test in tests {
        let requested: Vec<&str> = test.input.0.iter().map(|v| v.as_str()).collect();
        let available: Vec<&str> = test.input.1.iter().map(|v| v.as_str()).collect();
        assert_eq!(negotiate_languages(requested, available), test.output);
    }
}

#[test]
fn parse() {
    test_locale_fixtures("./tests/fixtures/locale/parsing.json");
}

#[test]
fn parse_ext() {
    test_locale_fixtures("./tests/fixtures/locale/parsing-ext.json");
}

#[test]
fn serialize() {
    test_locale_fixtures("./tests/fixtures/locale/serialize-options.json");
}

#[test]
fn options() {
    test_locale_fixtures("./tests/fixtures/locale/options.json");
}

#[test]
fn options_ext() {
    test_locale_fixtures("./tests/fixtures/locale/options-ext.json");
}


#[test]
fn negotiate() {
    test_negotiate_fixtures("./tests/fixtures/negotiate/filtering/exact-match.json");
}