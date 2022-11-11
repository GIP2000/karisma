use crate::{DbMemberTrait, DbObject};

pub enum Operator<T> {
    Eq(T),
    Lt(T),
    Lte(T),
    Gt(T),
    Gte(T),
}

pub enum Where<T> {
    Item(Operator<T>),
    Or(Vec<Where<T>>),
    And(Vec<Where<T>>),
    Not(Vec<Where<T>>),
    // Need to type check the Models better
    Select(Vec<crate::model::Models>),
    NestSingle(Vec<crate::model::Models>),
    NestAll(Vec<crate::model::Models>),
    NestHas(Vec<crate::model::Models>),
    NestNone(Vec<crate::model::Models>),
}

// {
//   id: 3,
//   AND:[
//       {first_name: "hi"},
//       {last_name: "bye"}
//   ],
//   posts: {
//      id: 2,
//      content: "Hello"
//   },
//   select: {
//      posts: {
//          id : 3,
//          content: "Hello"
//      },
//   }
// }
//
// ---->>
//
// vec![
//  Where::Item(Eq(UserEnumTypes::Id(3))),
//  Where::And(
//      vec![
//          Where::Item(Eq(UserEnumTypes::FirstName("hi"))),
//          Where::Item(Eq(UserEnumTypes::Lastname("bytw"))),
//      ]
//  ),
//
//  ...,
//  Where::Select(
//
//  vec![
//      Models::Posts(
//          vec![
//              Where::Eq(PostEnumTypes::id(3)),
//              Where::Eq(PostEnumTypes::Content(3)),
//          ]
//      )
//  ]
//
//  )
//
// ]
