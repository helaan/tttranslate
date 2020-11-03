use isolang::Language;

fn main() {
    println!("Hello, world!");
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2);
}

fn convert(isocode: &str) -> String {
    if isocode == "zh_Hans" {
        return "simpchinese".to_owned();
    }
    Language::from_639_1(isocode)
        .unwrap()
        .to_name()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_en() {
        assert_eq!(convert("en"), "english");
    }
    #[test]
    fn test_convert_de() {
        assert_eq!(convert("de"), "german");
    }
    #[test]
    fn test_convert_fr() {
        assert_eq!(convert("fr"), "french");
    }
    #[test]
    fn test_convert_it() {
        assert_eq!(convert("it"), "italian");
    }
    #[test]
    fn test_convert_zh() {
        assert_eq!(convert("zh_Hans"), "simpchinese");
    }
    #[test]
    fn test_convert_ja() {
        assert_eq!(convert("ja"), "japanese");
    }

    #[test]
    fn test_convert_ru() {
        assert_eq!(convert("ru"), "russian");
    }

    #[test]
    fn test_convert_pl() {
        assert_eq!(convert("pl"), "polish");
    }
}
