pub fn score(query: &str, target: &str) -> u32 {
    let mut score = 0;
    let mut last_match_idx = None;
    let mut target_chars = target.chars().enumerate();

    for qc in query.chars() {
        let mut found = false;
        while let Some((i, tc)) = target_chars.next() {
            if tc == qc {
                if let Some(prev_i) = last_match_idx {
                    if i == prev_i + 1 {
                        score += 5;
                    } else {
                        score += 1;
                    }
                } else {
                    score += if i == 0 { 10 } else { 3 };
                }
                last_match_idx = Some(i);
                found = true;
                break;
            }
        }

        if !found {
            return 0;
        }
    }

    score
}
