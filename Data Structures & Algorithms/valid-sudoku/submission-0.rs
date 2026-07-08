
use std::collections::HashSet;
impl Solution {

    pub fn check(board: &Vec<Vec<char>>, index: usize)-> bool{

        let mut set = HashSet::new();
        for i in 0..board.len(){
            let c = board[i][index];

            if set.contains(&c){
                return false;
            }else if c != '.'{
                set.insert(c);
            }
        }

        set = HashSet::new();
        for i in 0..board.len(){
            let c = board[index][i];

            if set.contains(&c){
                return false;
            }else if c != '.'{
                set.insert(c);
            }
        }

        set = HashSet::new();


        for i in (3* (index/3)  as usize)..(3* (index/3)+3   as usize){
            for j in ((3*index)%9 as usize)..((3*index)%9 +3 as usize){
                if !set.insert(board[i][j]) && board[i][j] != '.'{
                    return false; 
                }
            }
        }


    true
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {


        for i in 0..board.len(){
            if !Self::check(&board,i){
                return false;
            }

        }


        true
    }
}
