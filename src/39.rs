struct Solution{}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let i =0;
        let mut result:Vec<Vec<i32>> = Vec::new();
        let ans: Vec<i32> = Vec::new();
        Self::dfs(&candidates,target,i,&mut result,ans);
        return result;
    }
    pub fn dfs(candidates:&Vec<i32>, target : i32, i: usize ,result: &mut Vec<Vec<i32>>, mut ans : Vec<i32> ){
        if target<0{
            return ;
        }else if 0 == target{
            result.push(ans);
        }else{
            let n = candidates.len();
            for j in i..n{
                let mut _a = ans.clone();
                _a.push(candidates[j]);
                Self::dfs(candidates,target-candidates[j],j,result,_a);
            }
        }

    }
}

fn main() {
    let candidates = vec![2,3,6,7];
    let target = 7;
    let re = Solution::combination_sum(candidates, target);
    println!("{:?}",re);
}


