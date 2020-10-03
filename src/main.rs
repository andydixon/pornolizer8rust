use rand::Rng;
use titlecase::titlecase;

use pornolize::names;
use pornolize::prose;

mod pornolize;

fn main() {
    println!("{}", pornolize("The warnings of floodwater and potential for communities to be cut off relates to areas covered by an amber warning.

The weekend will see a \"spell of challenging and disruptive weather\" and Saturday will be \"a very wet and cold day for many\", said Met Office deputy chief meteorologist Laura Ellam.

The amber warnings are in force until 06:00 BST on Sunday - but the heavy rain is expected to continue until about noon as the stormy conditions push north and west.

The wind direction associated with the rainfall is \"unusual\" and rainfall is likely to occur in some areas that are normally well sheltered and drier, the Met Office said.

It added buses and trains may face delays or cancellations, because of \"difficult\" driving conditions and drains could become blocked with debris, as trees are now in full leaf.

Carol Holt from the Environment Agency said \"widespread and persistent rain\" is likely to lead to flooding.

The forecasts are for 25-50mm (1-2 inch) of rainfall in many areas, but 40-70mm in Scotland, and 70-90mm possible over higher ground in Wales and south-west England.

There is the potential for more than 120mm of rain on some of the most exposed high ground of Snowdonia and Exmoor and higher ground in Scotland.".to_string(), "en".to_string(), 99));
}

fn pornolize(text: String, language: String, swearibility: i16) -> String {
    return proseConverter(nameConverter(text, &language, swearibility), &language, swearibility);
}

fn nameConverter(text: String, language: &String, swearibility: i16) -> String {
    let mut names = names::getNames();
    let mut returnString = "";
    let mut rng = rand::thread_rng();

    let mut returnString: String = "".to_string();
    match names.get(language) {
        Some(nameArray) => {
            let mut wordArray = text.split(" ");
            let mut hasMatch = 0;
            for n in wordArray {
                if n == titlecase(n) {
                    hasMatch = hasMatch + 1;
                    if hasMatch == 2 {
                        hasMatch = 0;
                        if rng.gen_range(0, 100) < swearibility - 1 {
                            let mut selectedName = &nameArray[rng.gen_range(0, nameArray.len())];

                            returnString = returnString + &*"\"".to_string() + &selectedName + &*"\" ".to_string();
                        }
                    }
                } else {
                    hasMatch = 0;
                }
                returnString = returnString + &n.to_string() + " ";
            }
        }
        _ => {}
    }
    return returnString.to_owned();
}

fn proseConverter(text: String, language: &String, swearibility: i16) -> String {
    let mut prose = prose::getProse();
    let mut returnString: String = String::from("");
    let wordArray = text.split(" ");
    let mut passedWord: String = String::from("");
    let mut suffix = "";
    let mut rng = rand::thread_rng();


    match prose.get(language) {
        Some(proseArray) => {
            for w in wordArray {
                if w.ends_with("ing") || w.ends_with("ed") || w.ends_with("s") {
                    if w.ends_with("ing") { suffix = "ing"; }
                    if w.ends_with("ed") { suffix = "ed"; }
                    if w.ends_with("s") { suffix = "s"; }

                    passedWord = String::from(&proseArray[rng.gen_range(0, proseArray.len())]) + suffix;
                    suffix = "";
                } else {
                    passedWord = String::from(w);
                }
                returnString = returnString + &passedWord + " ";
            }
        }
        None => { println!("Unsupported Language: {}", language) }
    }
    return returnString;
}