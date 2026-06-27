impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut temp = column_number;
        let mut result = String::new();
        while temp>0 {
            temp-=1;

            let chars = temp% 26;
            let letter= (b'A' + chars as u8) as char;

            result.push(letter);
            temp/=26;

        }

        result.chars().rev().collect()

    }
}
