pub mod skm {
    use regex::Regex;

    pub struct SKM {
        skm_url: String,
        proxy: Option<Vec<String>>,
        from_to: Vec<(Vec<String>, String)>,
    }

    impl SKM {
        pub fn new(
            skm_url: String,
            proxy: Option<Vec<String>>,
            from_to: Vec<(Vec<String>, String)>,
        ) -> Self {
            SKM {
                skm_url: skm_url,
                proxy: proxy,
                from_to: from_to,
            }
        }

        fn get_station_id<'a>(&self, body: &'a str, station: &str) -> &'a str {
            // Replace white characters with commas
            let re = Regex::new(r"\s+").unwrap();
            let mut t = re.replace_all(station, ",").to_lowercase();
            if t.contains("lotniczy") {
                t += ",lotnisko";
            }

            // We connstruct search pattern. for example:
            // "data-keywords="gdansk,wrzeszcz" value=\""
            let search_phrase = "data-keywords=\"".to_string() + &t + "\" value=";
            let id_offset_start = body
                .find(&search_phrase)
                .expect(&format!("Pattern: {}", search_phrase))
                + search_phrase.len()
                + 1;
            let pattern_slice = &body[id_offset_start..];
            let id_offset_end = pattern_slice.find('"').expect("Id pattern not found");
            // SO I need to extract: "<number>" and the parse <number> to get value
            &pattern_slice[0..id_offset_end]
        }

        fn get_message(&self, body: &str, station: &str) -> String {
            // We connstruct search pattern to get remaining time. for example:
            // Najbliższa kolejka za</p>
            //<h3 class="no-print">28 min</h3>

            let search_phrase = "Najbl".to_string();
            let return_string: String = match body.find(&search_phrase) {
                Some(start_offset) => {
                    let pattern_slice = &body[start_offset..start_offset + 400]; // 400 characters should be enough
                                                                                 // find first two "dd min"
                    let mut next_train_minutes: String = "".to_owned();
                    Regex::new(r"[0-9]+\s[m][i][n]")
                        .unwrap()
                        .find_iter(pattern_slice)
                        .for_each(|x| {
                            next_train_minutes += x.as_str();
                            next_train_minutes += ", "
                        });

                    "  (".to_string() + station + " --> ) departs in " + &next_train_minutes
                }
                None => "No connections today".to_owned(),
            };
            return_string
        }


        async fn get_info(&self, x : Vec<String> ,msg_prefix : String, messages: &mut Vec<String>, actual_response : &str, client : &reqwest::Client) {

                let from = self.get_station_id(&actual_response, &x[0]);
                let to = self.get_station_id(&actual_response, &x[1]);
                // Get Data

                let from_id = from;
                let to_id = to;

                // Lets get current data and time
                let date = chrono::Local::now().format("%Y-%m-%d").to_string();
                let hour = chrono::Local::now().format("%H").to_string();
                let minutes = chrono::Local::now().format("%M").to_string();

                // Send a request to SKM web page
                let request = "".to_string()
                    + &self.skm_url
                    + "/rozklad/?from="
                    + from_id
                    + "&to="
                    + to_id
                    + "&date="
                    + &date
                    + "&hour="
                    + &hour
                    + "%3A"
                    + &minutes;

                // Get actual times for our chosen destination
                let res = client
                    .get(&request)
                    .send()
                    .await
                    .expect("Error sending SKM request")
                    .text();

                let actual_response = res.await.expect("Error: unwrapping SKM response");
                messages.push(msg_prefix.to_owned());
                messages.push(self.get_message(&actual_response, &x[0]));
        }

        pub async fn submit(&self) -> Result<Vec<String>, String> {
            // If there is proxy then pick first URL
            let client = reqwest::Client::new();

            // Get IDs of stations e.g. Gdansk Wrzeszcz : 7534
            let res = client.get(&(self.skm_url.clone())).send().await;

            // HERE is fine to return
            // Returning here is fine
            let res = match res{
                Ok(result) => result.text(),
                Err(i) => return Err(format!("Error sending SKM request: {}", i)),
            };

            let actual_response = res.await.expect("Error: unwrapping SKM response");

            let mut messages: Vec<String> = vec![];

//            let mut myfutures : Vec<impl > = Vec::new(); 
            let mut myfutures : Vec<_> = Vec::new(); 

            // TODO compiling
            for (x, msg_prefix) in self.from_to {
                myfutures.push(self.get_info(x, msg_prefix, &mut messages, &actual_response,&client));
            };

            futures::future::join_all(myfutures);

            Ok(messages)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
        type GenericResult<T> = Result<T, GenericError>;

        #[test]
        fn test_skm() -> Result<(), String> {
            let skm = SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                None,
                vec![(
                    vec![
                        "Gdansk Wrzeszcz".to_string(),
                        "Gdansk Port Lotniczy".to_string(),
                    ],
                    format!("Train to work "),
                )],
            )
            .submit()?;
            Ok(())
        }

        #[test]
        fn test_parsing_message() -> GenericResult<()> {
            // Let's read data to parse from stored file
            let mut file = std::fs::File::open("data/test_data.txt")?;

            let mut s = String::new();
            file.read_to_string(&mut s)?;

            let response = SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                None,
                vec![(
                    vec![
                        "Gdansk Wrzeszcz".to_string(),
                        "Gdansk Port Lotniczy".to_string(),
                    ],
                    format!("Train to work:"),
                )],
            )
            .get_message(&s, "Gdansk Wrzeszcz");
            let expected_response = "  (Gdansk Wrzeszcz --> ) departs in 16 min, 26 min, 80 min, ";
            assert_eq!(response, expected_response);
            Ok(())
        }

        #[test]
        fn test_parsing_message_missing() -> GenericResult<()> {
            // Let's read data to parse from stored file
            let mut file = std::fs::File::open("data/test2_data.txt")?;

            let mut s = String::new();
            file.read_to_string(&mut s)?;

            let response = SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                None,
                vec![(
                    vec![
                        "Gdansk Wrzeszcz".to_string(),
                        "Gdansk Port Lotniczy".to_string(),
                    ],
                    format!("Train to work "),
                )],
            )
            .get_message(&s, "Gdansk Wrzeszcz");
            let expected_response = "No connections today";
            assert_eq!(response, expected_response);
            Ok(())
        }
    }
}
