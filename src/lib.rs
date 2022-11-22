mod filter;
mod model;
//
// pub type RawDbOutput = str;
//
// pub trait DbParseable<T> {
//     fn db_parse(&self) -> T;
// }
//
// impl DbParseable<u32> for RawDbOutput {
//     fn db_parse(&self) -> u32 {
//         self.parse().expect("Error Parsing string to int")
//     }
// }
//
// impl DbParseable<String> for RawDbOutput {
//     fn db_parse(&self) -> String {
//         self.to_string()
//     }
// }
//
// pub trait DbMemberTrait {
//     const SELECT: &'static str;
//     const NAME: &'static str;
//
//     type WhereFilterType;
//
//     fn build_from_query(args: Vec<&RawDbOutput>) -> Self;
//     fn query_builder() -> String {
//         format!("SELECT {} FROM {}", Self::SELECT, Self::NAME)
//     }
//     fn where_builder(filter: Vec<crate::filter::Where<Self::WhereFilterType>>) -> String {
//         todo!("impl")
//     }
// }
//
// pub trait DbObject<T: DbMemberTrait> {
//     type WhereFilterType;
//     fn raw_query(_query: String) -> String {
//         return String::from("0,Greg,Presser\n1,First,Last\n2,Second,SLast");
//     }
//
//     fn find_many(filter: Vec<crate::filter::Where<Self::WhereFilterType>>) -> Vec<T> {
//         let resp = Self::raw_query(T::query_builder());
//         return resp
//             .split('\n')
//             .map(|s| T::build_from_query(s.split(',').collect()))
//             .collect();
//     }
// }
