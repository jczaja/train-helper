use macroquad::prelude::*;
use macroquad::ui::{root_ui};

mod skm;

#[macroquad::main("TrainHelper")]
async fn main() {

    let to_work_message = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            "Gdansk Wrzeszcz".to_string(),
            "Gdansk Port Lotniczy".to_string(),
        ],
    ).submit();
    let to_work_message = match to_work_message {
        Ok(mesg) => format!("Train to work {}", mesg),
        Err(err_msg) => err_msg,
    };
    let home_message = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            "Gdansk Port Lotniczy".to_string(),
            "Gdansk Wrzeszcz".to_string(),
        ],
    ).submit();
    let home_message = match home_message {
        Ok(mesg) => format!("Train home {}", mesg),
        Err(err_msg) => err_msg,
    };
    let mut myskin = root_ui().default_skin().clone();
    myskin.label_style = root_ui().style_builder().font_size(30).build(); 
    root_ui().push_skin(&myskin);
    loop {
        clear_background(WHITE);

        root_ui().label(None, &to_work_message);
        root_ui().label(None, &home_message);
        if root_ui().button(None, "Exit") {
           println!("pushed");
           return;
        }
        next_frame().await
    }
}
