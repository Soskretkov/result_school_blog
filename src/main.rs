use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use components::{Footer, Header, PageErrMsg};
use leptos_router::*;
use pages::*;
use server::Session;
use types::GlobContext;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let sess: Option<Session> = Some(Session {
        id: "f2901591-466b-47cb-9b9b-45557e45e346".to_string(),
        user_id: "400e".to_string(),
    });
    let (session, set_session) = create_signal::<Option<Session>>(sess);

    view! {
        <Router>
            { provide_context(GlobContext::new(session)) }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбрасывает session на None
                <main class="pt-[100px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_session={set_session}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_session={set_session}/>} }/>
                        <Route path="/users" view=move || { Users }/>
                        <Route path="/post" view=move || { view!{<div>"Новая статья"</div>} }/>
                        <Route path="/post/:id" view=move || { view!{<Post/>} }/>
                        <Route path="/*" view=move || { view!{ <PageErrMsg>"Запрошенная страница не существует"</PageErrMsg>}}/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
