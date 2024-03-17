extern crate derive_box_dyn;
use derive_box_dyn::IntoBoxDyn;

#[test]
fn boxes_one() {
    #[derive(IntoBoxDyn)]
    #[box_dyn_traits(Nothing1)]
    struct MyStruct;
    trait Nothing1 {}
    trait Nothing2 {}
    impl Nothing1 for MyStruct {}
    impl Nothing2 for MyStruct {}

    let _: Box<dyn Nothing1> = MyStruct {}.into();
}

#[test]
fn boxes_multiple() {
    #[derive(IntoBoxDyn)]
    #[box_dyn_traits(Nothing1, Nothing2)]
    struct MyStruct;
    trait Nothing1 {}
    trait Nothing2 {}
    impl Nothing1 for MyStruct {}
    impl Nothing2 for MyStruct {}

    let _: Box<dyn Nothing1> = MyStruct {}.into();
    let _: Box<dyn Nothing2> = MyStruct {}.into();
}
