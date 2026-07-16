impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack= Vec::new();

        for chars in s.chars(){
            if chars== '(' || chars== '[' || chars == '{'{
                stack.push(chars);
            }else if (chars=='}' && stack.last()==Some(&'{')) ||  (chars==']' && stack.last()==Some(&'[')) ||
                (chars==')' && stack.last()==Some(&'(')){
                stack.pop();
            }else{
                return false;
            }
        }

        stack.is_empty()
    }
}
