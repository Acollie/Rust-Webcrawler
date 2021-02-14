use soup::{Soup, QueryBuilderExt, NodeExt};
use std::collections::HashSet;
use std::iter::FromIterator;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Site{
    pub title:String,
    pub words:Vec<Word>
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Word{
    pub word:String,
    pub hits:i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebsiteTypes{
    PersonalSite,
    Shopping,
    Travel,
}

pub fn words_from_soup(soup:&Soup)->Site{
    let mut found_words:HashSet<String> = HashSet::new();
    let body_text=soup.tag("body").find_all();
    let mut site= Site{
        title: soup.tag("title").find().unwrap().text(),
        words: Default::default()
    };
    for body in body_text{
        for word in body.text().split(" "){

            let word=remove_format(&word.to_string());
            let common_words = words();
            if !common_words.contains(&word.to_uppercase()) && !word.contains(" ") || !word.contains("") && word != ""{
                if found_words.contains(&*word){
                    for mut search_word in &mut site.words{
                        if search_word.word == word{
                            search_word.hits += 1;
                        }
                    }
                } else {
                    let current_word = Word {
                        word: word.clone(),
                        hits: 1
                    };
                    site.words.push(current_word);
                    found_words.insert(word.clone());
                }
            }
        }
    }
    site.words.sort_by(|a,b| b.hits.cmp(&a.hits));
    return site;
}



fn remove_format(word:&String)->String{
    let word=word.replace("\n"," ");
    let word=word.replace(","," ");
    let word=word.replace("."," ");
    let word=word.replace(" "," ");
    return word
}

// From https://en.wikipedia.org/wiki/Most_common_words_in_English
fn words()->HashSet<String>{
    let words:Vec<String> = vec!["the".to_string(),
        "be".to_string().to_uppercase(),
        "to".to_string().to_uppercase(),
        "of".to_string().to_uppercase(),
        "in".to_string().to_uppercase(),
        "the".to_string().to_uppercase(),
        "is".to_string().to_uppercase(),
        "a".to_string().to_uppercase(),
        "have".to_string().to_uppercase(),
        "in".to_string().to_uppercase(),
        "that".to_string().to_uppercase(),
        "I".to_string().to_uppercase(),
        "and".to_string().to_uppercase(),
        "it".to_string().to_uppercase(),
        "for".to_string().to_uppercase(),
        "not".to_string().to_uppercase(),
        "on".to_string().to_uppercase(),
        "with".to_string().to_uppercase(),
        "he".to_string().to_uppercase(),
        "as".to_string().to_uppercase(),
        "but".to_string().to_uppercase(),
        "no".to_string().to_uppercase(),
        "might".to_string().to_uppercase(),
        "an".to_string().to_uppercase(),
        "you".to_string().to_uppercase(),
        "were".to_string().to_uppercase(),
        "around".to_string().to_uppercase(),
        "would".to_string().to_uppercase(),
        "this".to_string().to_uppercase(),

    ];
    return HashSet::from_iter(words);
}