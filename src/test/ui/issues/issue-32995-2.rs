#![allow(unused)]

fn main() {
    { fn f<X: ::std::marker()::Send>() {} }
    //~^ ERROR parenthesized type parameters may only be used with a `Fn` trait
    //~| WARN previously accepted

    { fn f() -> impl ::std::marker()::Send { } }
    //~^ ERROR parenthesized type parameters may only be used with a `Fn` trait
    //~| WARN previously accepted
}

#[derive(Clone)]
struct X;

impl ::std::marker()::Copy for X {}
//~^ ERROR parenthesized type parameters may only be used with a `Fn` trait
//~| WARN previously accepted
