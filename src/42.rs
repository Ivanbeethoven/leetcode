
struct Solution{}
impl Solution {

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut  stack  = Vec::<(i32,usize)>::new();
        let mut total =0 ;
        for (index, i) in height.iter().enumerate(){

            while !stack.is_empty() && i > &stack.last().unwrap().0{
                
                let pop = stack.pop().unwrap();
                if stack.is_empty(){
                    break;/////////!!!!!!!!!!!!!!!!!!!边界值处理
                }
                let la = stack.last().unwrap();
                total += (std::cmp::min(la.0, *i ) - pop.0) * (index - la.1-1) as i32;

            }

            stack.push((*i,index));
                
        }
        total
    }
     
}



fn main() {
    print!("{}",Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));

}