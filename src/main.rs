
struct Solution{}
impl Solution {

    pub fn int_to_roman(num: i32) -> String {
        let mut re =  String::new();
        let mut t = num;

        let a = t / 1000;
        for i in 0..a{re.push('M');}
        t = t % 1000;


        let mut a = t/100;
        t = t % 100;
        if a==4 {re.push_str("CD");}
        else if a==9 {{re.push_str("CM");}}
        else {
            if a>=5{re.push('D');a-=5}
            {
                for _ in 0..a{
                    re.push('C');
                }
            }
        }

        
        let mut a = t/10;
        t = t % 10;
        if a==4 {re.push_str("XL");}
        else if a==9{{re.push_str("XC");}}
        else {
            if a>=5{re.push('L');a-=5}
            {
                for _ in 0..a{
                    re.push('X');
                }
            }
        }   

        let mut a = t;
        if a==4 {re.push_str("IV");}
        else if a==9 {{re.push_str("IX");}}
        else {
            if a>=5{re.push('V');a-=5}
            {
                for _ in 0..a{
                    re.push('I');
                }
            }
        }   

        re
        
    }
     
}



fn main() {
    print!("{}",Solution::int_to_roman(3749));

}