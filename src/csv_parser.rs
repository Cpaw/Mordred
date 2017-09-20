//とりあえず文字列を分割して二次元のVecに保存する関数
//改行とカンマで分割
pub fn parse(s: &str)-> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = Vec::new();
    let v: Vec<&str> = s.split('\n').collect(); //&str型もString型もaplitがある

    for var in 0..v.len(){
        result.push(v[var].split(',').collect());
    }

    result.pop(); //\nでsplitしたときに最後尾に[""]っていう余計な要素ができるので、それを削除
    result
}


//CSVの形式を守ってるかをチェックする
pub fn check(v: Vec<Vec<&str>>, length: usize)-> bool{
    //一行のデータの数が指定の数と同等か
    for elem in v{
        if length != elem.len(){
            return false;
        }
    }

    true
}
