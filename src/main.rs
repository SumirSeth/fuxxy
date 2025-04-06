mod matchers;
use matchers::{fzf_style, jaro_winkler};

fn main() {
    let query = "abc";
    let target = "alphabetical";

    let jw_score = jaro_winkler::score(query, target);
    let fzf_score = fzf_style::score(query, target);

    println!("jaro_winkler score: {:.4}", jw_score);
    println!("fzf_score: {}", fzf_score);
}
