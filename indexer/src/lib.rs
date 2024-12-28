pub mod html_to_urls;
pub mod indexer_maker;

trait SplitToString{
    fn split_to_string(&self, p: impl Into<String>) -> Vec<String>;
    fn split_to_string_at_occurrence_tuple<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> (String, String);
    fn split_to_string_at_occurrence<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> Vec<String>;
}

impl SplitToString for String {
    fn split_to_string(&self, p: impl Into<String>) -> Vec<String> {
        self.split(Into::<String>::into(p).as_str()).map(|s| s.to_string()).collect()
    }

    fn split_to_string_at_occurrence_tuple<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> (String, String) {
        match occurrence {
            Occurrence::First => {
                let x = self.split_once(&Into::<String>::into(p)).unwrap_or((self.as_str(), ""));
                return (x.0.to_string(), x.1.to_string())
            },
            Occurrence::Last => {
                let x = self.chars().rev().collect::<String>();
                let x = x.split_once(&Into::<String>::into(p)).unwrap_or(("", x.as_str()));
                return (x.1.to_string().chars().rev().collect::<String>(), x.0.to_string().chars().rev().collect::<String>()) // has to be reversed
            },
            Occurrence::Nth(n) => {
                let split = self.split(&Into::<String>::into(p.clone())).collect::<Vec<&str>>();

                let len = split.len();

                if n >= len {
                    return (split.join(&Into::<String>::into(p)).to_string(), "".to_string())
                } else {
                    let first = split[..n].join(&Into::<String>::into(p.clone()));
                    let second = split[n..].join(&Into::<String>::into(p)); 

                    (first, second)
                }
            },
        }
    }

    fn split_to_string_at_occurrence<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> Vec<String> {
        let x = self.split_to_string_at_occurrence_tuple(p, occurrence);

        vec![x.0, x.1]
    }
}

enum Occurrence {
    First, 
    Last,
    Nth(usize),    
}

#[test]
fn test() {
    let s = "iusdghuif#aluiyeuit#aliugt#ayeiutu#aouwsgy#aluidhsfg#liyufagsd".to_string();
    let one = s.split_to_string_at_occurrence("#", Occurrence::First);
    let two = s.split_to_string_at_occurrence("?", Occurrence::Last);
    let three = s.split_to_string_at_occurrence("#", Occurrence::Nth(2));

    println!("{:?},\n{:?},\n{:?}", one, two, three)
}