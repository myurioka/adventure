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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "webkitSpeechRecognition")]
    pub type WebkitSpeechRecognition;

    #[wasm_bindgen(constructor, js_class = "webkitSpeechRecognition")]
    pub fn new() -> WebkitSpeechRecognition;

    #[wasm_bindgen(method, js_name = "start")]
    pub fn start(this: &WebkitSpeechRecognition);

    #[wasm_bindgen(method, js_name = "stop")]
    pub fn stop(this: &WebkitSpeechRecognition);

    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(
        this: &WebkitSpeechRecognition,
        event: &str,
        callback: &Closure<dyn FnMut(JsValue)>,
    );

    #[wasm_bindgen(method, setter, js_name = "lang")]
    pub fn set_lang(this: &WebkitSpeechRecognition, value: &str);

    #[wasm_bindgen(method, setter, js_name = "interimResults")]
    pub fn set_interim_results(this: &WebkitSpeechRecognition, value: bool);

    #[wasm_bindgen(method, setter, js_name = "continuous")]
    pub fn set_continuous(this: &WebkitSpeechRecognition, value: bool);
}

struct GameLoop;
impl GameLoop {

    pub async fn start(game: impl StaticGame + 'static) {
        log!("START");

        let ref_game = Rc::new(RefCell::new(game));

        // callback WebkitSpeechRecognition from JS

        let game_ref_for_speech = Rc::clone(&ref_game);
        let recognition = WebkitSpeechRecognition::new();
        let ref_recognition = Rc::new(RefCell::new(recognition));
        let ref_recognition_cloned = Rc::clone(&ref_recognition);

        ref_recognition.borrow_mut().set_lang("ja-JP");         // en-US
        ref_recognition.borrow_mut().set_interim_results(true); // 途中結果も取得する
        ref_recognition.borrow_mut().set_continuous(false); // 発話が一度途切れたら認識を終了 (true:continue)

        // speach recognition onresult
        let on_result = Closure::wrap(Box::new(move |event: JsValue| {
            log!("Speech recognition 'result' event kick.");

            let results = match js_sys::Reflect::get(&event, &JsValue::from_str("results")) {
                Ok(r) => r,
                Err(_) => {
                    log!("Failed to get 'results' from SpeechRecognitionEvent");
                    return;
                }
            };

            // `results` は SpeechRecognitionResultList
            let result_list_length = match js_sys::Reflect::get(&results, &JsValue::from_str("length")).ok().and_then(|l| l.as_f64()) {
                Some(len) => len as u32,
                None => {
                    log!("Failed to get 'length' from SpeechRecognitionResultList or it's not a number");
                    return;
                }
            };

            // interimResults が true の場合、複数の結果が含まれることがある
            // 通常、最後の結果が最新のもの
            if result_list_length > 0 {
                let last_result_item_index = result_list_length -1;
                let result_item = match js_sys::Reflect::get_u32(&results, last_result_item_index) {
                    Ok(item) => item,
                    Err(_) => {
                        log!("Failed to get result item at index {}", last_result_item_index);
                        return;
                        }
                };
                // `result_item` は SpeechRecognitionResult

                // 最初の候補 (SpeechRecognitionAlternative) を取得 (通常これが最も確からしい)
                let alternative = match js_sys::Reflect::get_u32(&result_item, 0) {
                    Ok(alt) => alt,
                    Err(_) => {
                        log!("Failed to get alternative from result item at index {}", last_result_item_index);
                        return;
                    }
                };

                let transcript = js_sys::Reflect::get(&alternative, &JsValue::from_str("transcript"))
                    .ok().and_then(|v| v.as_string()).unwrap_or_default();
                let confidence = js_sys::Reflect::get(&alternative, &JsValue::from_str("confidence"))
                    .ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
                   
                // isFinal は、この結果が最終的なものか (true)、それとも中間結果か (false) を示す
                let is_final = js_sys::Reflect::get(&result_item, &JsValue::from_str("isFinal"))
                    .ok().and_then(|v| v.as_bool()).unwrap_or(false);

                //log!("Transcript (isFinal: {}): \"{}\", Confidence: {:.2}", is_final, transcript, confidence);

                // 確定した結果 (isFinal: true) で、かつマイクがオンの場合に入力フィールドに反映する例
                if is_final {
                    let game_borrow = game_ref_for_speech.borrow(); // game_ref_for_speech を使用
                    if game_borrow.get_mike_status() { // マイクがオンの時だけ処理
                        let document = window().unwrap().document().unwrap();
                        if let Some(input_el_val) = document.get_element_by_id("input") {
                            if let Ok(input_el) = input_el_val.dyn_into::<HtmlInputElement>() {
                                input_el.set_value(&transcript);
                                //game_ref_for_speech.borrow_mut().on_input_changed(&transcript);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        ref_recognition.borrow_mut().add_event_listener("result", &on_result);
        on_result.forget();

        // speech recognition onerror
        let on_error = Closure::wrap(Box::new(move |error_event: JsValue| {
            // `error_event` は JavaScript の SpeechRecognitionErrorEvent オブジェクト
            let error_type = js_sys::Reflect::get(&error_event, &JsValue::from_str("error"))
                .unwrap_or_else(|_| JsValue::from_str("unknown error"))
                .as_string()
                .unwrap_or_else(|| "unknown error".to_string());
            log!("Speech recognition error: {}", error_type);
        }) as Box<dyn FnMut(JsValue)>);

        ref_recognition.borrow_mut().add_event_listener("error", &on_error);
        on_error.forget();

        // speech recognition onstart

        let on_start = Closure::wrap(Box::new(move |_: JsValue| {
            log!("Speech recognition service has started.");
        }) as Box<dyn FnMut(JsValue)>);
        ref_recognition.borrow_mut().add_event_listener("start", &on_start);
        on_start.forget();

        // speech recognition onend

        let ref_game_mike_cloned = Rc::clone(&ref_game);
        let on_end = Closure::wrap(Box::new(move |_: JsValue| {
            log!("Speech recognition service has stopped.");
            ref_game_mike_cloned.borrow_mut().set_mike_off();
            ref_recognition.borrow_mut().stop();
        }) as Box<dyn FnMut(JsValue)>);
        ref_recognition_cloned.borrow_mut().add_event_listener("end", &on_end);
        on_end.forget();

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
                log!("callback Input Event");
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

                // Start Recognition

                ref_recognition_cloned.borrow().start();
                ref_game_touch_cloned.borrow_mut().set_mike_changed();
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
                ref_game_touch_textarea_cloned.borrow_mut().on_click(e);
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