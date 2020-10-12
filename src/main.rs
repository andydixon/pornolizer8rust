extern crate kuchiki;
mod pornolize;
mod webpage_handler;
use pornolize::pornolizer_core;

use kuchiki::traits::*;

fn main() {
//     println!("{}", pornolizer_core::pornolize("The warnings of floodwater and potential for communities to be cut off relates to areas covered by an amber warning. The weekend will see a \"spell of challenging and disruptive weather\" and Saturday will be \"a very wet and cold day for many\", said Met Office deputy chief meteorologist Laura Ellam.
// The amber warnings are in force until 06:00 BST on Sunday - but the heavy rain is expected to continue until about noon as the stormy conditions push north and west. The wind direction associated with the rainfall is \"unusual\" and rainfall is likely to occur in some areas that are normally well sheltered and drier, the Met Office said.
// It added buses and trains may face delays or cancellations, because of \"difficult\" driving conditions and drains could become blocked with debris, as trees are now in full leaf.
// Carol Holt from the Environment Agency said \"widespread and persistent rain\" is likely to lead to flooding.
//
// The forecasts are for 25-50mm (1-2 inch) of rainfall in many areas, but 40-70mm in Scotland, and 70-90mm possible over higher ground in Wales and south-west England.
// There is the potential for more than 120mm of rain on some of the most exposed high ground of Snowdonia and Exmoor and higher ground in Scotland.".to_string(), "en".to_string(), 99));

    let request:webpage_handler::url_content = webpage_handler::webpage_handler::parse_url(String::from("https://www.bbc.co.uk/news"));

    let document = kuchiki::parse_html().one(request.body);

    for css_match in document.select("h1,h2,h3,h4,p,span").unwrap() { // could be .foo or #foo
        let as_node = css_match.as_node();

        // In this example, as_node represents an HTML node like
        //
        //   <p class='foo'>Hello world!</p>"
        //
        // Which is distinct from just 'Hello world!'. To get rid of that <p>
        // tag, we're going to get each element's first child, which will be
        // a "text" node.
        //
        // There are other kinds of nodes, of course. The possibilities are all
        // listed in the `NodeData` enum in this crate.

        if as_node.first_child().is_some() {
            let text_node = as_node.first_child().unwrap();

            // Let's get the actual text in this text node. A text node wraps around
            // a RefCell<String>, so we need to call borrow() to get a &str out.
            if text_node.as_text().is_some() {
                let mut text = text_node.as_text().unwrap().borrow();

                text.

                println!("{:?}", text);
            }
        }
    }

}
