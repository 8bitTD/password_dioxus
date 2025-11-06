
use dioxus::prelude::*;
use dioxus::desktop::use_wry_event_handler;

use super::app;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let mut app = use_signal(|| app::App::default());
    use_future(move || async move {
        loop {
            async_io::Timer::after(std::time::Duration::from_millis(10)).await;
            if app().opacity == 0.0{continue;}
            app.write().opacity -= 0.02;
            if app().opacity < 0.0 {app.write().opacity = 0.0;}
        }
    });
    use_wry_event_handler(move |event, _| { app::event_handler(event, app); });//ウィンドウ変更時の記録用処理
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {app}
    }
}

#[component]
pub fn Hero(app: Signal<app::App>) -> Element {
    rsx! {
        div {
            class: "h-screen flex justify-center items-center",
            div {
                class: "w-screen p-5 flex-col",
                div {
                    class: "flex justify-center items-center gap-4",
                    label { 
                        class: "w-2/10 p-4 text-right",
                        "{app().json.digit} 桁:"
                    }
                    input {
                        class: "w-7/10",
                        min: 1,
                        max: 30,
                        type: "range",
                        value: app().json.digit,
                        oninput: move|e|{
                            app.write().json.digit = e.data.value().parse::<usize>().unwrap();
                            app.write().password = app::create_password(app().json.digit);
                        }
                    }
                    label { 
                        class: "w-1/10 text-center text-lg translate-y-7",
                        opacity: "{app().opacity}",
                        transform: format!("translateY({}px)",-20.0*(1.0-app().opacity)),
                        "copy!"
                    }
                }
                div {
                    class: "flex justify-center items-center gap-4",
                    label { 
                        class: "w-2/10 p-4 text-right",
                        "パスワード:" 
                    }
                    input {  
                        class: "w-7/10 text-center p-2 text-lg font-bold rounded-lg",
                        type: "text",
                        value: "{app().password}"
                    }
                    button { 
                        class: "w-1/10  rounded-lg flex justify-center items-center",
                        onclick: move |_| {
                            app::set_clipboard(app().password.to_string());
                            app.write().opacity = 1.0;
                        },
                        img {
                            class: "",
                            width: 32,
                            height: 32,
                            src: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAACXBIWXMAAAsTAAALEwEAmpwYAAAA7UlEQVR4nO3WMQ7CMAwF0FyiQ4+DDwEn4AqcoaNnmDvDEZi7gJhYPbOwBKlIbdhRh5Y4MW79pb/7SV9RnLNY0gRqH2K7qn2lFrA+vmQREAm4PLqwOQkiIBJAvpdFAANAFAFMADEEMAJEEMAMyI6ACcdi03K0UgvYX9/8iJyA+7MLB25ETgD5nh+RG0DciKFDi+15sFwA4kRIAYgLITEh4kRIA+gL8dcTwhE1AKT/SgQDcE6o3N1+LhrAzXRCoB1QJJoQGmApE4IlAUrtz2ipHYDaJ4TSgCmJnRAaIDKznBAaoDXA6BigEQZYLE5vPlC47isG1kXRAAAAAElFTkSuQmCC",
                        }                        
                    }
                }
                div {
                    class: "flex justify-center mt-4",
                    button { 
                        class: "w-1/3 h-10",
                        onclick: move |_| {
                            app.write().password = app::create_password(app().json.digit);
                        },
                        "パスワード 作成"
                    }
                }
            }
        }
    }
}