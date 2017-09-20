pub fn parse(s: &str)-> Vec<Vec<&str>> {
    let v: Vec<&str> = s.split('\n').collect(); //&str型もString型もaplitがある
    let mut res: Vec<Vec<&str>> = Vec::new();

    for var in 0..v.len(){
        res.push(v[var].split(',').collect());
    }
    res
}
