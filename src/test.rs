#[macro_export]
macro_rules! generate_regex_test {
    ($($name:ident, $fn_name:ident, $pattern:expr, $string:expr, $expected:expr)*) => {
    $(
        #[test]
        fn $name() {
            let res = $fn_name($pattern, $string);
            let expected: Result<bool, &str> = $expected;
            match (expected, &res) {
                (Ok(e), Ok(r)) => assert_eq!(
                    *r, e,
                    "Testing that /{}/ tested for \"{}\" should be {}",
                    $pattern, $string, e
                ),
                (Err(_), Err(_)) => (),
                _ => panic!(
                    "Expectation failed: {:?} is not equal to {:?} for pattern {} and string {}",
                    res, expected, $pattern, $string
                ),
            }
        }
    )*
    }
}

#[macro_export]
macro_rules! regex_tests {
    ($fn_name:ident) => {
        $crate::generate_regex_test!(test0, $fn_name, "cc|a*x", "z", Ok(false));
        $crate::generate_regex_test!(
            test1,
            $fn_name,
            "(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)9",
            "abcdefghijkl9",
            Ok(true)
        );
        $crate::generate_regex_test!(test2, $fn_name, "a.b", "acb", Ok(true));
        $crate::generate_regex_test!(test3, $fn_name, ")", "", Err(""));
        $crate::generate_regex_test!(test4, $fn_name, "", "", Ok(true));
        $crate::generate_regex_test!(test5, $fn_name, "abc", "abc", Ok(true));
        $crate::generate_regex_test!(test6, $fn_name, "abc", "xbc", Ok(false));
        $crate::generate_regex_test!(test7, $fn_name, "abc", "axc", Ok(false));
        $crate::generate_regex_test!(test8, $fn_name, "abc", "abx", Ok(false));
        $crate::generate_regex_test!(test9, $fn_name, "abc", "xabcy", Ok(true));
        $crate::generate_regex_test!(test10, $fn_name, "abc", "ababc", Ok(true));
        $crate::generate_regex_test!(test11, $fn_name, "ab*c", "abc", Ok(true));
        $crate::generate_regex_test!(test12, $fn_name, "ab*bc", "abc", Ok(true));
        $crate::generate_regex_test!(test13, $fn_name, "ab*bc", "abbc", Ok(true));
        $crate::generate_regex_test!(test14, $fn_name, "ab*bc", "abbbbc", Ok(true));
        $crate::generate_regex_test!(test15, $fn_name, "ab+bc", "abbc", Ok(true));
        $crate::generate_regex_test!(test16, $fn_name, "ab+bc", "abc", Ok(false));
        $crate::generate_regex_test!(test17, $fn_name, "ab+bc", "abq", Ok(false));
        $crate::generate_regex_test!(test18, $fn_name, "ab+bc", "abbbbc", Ok(true));
        $crate::generate_regex_test!(test19, $fn_name, "ab?bc", "abbc", Ok(true));
        $crate::generate_regex_test!(test20, $fn_name, "ab?bc", "abc", Ok(true));
        $crate::generate_regex_test!(test21, $fn_name, "ab?bc", "abbbbc", Ok(false));
        $crate::generate_regex_test!(test22, $fn_name, "ab?c", "abc", Ok(true));
        $crate::generate_regex_test!(test23, $fn_name, "^abc$", "abc", Ok(true));
        $crate::generate_regex_test!(test24, $fn_name, "^abc$", "abcc", Ok(false));
        $crate::generate_regex_test!(test25, $fn_name, "^abc", "abcc", Ok(true));
        $crate::generate_regex_test!(test26, $fn_name, "^abc$", "aabc", Ok(false));
        $crate::generate_regex_test!(test27, $fn_name, "abc$", "aabc", Ok(true));
        $crate::generate_regex_test!(test28, $fn_name, "^", "abc", Ok(true));
        $crate::generate_regex_test!(test29, $fn_name, "$", "abc", Ok(true));
        $crate::generate_regex_test!(test30, $fn_name, "a.c", "abc", Ok(true));
        $crate::generate_regex_test!(test31, $fn_name, "a.c", "axc", Ok(true));
        $crate::generate_regex_test!(test32, $fn_name, "a.*c", "axyzc", Ok(true));
        $crate::generate_regex_test!(test33, $fn_name, "a.*c", "axyzd", Ok(false));
        $crate::generate_regex_test!(test34, $fn_name, "a[bc]d", "abc", Ok(false));
        $crate::generate_regex_test!(test35, $fn_name, "a[bc]d", "abd", Ok(true));
        $crate::generate_regex_test!(test40, $fn_name, "a[\\-b]", "a-", Ok(true));
        $crate::generate_regex_test!(test41, $fn_name, "a[]b", "-", Err(""));
        $crate::generate_regex_test!(test42, $fn_name, "a[", "-", Err(""));
        $crate::generate_regex_test!(test43, $fn_name, "a\\", "-", Err(""));
        $crate::generate_regex_test!(test44, $fn_name, "abc)", "-", Err(""));
        $crate::generate_regex_test!(test45, $fn_name, "(abc", "-", Err(""));
        $crate::generate_regex_test!(test46, $fn_name, "a]", "a]", Ok(true));
        $crate::generate_regex_test!(test47, $fn_name, "a[]]b", "a]b", Ok(true));
        $crate::generate_regex_test!(test48, $fn_name, "a[\\]]b", "a]b", Ok(true));
        $crate::generate_regex_test!(test49, $fn_name, "a[^bc]d", "aed", Ok(true));
        $crate::generate_regex_test!(test50, $fn_name, "a[^bc]d", "abd", Ok(false));
        $crate::generate_regex_test!(test53, $fn_name, "a[^]b]c", "a]c", Ok(false));
        $crate::generate_regex_test!(test54, $fn_name, "a[^]b]c", "adc", Ok(true));
        $crate::generate_regex_test!(test74, $fn_name, "ab|cd", "abc", Ok(true));
        $crate::generate_regex_test!(test75, $fn_name, "ab|cd", "abcd", Ok(true));
        $crate::generate_regex_test!(test76, $fn_name, "()ef", "def", Ok(true));
        $crate::generate_regex_test!(test77, $fn_name, "$b", "b", Ok(false));
        $crate::generate_regex_test!(test78, $fn_name, "a\\(b", "a(b", Ok(true));
        $crate::generate_regex_test!(test79, $fn_name, "a\\(*b", "ab", Ok(true));
        $crate::generate_regex_test!(test80, $fn_name, "a\\(*b", "a((b", Ok(true));
        $crate::generate_regex_test!(test81, $fn_name, "a\\\\b", "a\\b", Ok(true));
        $crate::generate_regex_test!(test82, $fn_name, "((a))", "abc", Ok(true));
        $crate::generate_regex_test!(test83, $fn_name, "(a)b(c)", "abc", Ok(true));
        $crate::generate_regex_test!(test84, $fn_name, "a+b+c", "aabbabc", Ok(true));
        $crate::generate_regex_test!(test85, $fn_name, "(a+|b)*", "ab", Ok(true));
        $crate::generate_regex_test!(test86, $fn_name, "(a+|b)+", "ab", Ok(true));
        $crate::generate_regex_test!(test87, $fn_name, "(a+|b)?", "ab", Ok(true));
        $crate::generate_regex_test!(test88, $fn_name, ")(", "-", Err(""));
        $crate::generate_regex_test!(test89, $fn_name, "[^ab]*", "cde", Ok(true));
        $crate::generate_regex_test!(test90, $fn_name, "abc", "", Ok(false));
        $crate::generate_regex_test!(test91, $fn_name, "a*", "", Ok(true));
        $crate::generate_regex_test!(test92, $fn_name, "a|b|c|d|e", "e", Ok(true));
        $crate::generate_regex_test!(test93, $fn_name, "(a|b|c|d|e)f", "ef", Ok(true));
        $crate::generate_regex_test!(test94, $fn_name, "abcd*efg", "abcdefg", Ok(true));
        $crate::generate_regex_test!(test95, $fn_name, "ab*", "xabyabbbz", Ok(true));
        $crate::generate_regex_test!(test96, $fn_name, "ab*", "xayabbbz", Ok(true));
        $crate::generate_regex_test!(test97, $fn_name, "(ab|cd)e", "abcde", Ok(true));
        $crate::generate_regex_test!(test98, $fn_name, "[abhgefdc]ij", "hij", Ok(true));
        $crate::generate_regex_test!(test99, $fn_name, "^(ab|cd)e", "abcde", Ok(false));
        $crate::generate_regex_test!(test100, $fn_name, "(abc|)ef", "abcdef", Ok(true));
        $crate::generate_regex_test!(test101, $fn_name, "(a|b)c*d", "abcd", Ok(true));
        $crate::generate_regex_test!(test102, $fn_name, "(ab|ab*)bc", "abc", Ok(true));
        $crate::generate_regex_test!(test103, $fn_name, "a([bc]*)c*", "abc", Ok(true));
        $crate::generate_regex_test!(test104, $fn_name, "a([bc]*)(c*d)", "abcd", Ok(true));
        $crate::generate_regex_test!(test105, $fn_name, "a([bc]+)(c*d)", "abcd", Ok(true));
        $crate::generate_regex_test!(test106, $fn_name, "a([bc]*)(c+d)", "abcd", Ok(true));
        $crate::generate_regex_test!(test107, $fn_name, "a[bcd]*dcdcde", "adcdcde", Ok(true));
        $crate::generate_regex_test!(test108, $fn_name, "a[bcd]+dcdcde", "adcdcde", Ok(false));
        $crate::generate_regex_test!(test109, $fn_name, "(ab|a)b*c", "abc", Ok(true));
        $crate::generate_regex_test!(test110, $fn_name, "((a)(b)c)(d)", "abcd", Ok(true));
        $crate::generate_regex_test!(test112, $fn_name, "^a(bc+|b[eh])g|.h$", "abh", Ok(true));
        $crate::generate_regex_test!(
            test113,
            $fn_name,
            "(bc+d$|ef*g.|h?i(j|k))",
            "effgz",
            Ok(true)
        );
        $crate::generate_regex_test!(test114, $fn_name, "(bc+d$|ef*g.|h?i(j|k))", "ij", Ok(true));
        $crate::generate_regex_test!(
            test115,
            $fn_name,
            "(bc+d$|ef*g.|h?i(j|k))",
            "effg",
            Ok(false)
        );
        $crate::generate_regex_test!(
            test116,
            $fn_name,
            "(bc+d$|ef*g.|h?i(j|k))",
            "bcdd",
            Ok(false)
        );
        $crate::generate_regex_test!(
            test117,
            $fn_name,
            "(bc+d$|ef*g.|h?i(j|k))",
            "reffgz",
            Ok(true)
        );
        $crate::generate_regex_test!(test118, $fn_name, "(((((((((a)))))))))", "a", Ok(true));
        $crate::generate_regex_test!(
            test119,
            $fn_name,
            "multiple words of text",
            "uh-uh",
            Ok(false)
        );
        $crate::generate_regex_test!(
            test120,
            $fn_name,
            "multiple words",
            "multiple words, yeah",
            Ok(true)
        );
        $crate::generate_regex_test!(test121, $fn_name, "(.*)c(.*)", "abcde", Ok(true));
        $crate::generate_regex_test!(test122, $fn_name, "\\((.*), (.*)\\)", "(a, b)", Ok(true));
        $crate::generate_regex_test!(test124, $fn_name, "[k]", "ab", Ok(false));
        $crate::generate_regex_test!(test128, $fn_name, "^(.+)?B", "AB", Ok(true));
        $crate::generate_regex_test!(test129, $fn_name, "(a)(b)c|ab", "ab", Ok(true));
        $crate::generate_regex_test!(test130, $fn_name, "(a)+x", "aaax", Ok(true));
        $crate::generate_regex_test!(test131, $fn_name, "([ac])+x", "aacx", Ok(true));
        $crate::generate_regex_test!(
            test132,
            $fn_name,
            "([^/]*/)*sub1/",
            "d:msgs/tdir/sub1/trial/away.cpp",
            Ok(true)
        );
        $crate::generate_regex_test!(
            test133,
            $fn_name,
            "([^.]*)\\.([^:]*):[T ]+(.*)",
            "track1.title:TBlah blah blah",
            Ok(true)
        );
        $crate::generate_regex_test!(test134, $fn_name, "([^N]*N)+", "abNNxyzN", Ok(true));
        $crate::generate_regex_test!(test135, $fn_name, "([^N]*N)+", "abNNxyz", Ok(true));
        $crate::generate_regex_test!(test136, $fn_name, "([abc]*)x", "abcx", Ok(true));
        $crate::generate_regex_test!(test137, $fn_name, "([abc]*)x", "abc", Ok(false));
        $crate::generate_regex_test!(test138, $fn_name, "([xyz]*)x", "abcx", Ok(true));
        $crate::generate_regex_test!(test139, $fn_name, "(a)+b|aac", "aac", Ok(true));
        $crate::generate_regex_test!(test150, $fn_name, "*a", "-", Err(""));
        $crate::generate_regex_test!(test151, $fn_name, "(*)b", "-", Err(""));
        $crate::generate_regex_test!(test153, $fn_name, "a**", "-", Err(""));
        $crate::generate_regex_test!(test158, $fn_name, "([abc])*d", "abbbcd", Ok(true));
        $crate::generate_regex_test!(test159, $fn_name, "([abc])*bcd", "abcd", Ok(true));
        $crate::generate_regex_test!(test160, $fn_name, "((((((((((a))))))))))", "a", Ok(true));
        $crate::generate_regex_test!(test161, $fn_name, "a[-]?c", "ac", Ok(true));
        $crate::generate_regex_test!(test162, $fn_name, "^(.+)?B", "AB", Ok(true));
    };
}

