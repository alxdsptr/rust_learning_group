fn main() {
    let vec: Vec<_> = vec!["Rust", "is", "the", "Genshin", "Impact", "of", "programming", "languages", "but", "if", "you", "learn", "it", "well", "it", "feels", "like", "Saizeriya"].into_iter().map(String::from).collect();

    fn this_is_t(t: String) -> String { t }
    fn this_is_ref(r: &String) -> &String { r }
    fn this_is_mut_ref(r: &mut String) -> &mut String { r }

    /*
        Guide: Fix the code below to make it compile.

        DO NOT REORDER any line in the following code.

        Fix errors in the following order:
        1. Check the function signatures and fix function and macro calls. Add `mut`, `&`, `&mut` where necessary. Do not change the `in vec` part.
        2. Solve compiler error E0382. The compiler will suggest something for you. Consider ownership rules and reason why the solution works.
        3. Solve compiler error E0502 by commenting out one line of the `assert_eq!`. Consider borrowing rules and reason why commenting out that line works.
     */

    for mut word // FIX ME
    in vec {
        let t = {
            this_is_t(word.clone()) // FIX ME
        };
        let r = {
            this_is_ref(&word) // FIX ME
        };
        let r_mut = {
            this_is_mut_ref(&mut word) // FIX ME
        };

        // assert_eq!(t, *r); // FIX ME
        assert_eq!(t, *r_mut); // FIX ME
    }
}