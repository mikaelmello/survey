use crate::{
    formatter::MultiCountFormatter,
    list_option::{CountedListOption, ListOption},
    test::fake_backend,
    ui::{Key, KeyModifiers},
    MultiCount,
};

#[test]
/// Tests that a closure that actually closes on a variable can be used
/// as a Select formatter.
fn closure_formatter() {
    let mut backend = fake_backend(vec![Key::Right(KeyModifiers::NONE), Key::Enter]);

    let formatted = String::from("Thanks!");
    let formatter: MultiCountFormatter<'_, &str> = &|_| formatted.clone();

    let options = vec!["one", "two", "three"];

    let ans = MultiCount::new("Question", options)
        .with_formatter(formatter)
        .prompt_with_backend(&mut backend)
        .unwrap();

    //assert_eq!(vec![(1, ListOption::new(0, "one"))], ans);
    assert_eq!(
        vec![CountedListOption {
            count: 1,
            list_option: ListOption::new(0, "one")
        }],
        ans
    );
}

#[test]
// Anti-regression test: https://github.com/mikaelmello/inquire/issues/30
fn down_arrow_on_empty_list_does_not_panic() {
    let mut backend = fake_backend(vec![
        Key::Char('9', KeyModifiers::NONE),
        Key::Down(KeyModifiers::NONE),
        Key::Backspace,
        Key::Char('3', KeyModifiers::NONE),
        Key::Down(KeyModifiers::NONE),
        Key::Backspace,
        Key::Enter,
    ]);

    let options = vec![1, 2, 3];

    let ans = MultiCount::new("Question", options)
        .prompt_with_backend(&mut backend)
        .unwrap();

    //assert_eq!(Vec::<(u32, ListOption<i32>)>::new(), ans);
    assert_eq!(Vec::<CountedListOption<i32>>::new(), ans);
}

#[test]
// Anti-regression test: https://github.com/mikaelmello/inquire/issues/31
fn list_option_indexes_are_relative_to_input_vec() {
    let mut backend = fake_backend(vec![
        Key::Down(KeyModifiers::NONE),
        Key::Right(KeyModifiers::NONE),
        Key::Down(KeyModifiers::NONE),
        Key::Right(KeyModifiers::SHIFT),
        Key::Enter,
    ]);

    let options = vec![1, 2, 3];

    let ans = MultiCount::new("Question", options)
        .prompt_with_backend(&mut backend)
        .unwrap();
    assert_eq!(
        vec![
            CountedListOption {
                count: 1,
                list_option: ListOption::new(1, 2)
            },
            CountedListOption {
                count: 10,
                list_option: ListOption::new(2, 3)
            }
        ],
        ans
    );
}

#[test]
// Anti-regression test: https://github.com/mikaelmello/inquire/issues/195
fn starting_cursor_is_respected() {
    let mut backend = fake_backend(vec![Key::Right(KeyModifiers::NONE), Key::Enter]);
    let options = vec![1, 2, 3];

    let ans = MultiCount::new("Question", options)
        .with_starting_cursor(2)
        .prompt_with_backend(&mut backend)
        .unwrap();

    assert_eq!(
        vec![CountedListOption {
            count: 1,
            list_option: ListOption::new(2, 3)
        }],
        ans
    );
}

#[test]
fn naive_assert_fuzzy_match_as_default_scorer() {
    let mut backend = fake_backend(vec![
        Key::Char('w', KeyModifiers::NONE),
        Key::Char('r', KeyModifiers::NONE),
        Key::Char('r', KeyModifiers::NONE),
        Key::Char('y', KeyModifiers::NONE),
        Key::Right(KeyModifiers::NONE),
        Key::Enter,
    ]);

    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let ans = MultiCount::new("Question", options)
        .prompt_with_backend(&mut backend)
        .unwrap();

    assert_eq!(
        vec![CountedListOption {
            count: 1,
            list_option: ListOption::new(2, "Strawberry")
        }],
        ans
    );
}

#[test]
fn chars_do_not_affect_prompt_without_filtering() {
    let mut backend = fake_backend(vec![
        Key::Char('w', KeyModifiers::NONE),
        Key::Char('r', KeyModifiers::NONE),
        Key::Char('r', KeyModifiers::NONE),
        Key::Char('y', KeyModifiers::NONE),
        Key::Right(KeyModifiers::NONE),
        Key::Enter,
    ]);

    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let ans = MultiCount::new("Question", options)
        .without_filtering()
        .prompt_with_backend(&mut backend)
        .unwrap();

    assert_eq!(
        vec![CountedListOption {
            count: 1,
            list_option: ListOption::new(0, "Banana")
        }],
        ans
    );
}
