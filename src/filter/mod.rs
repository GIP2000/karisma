use crate::{DbMemberTrait, DbObject};

pub enum VoP<T> {
    Virtual(T),
    Physical(T),
}

pub enum LinkWhere<T> {
    Link(Where<T>),
    Terminal(T),
}

pub enum Where<T> {
    AND(Vec<LinkWhere<T>>),
    NOT(Vec<LinkWhere<T>>),
    NEST(Vec<Where<T>>),
    EQ(T),
    LTE(T),
    LT(T),
    GT(T),
    GTE(T),
}


