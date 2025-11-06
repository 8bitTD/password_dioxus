use dioxus::prelude::*;
use dioxus::desktop::*;
use dioxus::desktop::tao::event::Event;
use rand::Rng;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use super::json::*;

#[derive(Debug, Clone)]
pub struct App{
    pub json: Json,
    pub opacity: f32,
    pub password: String,
}

impl Default for App{
    fn default() -> App{
        let json = Json::new();
        let password = create_password(json.digit);
        App { 
            json: json,
            opacity: 0.0,
            password: password,
        }
    }
}


pub fn event_handler<UserWindowEvent>(event: &Event<UserWindowEvent>, mut app: Signal<App>){
    if let Event::WindowEvent{//ウィンドウサイズ変更時の処理
        event: WindowEvent::Resized(size),
        ..
    } = event {
        app.write().json.wi.width = size.width;
        app.write().json.wi.height = size.height;
    }
    if let Event::WindowEvent{//ウィンドウ位置変更時の処理
        event: WindowEvent::Moved(pos),
        ..
    } = event {
        app.write().json.wi.pos_x = pos.x;
        app.write().json.wi.pos_y = pos.y;
    }
    if let Event::WindowEvent{//exe終了時に情報保存する処理
        event: WindowEvent::CloseRequested, 
        ..
    } = event {
        app().json.save();
    }  
}

pub fn create_password(digit: usize) -> String{//指定された桁のランダムなパスワードを作成する処理
    let mut rng = rand::rng();
    let chars: String = (0..digit).map(|_| rng.sample(rand::distr::Alphanumeric) as char).collect();
    return chars;
}

pub fn set_clipboard(mozi: String){//クリップボードに文字を保存する処理
    let tmp: Result<ClipboardContext, _> = ClipboardProvider::new();
    if tmp.is_err(){return;}
    let mut ctx = tmp.unwrap();
    let _ = ctx.set_contents(mozi);
}

pub fn load_icon_from_url(url: &str) -> Option<tao::window::Icon>{//指定したURLのアイコンを取得する処理
    let Ok(response) = reqwest::blocking::get(url) else {return None};
    let bytes = response.bytes().unwrap();
    let Ok(img) = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format() else {return None};
    let Ok(dyim) = img.decode() else {return None};
    let pixels = dyim.as_bytes().to_vec();
    let width = dyim.width();
    let height = dyim.height();
    let Ok(ico) = tao::window::Icon::from_rgba(pixels, width, height) else {return None};
    return Some(ico); 
}