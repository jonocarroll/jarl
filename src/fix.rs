use crate::message::*;

pub fn apply_fixes(fixes: &Vec<Message>, contents: &str) -> String {
    let fixes = fixes
        .iter()
        .map(|msg| match msg {
            Message::AnyDuplicated { fix, .. }
            | Message::AnyIsNa { fix, .. }
            | Message::TrueFalseSymbol { fix, .. } => fix,
        })
        .collect::<Vec<_>>();
    let old_content = contents;
    let mut new_content = old_content.to_string();
    let mut diff_length = 0;

    for fix in fixes {
        let mut start: i32 = fix.start.try_into().unwrap();
        let mut end: i32 = fix.end.try_into().unwrap();
        // println!("original start: {}", start);
        // println!("original end: {}", end);
        // println!("old_length: {}", old_length);
        // println!("new_length: {}", new_length);

        // println!("diff_length: {}", diff_length);

        start += diff_length;
        end += diff_length;

        diff_length += fix.offset_change_before;

        // println!("new start: {}", start);
        // println!("new end: {}\n", end);

        let start_usize = start as usize;
        let end_usize = end as usize;

        new_content.replace_range(start_usize..end_usize, &fix.content);
    }

    new_content.to_string()
}
