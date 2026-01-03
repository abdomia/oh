use csv::StringRecord;

pub fn logging_csv(data: (StringRecord, Vec<StringRecord>)) {
    let mut max_each_col = vec![
        0; data.1.iter().nth(0).unwrap().len()
    ];

    for i in 0..max_each_col.len() {
        let mut col: Vec<String> = vec![];
        col.push(data
            .0
            .iter()
            .nth(i)
            .unwrap()
            .to_string()
        );

        col.extend(data.1.iter().map(|row| {
            let v = row.iter().nth(i);
            if let Some(val) = v {
                val.to_string()
            } else {
                "".to_string()
            }
        }));

        let max_len = col
            .iter()
            .max_by_key(|&x| x.chars().count())
            .unwrap()
            .chars()
            .count();
        max_each_col[i] = max_len;
    }
    // for headers
    let mut joind_head = "".to_string();
    for (i, h) in data.0.iter().enumerate() {
        let size= max_each_col[i];
        let h_size = h.chars().count();
        let dist = size.abs_diff(h_size);
        let formated = format!("{}{} | ", h, " ".repeat(dist));
        joind_head.push_str(&formated);
    }
    println!("{}", joind_head);
    println!("{}", "-".repeat(joind_head.chars().count()));

    // for data logging
    for row_record in data.1.iter() {
        create_table(&row_record, &max_each_col);
    }
}

fn create_table(record: &StringRecord, max_each_col: &Vec<usize>) {
    let mut joind_row = "".to_string();
    for (i, rcrd) in record.iter().enumerate() {
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
