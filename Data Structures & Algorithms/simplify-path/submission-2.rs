impl Solution {
    pub fn simplify_path(path: String) -> String {
       
        
        let mut stack = Vec::new();

        for token in path.split("/"){
            match token{
                "" | "." => {},

                ".." => {stack.pop();},
                valid => {stack.push(valid);}
            }

        }
        format!("/{}",stack.join("/"))
    }
}