/// 38.
///
/// 报数序列是一个整数序列，按照其中的整数的顺序进行报数，得到下一个数。其前五项如下：
///
/// 1.     1
/// 2.     11
/// 3.     21
/// 4.     1211
/// 5.     111221
/// 1 被读作  "one 1"  ("一个一") , 即 11。
/// 11 被读作 "two 1s" ("两个一"）, 即 21。
/// 21 被读作 "one 2",  "one 1" （"一个二" ,  "一个一") , 即 1211。
///
/// 给定一个正整数 n（1 ≤ n ≤ 30），输出报数序列的第 n 项。
///
/// 注意：整数顺序将表示为一个字符串。
///
///  
///
/// 示例 1:
///
/// 输入: 1
/// 输出: "1"
/// 示例 2:
///
/// 输入: 4
/// 输出: "1211"
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/count-and-say
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn count_and_say(n: i32) -> String {
    let (mut num_str, mut result) = (String::from(""), String::from("1"));
    for i in 1..n {
        num_str = result;
        result = String::from("");
        let mut current_num = 0;
        while current_num < num_str.len() {
            let (mut count, mut next_num) = (0, current_num);
            while next_num < num_str.len() && &num_str[current_num..=current_num] == &num_str[next_num..=next_num] {
                next_num += 1;
                count += 1;
            }
            result.push(count.to_string().parse().unwrap());
            result.push_str(&num_str[current_num..=current_num]);
            current_num = next_num;
        }
    }
    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        let res = count_and_say(4);
        assert_eq!(res, String::from("1211"));
        let res = count_and_say(5);
        assert_eq!(res, String::from("111221"));
        let res = count_and_say(6);
        assert_eq!(res, String::from("312211"));
    }
}
