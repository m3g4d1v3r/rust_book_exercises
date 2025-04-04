enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);

    let first = &v[0];

    println!("The first element is: {first}");

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in row {
        match cell {
            SpreadsheetCell::Int(value) => println!("{}", value),
            SpreadsheetCell::Float(value) => println!("{}", value),
            SpreadsheetCell::Text(value) => println!("{}", value),
        }
    }

    let arr = [1, 2, 3, 4, 5];
    let mut v2 = Vec::new();

    for el in arr {
        v2.push(el);
        // for idx in 0..v2.len() {
        //     let el_option = v2.get(idx);
        //     match el_option {
        //         Some(data) => println!("{}", data.to_string()),
        //         None => println!("Nothing"),
        //     }
        // }
    }

    for _idx in 0..v2.len() {
        let el_option = v2.pop();
        match el_option {
            Some(data) => println!("{}", data.to_string()),
            None => println!("Nothing"),
        }
    }
}
