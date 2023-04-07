use regex::Regex;

pub fn replace_param(text: &String, param: &str, value: String) -> String {
    let mut par = String::from(param);

    // par = r"\$\{\{".to_owned()+&par+r"\}\}";
    par = r"\$(\{(?:\{\{??".to_owned()+&par+&r"\}\}))".to_owned();

    let regex = Regex::new(&par).unwrap(); // Necessário ser no formato do rust(Que chatooooo)
    let ret = regex.replace_all(text, value).to_string();
    return ret;
}
