use soup::{Soup, QueryBuilderExt, NodeExt};


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


    let body_text=soup.tag("body").find_all();

    let mut site= Site{
        title: soup.tag("title").find().unwrap().text(),
        words: Default::default()
    };
    for x in body_text{
        for word in x.text().split(" "){
            let word=word.replace("\n","");
            if word!="" && word !="\n"{
                if !words().contains(&word.to_string().to_uppercase()){
                    let mut current_word=Word{
                        word: word.clone(),
                        hits: 1
                    };
                    for test_word in x.text().split(" "){
                        if word== test_word{
                            current_word.hits+=1;
                        }
                    }
                    site.words.push(current_word);
                }
            }

        }
    }
    site.words.sort_by(|a,b| b.hits.cmp(&a.hits));
    return site;
}

// From https://en.wikipedia.org/wiki/Most_common_words_in_English
fn words()->Vec<String>{
    let words:Vec<String> = vec!["the".to_string(),
        "be".to_string().to_uppercase(),
        "to".to_string().to_uppercase(),
        "of".to_string().to_uppercase(),
        "in".to_string().to_uppercase(),
        "the".to_string().to_uppercase(),
        "is".to_string().to_uppercase(),
        "a".to_string().to_uppercase(),
        "in".to_string().to_uppercase(),
        "that".to_string().to_uppercase(),
        "I".to_string().to_uppercase(),
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

    ];
    return words;
}