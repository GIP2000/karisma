type RawDbOutput = str;

trait ParseDbOutput<T> {
    fn parse_db(&self) -> T;
}

impl<T> ParseDbOutput<T> for RawDbOutput
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn parse_db(&self) -> T {
        let a = self.parse::<T>().expect("error parsing db object");
        return a;
    }
}

trait Equal {
    type T;
}
impl<T> Equal for T {
    type T = T;
}

pub struct AssertEquals<T1, T2>(T1, std::marker::PhantomData<T2>)
where
    T1: Equal<T = T2>;

pub trait AssertHasParent<T> {
    fn assert_has_parent(&self, _: T);
}

pub trait DbMemberTrait<TParent>
where
    Self: Sized + AssertHasParent<TParent>,
{
    const SELECT: &'static str;
    const NAME: &'static str;
    fn build_from_query(args: Vec<&RawDbOutput>) -> Self;
    fn get_select(&self) -> &'static str {
        Self::SELECT
    }
    fn query_builder() -> String {
        todo!("build query")
    }
    fn where_builder() -> String {
        todo!("Implement Where")
    }
}

pub trait DbObject<TView>
where
    Self: Sized,
    TView: DbMemberTrait<Self>,
{
    fn raw_query(_query: String) -> String {
        todo!("Implement Raw Query")
    }
    fn find() -> Option<TView> {
        todo!("Implement Find")
    }
    fn find_many() -> Vec<TView> {
        todo!("Implement Find Many")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct User {
        id: u32,
        first_name: String,
        last_name: String,
        posts: Vec<Post>,
    }

    struct Post {
        id: u32,
        content: String,
        author_id: u32,
        author: User,
    }

    // Plain examle without virtual;
    struct UserView {
        id: u32,
        first_name: String,
    }

    impl AssertHasParent<User> for UserView {
        fn assert_has_parent(&self, User { id, first_name, .. }: User) {
            let _: AssertEquals<_, u32> = AssertEquals(id, std::marker::PhantomData);
            let _: AssertEquals<_, String> = AssertEquals(first_name, std::marker::PhantomData);
        }
    }

    impl DbMemberTrait<User> for UserView {
        const SELECT: &'static str = "id,first_name";
        const NAME: &'static str = "User";
        fn build_from_query(args: Vec<&RawDbOutput>) -> Self {
            Self {
                id: args[0].parse_db(),
                first_name: args[1].parse_db(),
            }
        }
    }
    impl DbObject<UserView> for User {}

    // Exmaple with a single relation

    struct PostView {
        id: u32,
        // #[User]
        author: UserView,
        content: String,
    }

    impl AssertHasParent<Post> for PostView {
        fn assert_has_parent(
            &self,
            Post {
                id,
                content,
                author,
                ..
            }: Post,
        ) {
            let _: AssertEquals<_, u32> = AssertEquals(id, std::marker::PhantomData);
            let _: AssertEquals<_, String> = AssertEquals(content, std::marker::PhantomData);
            let _: &dyn AssertHasParent<User> = &self.author as &dyn AssertHasParent<User>;
        }
    }

    impl DbMemberTrait<Post> for PostView {
        const SELECT: &'static str = "id,{},content";
        const NAME: &'static str = "Post";
        fn build_from_query(args: Vec<&RawDbOutput>) -> Self {
            Self {
                id: args[0].parse_db(),
                author: UserView::build_from_query(args.clone().split_at(1).1.to_vec()),
                content: args[1].parse_db(),
            }
        }
    }

    impl DbObject<PostView> for Post {}
}
