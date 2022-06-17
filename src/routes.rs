use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/solve/:puzzle")]
    Solve { puzzle: String },
    #[at("/set/:puzzle")]
    Set { puzzle: String },
    #[at("/set")]
    SetNew,
}
