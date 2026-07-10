use std::collections::VecDeque;
impl Solution {

    pub fn surrounded(board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, q: &mut Vec<(usize, usize)>)-> bool{
        if visited[i][j]{
            return true;
        }

        visited[i][j] = true;
        let mut is_surr = true;
        if (i==0 || i== board.len() -1 || j==0 || j == board[0].len()-1) && board[i][j] == 'O'{
            is_surr= false;
        }

        if board[i][j] == 'O'{
            q.push((i,j));
        }else{
            return true;
        }

        let mut left = true;
        let mut right = true;
        let mut up = true;
        let mut down = true;
        if i>0{
            left = Self::surrounded(board, visited,i-1,j,q);
        }
        if j>0{
            down = Self::surrounded(board, visited, i,j-1,q);
        }
        if i+1<board.len(){
            right = Self::surrounded(board,visited,i+1,j,q); 
        }
        if j+1< board[0].len(){
            up = Self::surrounded(board,visited,i,j+1,q);
        } 

        is_surr && up && left && down && right         
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {

        let mut visited = vec![vec![false ; board[0].len()]; board.len()];
        let mut q = Vec::new();

        for i in 0..board.len(){
            for j in 0..board[0].len(){

                if board[i][j] == 'O' && !visited[i][j] && Self::surrounded(board, &mut visited, i, j, &mut q){

                    for (x,y) in q.iter(){
                        board[*x][*y] ='X';
                    }

                }
                q.clear()

            }
        }


    }
}
