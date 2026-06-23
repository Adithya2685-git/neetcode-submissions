impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();

        let mut left = 0;
        let mut right = people.len() -1;

        let mut count =0;

        while left <= right{
            if left ==right{count +=1; break;}
            let sum = people[left] + people[right];

            if people[left] > limit{
                count +=1;
                left +=1 ;
            }else if people[right] >limit{ 
                count +=1;
                right -=1;
            } else if sum <= limit{
                count+=1;
                left+=1; 
                right-=1;
            } else {
                if people[left] > people[right] {
                    count+=1;
                    left+=1;
                }else{
                    count+=1;
                    right -=1;
                }
            }
        }
        count
    }
}

