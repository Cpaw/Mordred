extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable,Debug)]
struct MyRecord {
    id: i32,
    x: f32,
    y: f32,
}

fn main() {
    let mut rdr = csv::Reader::from_file("/file/test.csv").unwrap().has_headers(true);
    let mut rows: Vec<MyRecord> = Vec::new();
    for record in rdr.decode() {
        if let Ok(r) = record {
            rows.push(r);
        }
    }
    println!("{:?}", rows);
}
