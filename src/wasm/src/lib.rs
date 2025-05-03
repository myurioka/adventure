mod game;
mod common;
use crate::common::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, window, InputEvent, XmlHttpRequest, Event, EventTarget, HtmlInputElement, MouseEvent};
use std::{cell::RefCell, rc::Rc};
use game::Game;
use game::StaticGame;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// main

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue>{
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move{
        let document = window().unwrap().document().unwrap();
        let game = Game::new(document);
        GameLoop::start(game)
            .await;
    });
    Ok(())
}

struct GameLoop;
impl GameLoop {

    pub async fn start(game: impl StaticGame + 'static) {
        log!("START");

        let ref_game = Rc::new(RefCell::new(game));

        // callback frame from JS

        {
            let closure = Rc::new(RefCell::new(None));
            let closure_cloned = Rc::clone(&closure);
            let ref_game_frame_cloned = Rc::clone(&ref_game);
            let mut frame = 0;

            closure_cloned.replace(Some(Closure::wrap(Box::new(move |_time: f64| {
                frame += 1;
                if frame % 5 == 0 {
                    ref_game_frame_cloned.borrow_mut().on_animation_frame();
                }
                request_animation_frame(closure.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut(f64)>)));
            request_animation_frame(closure_cloned.borrow().as_ref().unwrap());

            fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
                window()
                    .unwrap()
                    .request_animation_frame(f.as_ref().unchecked_ref())
                    .expect("should register `requestAnimationFrame` OK");
            }
        }

        // callback http request from JS

        let ref_game_cloned = Rc::clone(&ref_game);
        let _xhr = XmlHttpRequest::new().unwrap();
        let _xhr_cloned = Rc::new(RefCell::new(_xhr.clone()));

        {
            let ref_game_http_request_cloned = Rc::clone(&ref_game);

            let closure_http_request = Closure::wrap(Box::new( move |e: Event| {
                let xhr_target = e.target().unwrap();
                let xhr = xhr_target.dyn_ref::<XmlHttpRequest>().unwrap();
                if xhr.ready_state() == 4{
                    match xhr.status(){
                        Ok(200) => {
                            match xhr.response_text() {
                                Ok(Some(response_text)) => {
                                    match serde_json::from_str::<GeminiResponseBody>(&response_text) {
                                        Ok(parsed_response) => {
                                            if let Some(candidate) = parsed_response.candidates.first() {
                                                if let Some(part) = candidate.content.parts.first(){
                                                    ref_game_http_request_cloned.borrow_mut().on_http_request(part.text.clone());
                                                } else {
                                                    ref_game_http_request_cloned.borrow_mut().on_http_request("Error: No candidates found".to_string());
                                                }
                                            } else {
                                                log!("Error: No candidates found in resonse");
                                                ref_game_http_request_cloned.borrow_mut().on_http_request("Error: No candidates found".to_string());
                                            }
                                        }
                                        Err(e) => {
                                            log!("JSON Parse Error: {:?}", e);
                                            ref_game_http_request_cloned.borrow_mut().on_http_request(format!("Error parsing response: {}", e));
                                        }
                                    }
                                }
                                Ok(None) => {
                                    log!("Error reading response text: {:?}", e);
                                    ref_game_http_request_cloned.borrow_mut().on_http_request("Error: Empty response".to_string());
                                }
                                Err(e) => {
                                    log!("Error getting HTTP status: {:?}", e);
                                    ref_game_http_request_cloned.borrow_mut().on_http_request(format!("Network Error: {:?}", e));
                                }
                            }
                        }
                        Ok(status_code) => {
                            log!("HTTP Error: Status {}", status_code);
                            let error_text = xhr.response_text().unwrap_or(Some("Failed to get error details".to_string())).unwrap_or_default();
                            log!("Error Response Body: {}", error_text);
                            ref_game_http_request_cloned.borrow_mut().on_http_request(format!("HTTP Error: {}", status_code));
                        }
                        Err(e) => {
                            log!("Error Response HTTP status: {:?}", e);
                            ref_game_http_request_cloned.borrow_mut().on_http_request(format!("Network Error: {:?}", e));
                        }
                    }
                }

            }) as Box<dyn FnMut(_)>);

            let event_target: &EventTarget = _xhr.as_ref();
            event_target.add_event_listener_with_callback(
                "readystatechange",
                closure_http_request.as_ref().unchecked_ref()).unwrap();

            closure_http_request.forget();
        }

        // callback Input Event from JS

        {

            let closure_input = Closure::wrap(Box::new(move |e: InputEvent| {
                let input = e
                    .current_target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap();

                let _input_text = sanitize(input.value());

                let _api_endpoint = format!("{}",ref_game_cloned.borrow().get_api_endpoint());

                if _api_endpoint == "" {
                    let api_endpoint = format!("{}{}", GEMINI_API_ENDPOINT.to_string(), _input_text);
                    let _= ref_game_cloned.borrow_mut().set_api_endpoint(api_endpoint);
                    let _= ref_game_cloned.borrow_mut().next_page();
                    let _text = input.dyn_into::<HtmlInputElement>().unwrap();
                    _text.set_value("");
                    return;
                }

                let api_endpoint = format!("{}",ref_game_cloned.borrow().get_api_endpoint());
                           
                let _text = ref_game_cloned.borrow().create_prompt(_input_text);
                let _= ref_game_cloned.borrow_mut().next_page();
                let request_body = GeminiRequestBody {
                    contents: vec![GeminiRequestContent {
                    parts: vec![GeminiRequestPart {text: _text}],
                    }],
                };

                let payload = match serde_json::to_string(&request_body){
                    Ok(json) => json,
                    Err(e) => {
                    log!("Failed to serialize request body: {}", e);
                    return;
                    }
                };

                match _xhr_cloned.borrow().open("POST", &api_endpoint) {
                    Ok(_) => {
                        if let Err(e) = _xhr_cloned.borrow().set_request_header("Content-Type","application/json"){
                            log!("Failed to set Context-Type header: {:?}", e);
                            return;
                        }

                        match _xhr_cloned.borrow().send_with_opt_str(Some(&payload)) {
                            Ok(_) => {
                                log!("Request sent successfully.");
                            }
                            Err(e) => log!("Failed to send request: {:?}", e),
                        }
                    },
                    Err(e) => {
                        log!("Failed to open XHR request: {:?}", e);
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let _document = window().unwrap().document().unwrap();
            let _text = _document.get_element_by_id("input").unwrap();
            _text.add_event_listener_with_callback(
                "change",
                closure_input.as_ref().unchecked_ref(),
            ).unwrap();
            closure_input.forget();
        }

        // callback touch from JS

        {
            let ref_game_touch_cloned = Rc::clone(&ref_game);
            let c = Closure::wrap(Box::new(move |e:MouseEvent| {
                ref_game_touch_cloned.borrow_mut().on_click();
            }) as Box<dyn FnMut(_)>);
            let _document = window().unwrap().document().unwrap();
            let _canvas = _document.get_element_by_id("canvas").unwrap();
            _canvas.add_event_listener_with_callback(
                "mousedown",
                c.as_ref().unchecked_ref(),
            ).unwrap();
            c.forget();

            let ref_game_touch_textarea_cloned = Rc::clone(&ref_game);
            let d = Closure::wrap(Box::new(move |e:MouseEvent| {
                ref_game_touch_textarea_cloned.borrow_mut().on_click();
            }) as Box<dyn FnMut(_)>);
            let _document = window().unwrap().document().unwrap();
            let _canvas = _document.get_element_by_id("mytextarea").unwrap();
            _canvas.add_event_listener_with_callback(
                "mousedown",
                d.as_ref().unchecked_ref(),
            ).unwrap();
            d.forget();
        }

        // callback image load from JS

        {
            let ref_game_image_cloned = Rc::clone(&ref_game);

            wasm_bindgen_futures::spawn_local(async move {
                let _image = HtmlImageElement::new().unwrap();
                let f = Closure::once(Box::new(|| {
                    log!("IMAGE LOAD...");
                }));
                _image.set_onload(Some(f.as_ref().unchecked_ref()));
                f.forget();

                _image.set_src("screen.svg");
                let _result = wasm_bindgen_futures::JsFuture::from(_image.decode()).await;

                ref_game_image_cloned.borrow_mut().on_image(_image);
                ref_game.borrow().draw();
            });
        }
    }
}