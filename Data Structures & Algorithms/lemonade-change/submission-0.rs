impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut fives = 0;

        let mut tens = 0;
        let mut twentys = 0;

        for &item in bills.iter(){ 
            if item == 5{
                fives += 1;
            }else if item == 10{
                tens += 1;
                if fives>0{
                    fives-=1;
                }else{
                    return false
                }
            }else{ 
                twentys +=1;
                if tens>0 && fives>0{
                    tens-=1;
                    fives-=1;
                }else if fives >=3{
                    fives-=3;
                }else{
                    return false;
                }
            }
        }

    true
    }
}
