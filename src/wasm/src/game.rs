use crate::common::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement, CanvasRenderingContext2d, Document, HtmlInputElement, HtmlTextAreaElement, HtmlElement, MouseEvent};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[derive(Debug, Clone)]
pub struct Game{
    document: Document,
    image: HtmlImageElement,
    page: usize,
    message: String,
    api_endpoint: String,
    mike: bool,
}
pub trait StaticGame {
    fn new(document: Document) -> Self;
    fn set_image(&mut self, image:HtmlImageElement);
    fn set_message(&mut self, text:String);
    fn set_api_endpoint(&mut self, _api_endpoint:String);
    fn set_mike_on(&mut self);
    fn set_mike_off(&mut self);
    fn next_page(&mut self);
    fn on_animation_frame(&mut self);
    fn on_image(&mut self, _image: HtmlImageElement);
    fn on_http_request(&mut self, response: String);
    fn on_click(&mut self); 
    fn on_input_changed(&mut self, transcript: &String);
    fn get_document(&self) -> Document;
    fn get_canvas(&self) -> HtmlCanvasElement;
    fn get_context(&self) -> CanvasRenderingContext2d;
    fn get_message(&self) -> String;
    fn get_page(&self) -> usize;
    fn get_page_type(&self) -> PageType;
    fn get_api_endpoint(&self) -> String;
    fn get_mike_status(&self) -> bool;
    fn update(&mut self);
    fn draw(&self);
    fn clear(&self);
    fn create_prompt(&self, _text:String) -> String;
}

impl StaticGame for Game{

    // init

    fn new(document: Document) -> Self{
        let _canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        let _screen_width = _canvas.client_width() as f32;
        let _screen_height = _canvas.client_height() as f32;
        let _image = HtmlImageElement::new().unwrap();

        Game {
            document: document,
            image: _image,
            page: 0,
            message: String::from(""),
            api_endpoint: String::from(""),
            mike: false,
        }
    }

    fn get_document(&self) -> Document{
        self.document.clone()
    }
    
    fn get_canvas(&self) -> HtmlCanvasElement{
        let _canvas = self.document.get_element_by_id("canvas").unwrap();
        _canvas.dyn_into::<HtmlCanvasElement>().unwrap()
    }
    fn get_context(&self) -> CanvasRenderingContext2d{
        self.get_canvas().get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap()
    }
    fn get_api_endpoint(&self) -> String {
        self.api_endpoint.clone()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
    fn get_page(&self) -> usize {
        self.page.clone()
    }
    fn get_page_type(&self) -> PageType{
        if self.page == 0 { return PageType::First; }
        if self.page == 15 { return PageType::Fin; }
        let _p = self.page % 2;
        match _p {
            0 => { PageType::Output },
            _ => { PageType::Input }
        }
    }
    fn get_mike_status(&self) -> bool {
        self.mike.clone()
    }
    fn set_image(&mut self, image:HtmlImageElement){
        self.image = image;
    }
    fn set_message(&mut self, message:String){
        self.message = message;
    }
    fn set_api_endpoint(&mut self, api_endpoint:String){
        self.api_endpoint = api_endpoint;
    }
    fn set_mike_on(&mut self){
        self.mike = true;
    }
    fn set_mike_off(&mut self){
        self.mike = false;
    }
    fn next_page(&mut self) {
        if self.page == LAST_PAGE {
            self.set_api_endpoint(String::from(""));
            self.page = 0;
            return
        }
        self.page +=  1;
    }

    // Speech recognition result

    fn on_input_changed(&mut self, transcript: &String){
        let _document = &self.get_document();
        let _input = _document.get_element_by_id("input").unwrap();
        let _text = _input.dyn_into::<HtmlInputElement>().unwrap();
        let _= _text.set_value(&transcript);
        let _= _text.focus();
        let _= self.set_mike_off();
    }

    // Gemini Prompt

    fn create_prompt(&self, _text:String) -> String {
        let _page = self.page;
        match _page {
            1 .. 18 => {
                let _chapter = (_page + 1) / 2;
                format!("{} の英訳は、{} で正しいですか？", TEXT_CHAPTER_TEXT_PLACEHOLDER[_chapter], _text)  
            },
            _ => {
                String::from("")
            }
        }
    }

    // callback image load

    fn on_image(&mut self, _image: HtmlImageElement) {
        let _= self.set_image(_image);
    }

    // callback http request

    fn on_http_request(&mut self, response: String) {
        self.set_message(response);
    }

    // game controller

    fn update(&mut self){
        let _image = HtmlImageElement::new().unwrap();
        self.clear();
        self.draw();
    }

    // callback animation

    fn on_animation_frame(&mut self) {
        self.update();
    }

    // callback click: controll page number

    fn on_click(&mut self) {

        // Mike Display ON/OFF

        let _page_type = self.get_page_type();

        match _page_type {
            PageType::First => {
                return;
            },
            PageType::Input => {
                let _= self.set_mike_on();
            },
            PageType::Output | PageType::Fin => {
                let _document = &self.get_document();
                let _input = _document.get_element_by_id("input").unwrap();
                let _text = _input.dyn_into::<HtmlInputElement>().unwrap();
                let _= _text.set_value("");
                let _= self.set_message(String::from(""));
                let _= self.set_mike_off();
                self.next_page();
            }
        }
    }

    // draw

    fn draw(&self){

        // Get Page
        let _context = self.get_context();
        let _page = self.get_page();
        let _chapter = (_page + 1) / 2; // page:1,2 -> chapter:1,  page:3,4 -> chapter:2 ...
        let _page_type = self.get_page_type();

        // Get Screen
        let _canvas = self.document.get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        let _canvas_width = self.get_canvas().width() as f64;
        let _canvas_height = self.get_canvas().height() as f64;
        let _canvas_top = self.get_canvas().client_top() as f64;
        let _canvas_left = self.get_canvas().client_left() as f64;

        // Get InputText
        let _document = &self.get_document();
        let _input = _document.get_element_by_id("input").unwrap();
        let _input_element = _input.dyn_into::<HtmlInputElement>().unwrap();
        let _= _input_element.style().set_property("background-color", "#D9FFB3");


        // Get Textarea
        let _mytextarea = _document.get_element_by_id("mytextarea").unwrap();
        let _textarea = _mytextarea.dyn_into::<HtmlElement>().unwrap();
        let _textarea_cloned = _textarea.clone();
        let _textarea_message = _textarea.dyn_into::<HtmlTextAreaElement>().unwrap();

        match _page_type {

            // Opening

            PageType::First => {
                // Title
                let _= _context.set_fill_style_str(DEFAULT_COLOR);
                let _= _context.set_font("36px MyFont");
                let _= _context.set_text_align("center");
                let _= _context.fill_text(TITLE, _canvas_width / 2.0, 90.0);
                let _= _context.set_font("18px MyFont");
                let _lines: Vec<&str> = TEXT_OPEN.split('\n').collect();
                // Intro 
                for i in 0.._lines.len(){
                    let _= _context.fill_text(_lines[i], _canvas_width / 2.0, (130.0 + (TEXT_SPACE * i) as f32).into());
                }
                // Mike
                let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 120.0, 900.0, 60.0,150.0, _canvas_width / 2.0 - 120.0 / 2.0, 210.0, 120.0, 300.0);
                // LITTLE RED RIDING HOOD & WOLFS
                let _= _context.set_stroke_style_str("rgba(-1,128, 0)");
                let _= _context.begin_path();
                let _= _context.close_path();
                let _= _context.stroke();
                let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 0.0,0.0,120.0,150.0,-60.0,340.0,240.0,300.0);
                let _ =  _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 0.0,150.0,120.0,150.0,140.0,360.0,260.0,320.0);
                let _ =  _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 120.0,0.0,120.0,150.0, 360.0,340.0,240.0,300.0);
                // INPUT TEXT
                let _= _input_element.set_disabled(false);
                let _= _input_element.set_placeholder(TEXT_CHAPTER_TEXT_PLACEHOLDER[0]);
            },

            // Finish

            PageType::Fin => {
                // Title
                let _= _context.set_fill_style_str(DEFAULT_COLOR);
                let _= _context.set_font("36px MyFont");
                let _= _context.set_text_align("center");
                let _= _context.fill_text(TITLE, _canvas_width / 2.0, 90.0);
                let _= _context.set_font("18px MyFont");
                let _lines: Vec<&str> = TEXT_OPEN.split('\n').collect();
                let _= _textarea_message.set_value("");
                // Illustration
                let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 120.0, 750.0, 120.0, 150.0, 230.0, 240.0, 120.0, 150.0);
                // TEXTAREA
                let _= _textarea_message.set_value("");
                let _= _textarea_cloned.style().set_property("display", "none");
            },

            // Contents

            PageType::Input | PageType::Output => {

                // Gray Disaplay 
                if PageType::Output == _page_type {_context.set_global_alpha(0.3);}
 
                // Title
                let _= _context.set_font("18px Hiragino Sans");
                let _= _context.set_fill_style_str(DEFAULT_COLOR);
                let _= _context.set_text_align("center");
                let _= _context.fill_text("【 LITTLE RED RIDING HOOD 】", _canvas_width / 2.0, 30.0);
                // CHAPTER
                let _= _context.set_text_align("left");
                let _lines: Vec<&str> = TEXT_CHAPTER[_chapter].split('\n').collect();
                for i in 0.._lines.len() {
                    let _=  _context.fill_text(_lines[i], 10.0, (70.0 + (TEXT_SPACE * i) as f32).into());
                }
                // Illustration
                let mut _f  = (0.0, 0.0, 0.0, 0.0);
                let mut _d = (0.0, 0.0, 0.0, 0.0);
                match _chapter {
                    1 => {
                        _f = (0.0, 300.0, 120.0, 150.0);
                        _d = (340.0, 440.0, 240.0, 300.0);
                    }
                    2 => {
                        _f = (120.0, 300.0, 120.0, 150.0);
                        _d = (300.0, 340.0, 240.0, 300.0);
                    }
                    3 => {
                        _f = (0.0, 450.0, 120.0, 150.0);
                        _d = (300.0, 340.0, 240.0, 300.0);
                    }
                    4 => {
                        _f = (120.0, 450.0, 120.0, 150.0);
                        _d = (360.0, 480.0, 220.0, 280.0);
                    }
                    5 => {
                        _f = (0.0, 600.0, 120.0, 150.0);
                        _d = (320.0, 440.0, 240.0, 300.0);
                    }
                    6 => {
                        _f = (120.0, 600.0, 120.0, 150.0);
                        _d = (330.0, 340.0, 240.0, 300.0);
                    }
                    7 => {
                        _f = (0.0, 750.0, 120.0, 150.0);
                        _d = (330.0, 380.0, 220.0, 280.0);
                    }
                    8 => {
                        _f = (120.0, 750.0, 120.0, 150.0);
                        _d = (230.0, 240.0, 120.0, 150.0);
                    }
                    _=> {}
                }
                let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, _f.0, _f.1, _f.2, _f.3, _d.0, _d.1, _d.2, _d.3);
                // CONTEXT
                let _lines: Vec<&str> = TEXT_CHAPTER[_chapter].split('\n').collect();
                for i in 0.._lines.len() {
                    let _=  _context.fill_text(_lines[i], 10.0, (70.0 + (TEXT_SPACE * i) as f32).into());
                }
                // Mike
                 _context.set_global_alpha(0.5);
                if self.get_mike_status() {
                    let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 60.0, 900.0, 60.0, 150.0, _canvas_width / 2.0 - 120.0 / 2.0, 210.0, 120.0, 300.0);
                } else {
                    _context.set_global_alpha(0.3);
                    let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &self.image, 0.0, 900.0, 60.0, 150.0, _canvas_width / 2.0 - 120.0 / 2.0, 210.0, 120.0, 300.0);
                    _context.set_global_alpha(1.0);
                }
                
                // Message from AI
                if _page_type == PageType::Output {
                    let _= _input_element.set_value("Touch or Click Screen");
                    let _= _input_element.set_disabled(true);

                    // border
                    let _= _context.set_global_alpha(1.0); 
                    let _= _context.begin_path();
                    let _= _context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &self.image,
                        125.0, 150.0, 90.0, 50.0, _canvas_left + 10.0, _canvas_top + 40.0, _canvas_width - 20.0, _canvas_height - 100.0);

                    //  Message
                    //let _mytextarea = _document.get_element_by_id("mytextarea").unwrap();
                    let _input = _document.get_element_by_id("input").unwrap();
                    let _message = self.get_message();
                    let _document = &self.get_document();
                    let _width = format!("{}px", _canvas_width - 20.0);
                    let _height = format!("{}px", _canvas_height - 40.0);
                    let _left = format!("{}px", _canvas_left as f32 + 50.0);
                    let _top = format!("{}px", _canvas_top as f32 + 50.0);
                    let _= _textarea_cloned.style().set_property("display", "block");
                    let _= _textarea_cloned.style().set_property("left",  &_left);
                    let _= _textarea_cloned.style().set_property("top",  &_top);
                    let _= _textarea_cloned.style().set_property("max-width", "860px");
                    let _= _textarea_cloned.style().set_property("width", &_width);
                    let _= _textarea_cloned.style().set_property("height", &_height);
                    let _= _textarea_cloned.style().set_property("visibility", "visible");
                    let _= _textarea_message.set_value(&_message);
                }
                if _page_type == PageType::Input {
                    // INPUT TEXT
                    let _= _input_element.set_disabled(false);
                    let _= _input_element.set_placeholder(TEXT_CHAPTER_TEXT_PLACEHOLDER[_chapter]);
                    // TEXTAREA
                    let _= _textarea_message.set_value("");
                    let _= _textarea_cloned.style().set_property("display", "none");
                }
            }
        }
     }

    // clear screen

    fn clear(&self){
        let _context = self.get_context();
         _context.clear_rect(
            0.0,
            0.0,
            1000.0 as f64,
            1000.0 as f64,
        );
    }
}