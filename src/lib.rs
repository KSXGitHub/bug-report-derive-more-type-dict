use derive_more::{Display, Error};

pub trait MyTypeDict {
    type Foo;
    type Bar;
}

#[derive(Debug, Display, Error)]
pub enum MyEnum<TypeDict: MyTypeDict> {
    Foo(#[error(source)] TypeDict::Foo),
    Bar(#[error(source)] TypeDict::Bar),
}
