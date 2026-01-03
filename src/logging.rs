use csv::StringRecord;

pub fn logging_csv(header: StringRecord, data: Vec<StringRecord>) {
    let mut max_each_col = vec![
        0; data.iter().nth(0).unwrap().len()
    ];
    for i in 0..max_each_col.len() {
        let col: Vec<String> = data.iter().map(|elem| {
            let v = elem.iter().nth(i);
            if let Some(val) = v {
                val.to_string()
            } else {
                "".to_string()
            }
        }).collect();

        let max_len = col
            .iter()
            .max_by_key(|&x| x.len())
            .unwrap()
            .chars()
            .count();
        max_each_col[i] = max_len;
    }

    for row_record in data.iter() {
        let mut joind_row = "".to_string();
        for (i, rcrd) in row_record.iter().enumerate() {
            let check = max_each_col[i].checked_sub(rcrd.chars().count());
            if let Some(dist) = check {
                let formated = format!("{}{} | ", rcrd, " ".repeat(dist));
                joind_row.push_str(&formated);
            } else {
                println!("error here while substracting");
            }
        }
        println!("{}", joind_row);
    }
}

