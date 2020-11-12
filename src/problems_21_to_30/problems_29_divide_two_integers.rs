/// 29.
///
/// 给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。
///
/// 返回被除数 dividend 除以除数 divisor 得到的商。
///
/// 示例 1:
///
/// 输入: dividend = 10, divisor = 3
/// 输出: 3
/// 示例 2:
///
/// 输入: dividend = 7, divisor = -3
/// 输出: -2
/// 说明:
///
/// 被除数和除数均为 32 位有符号整数。
/// 除数不为 0。
/// 假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231,  231 − 1]。本题中，如果除法结果溢出，则返回 231 − 1。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/divide-two-integers
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == std::i32::MIN && divisor == -1 { return std::i32::MAX; }
    let (mut dividend_mut, mut divisor_mut, mut flag_mut) = (dividend, divisor, 1);
    if dividend_mut > 0 {
        dividend_mut = -dividend_mut;
    } else {
        flag_mut = -flag_mut;
    }
    if divisor_mut > 0 {
        divisor_mut = -divisor_mut
    } else {
        flag_mut = -flag_mut;
    }

    let (mut res, mut tmp, mut k) = (0, 0, 0);
    while dividend_mut <= divisor_mut {
        tmp = divisor_mut;
        k = 1;
        while dividend_mut <= tmp + tmp && tmp + tmp < 0 {
            tmp += tmp;
            k += k;
        }
        res = res + k;
        dividend_mut -= tmp
    }
    if flag_mut > 0 { res } else { -res }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        let res = divide(10, 3);
        assert_eq!(res, 3);
        let res = divide(7, -3);
        assert_eq!(res, -2);
    }
}
