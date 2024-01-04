mod skm;
mod ztm;
use std::cell::RefCell;
use std::rc::Rc;

async fn get_requests() -> (
    Result<Rc<RefCell<Vec<(String, u32)>>>, String>,
    Result<Rc<RefCell<Vec<(String, u32)>>>, String>,
) {
    let try_skm_messages = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            (
                vec![
                    "Gdansk Port Lotniczy".to_string(), //TODO: Add support to Firoga
                    "Gdansk Wrzeszcz".to_string(),
                ],
                format!("Gdansk Wrzeszcz "),
            ),
            (
                vec![
                    "Gdansk Port Lotniczy".to_string(), //TODO: Add support to Firoga
                    "Gdynia Glowna".to_string(),
                ],
                format!("Gdynia Glowna "),
            ),
        ],
    );

    // busses
    let try_ztm_messages = ztm::ztm::ZTM::new(
        None,
        vec![
        //            (
        //                "1482", // ID of bus stop
        //                vec![158, 258],
        //                "Bus to Parkour:\n",
        //            ),
        //            (
        //                "1404", // ID of bus stop
        //                vec![110],
        //                "Bus to Gdansk Wrzeszcz (Through IKEA):\n",
        //            ),
        //            (
        //                "1404", // ID of bus stop
        //                vec![210],
        //                "Bus to Gdansk Glowny (Through IKEA):\n",
        //            ),
        //            (
        //                "1404", // ID of bus stop
        //                vec![122],
        //                "Bus to Sopot (Through IKEA):\n",
        //            ),
                ],
    );

    (
        try_skm_messages.submit().await,
        try_ztm_messages.submit().await,
    )
}

pub fn get_messages() -> (Vec<String>, Vec<String>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Error: Unable to init runtime");
    let (try_skm_messages, try_ztm_messages) = rt.block_on(get_requests());

    let skm_messages = match try_skm_messages {
        Ok(msgs) => {
            msgs.borrow_mut()
                .sort_by(|(_a1, a2), (_b1, b2)| a2.partial_cmp(b2).unwrap());
            let just_messages: Vec<String> =
                msgs.borrow().clone().into_iter().map(|(a, _b)| a).collect();
            just_messages
        }
        Err(err_msg) => vec![(err_msg)],
    };

    let ztm_messages = match try_ztm_messages {
        Ok(msgs) => {
            msgs.borrow_mut()
                .sort_by(|(_a1, a2), (_b1, b2)| a2.partial_cmp(b2).unwrap());
            let just_messages: Vec<String> =
                msgs.borrow().clone().into_iter().map(|(a, _b)| a).collect();
            just_messages
        }
        Err(err_msg) => vec![err_msg],
    };

    (skm_messages, ztm_messages)
}
