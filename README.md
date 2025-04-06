# Fuxxy - Fuzzy Finder
Fuxxy is a *fast*  and *lightweight* fuzzy finder built in rust.

## What is a fuzzy finder?  
Fuzzy search is a type of search algorithm that returns results even when the query is partially correct, misspelled, or incomplete. It does this by measuring the similarity between the query and potential matches, rather than looking for an exact match.

## Design : Jaro-Winkler + FZF Style Fuzzy Search
The idea is to combine two types of fuzzy searches that each have their own advantages and pitfalls.
`jw_score` -> (0 to 1).  
`fzf_score` -> (0 to x).  
***Combine*** them using a weighted formula.  
`combined_score` = $\alpha \times$ `jw_score` + $\beta \times$ `fzf_score(normalised)`


---

Made with ❤️  by [Sumir](https://github.com/sumirseth). 
