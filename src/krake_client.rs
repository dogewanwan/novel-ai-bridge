use crate::error::BridgeError;
use arc_swap::ArcSwap;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokenizers::Tokenizer;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct KrakeClient {
    bearer: Arc<String>,
    tokenizer: Arc<Tokenizer>,
    client: reqwest::Client,
    last_request_time: Arc<ArcSwap<SystemTime>>,
    semaphore: Arc<Semaphore>,
}

#[derive(Debug, Clone, Deserialize)]
struct KrakeResponse {
    output: String,
}

const BAD_TOKENS: &'static [[u32; 1]] = &[
    [60],
    [62],
    [544],
    [683],
    [696],
    [880],
    [905],
    [1008],
    [1019],
    [1084],
    [1092],
    [1181],
    [1184],
    [1254],
    [1447],
    [1570],
    [1656],
    [2194],
    [2470],
    [2479],
    [2498],
    [2947],
    [3138],
    [3291],
    [3455],
    [3725],
    [3851],
    [3891],
    [3921],
    [3951],
    [4207],
    [4299],
    [4622],
    [4681],
    [5013],
    [5032],
    [5180],
    [5218],
    [5290],
    [5413],
    [5456],
    [5709],
    [5749],
    [5774],
    [6038],
    [6257],
    [6334],
    [6660],
    [6904],
    [7082],
    [7086],
    [7254],
    [7444],
    [7748],
    [8001],
    [8088],
    [8168],
    [8562],
    [8605],
    [8795],
    [8850],
    [9014],
    [9102],
    [9259],
    [9318],
    [9336],
    [9502],
    [9686],
    [9793],
    [9855],
    [9899],
    [9955],
    [10148],
    [10174],
    [10943],
    [11326],
    [11337],
    [11661],
    [12004],
    [12084],
    [12159],
    [12520],
    [12977],
    [13380],
    [13488],
    [13663],
    [13811],
    [13976],
    [14412],
    [14598],
    [14767],
    [15640],
    [15707],
    [15775],
    [15830],
    [16079],
    [16354],
    [16369],
    [16445],
    [16595],
    [16614],
    [16731],
    [16943],
    [17278],
    [17281],
    [17548],
    [17555],
    [17981],
    [18022],
    [18095],
    [18297],
    [18413],
    [18736],
    [18772],
    [18990],
    [19181],
    [20095],
    [20197],
    [20481],
    [20629],
    [20871],
    [20879],
    [20924],
    [20977],
    [21375],
    [21382],
    [21391],
    [21687],
    [21810],
    [21828],
    [21938],
    [22367],
    [22372],
    [22734],
    [23405],
    [23505],
    [23734],
    [23741],
    [23781],
    [24237],
    [24254],
    [24345],
    [24430],
    [25416],
    [25896],
    [26119],
    [26635],
    [26842],
    [26991],
    [26997],
    [27075],
    [27114],
    [27468],
    [27501],
    [27618],
    [27655],
    [27720],
    [27829],
    [28052],
    [28118],
    [28231],
    [28532],
    [28571],
    [28591],
    [28653],
    [29013],
    [29547],
    [29650],
    [29925],
    [30522],
    [30537],
    [30996],
    [31011],
    [31053],
    [31096],
    [31148],
    [31258],
    [31350],
    [31379],
    [31422],
    [31789],
    [31830],
    [32214],
    [32666],
    [32871],
    [33094],
    [33376],
    [33440],
    [33805],
    [34368],
    [34398],
    [34417],
    [34418],
    [34419],
    [34476],
    [34494],
    [34607],
    [34758],
    [34761],
    [34904],
    [34993],
    [35117],
    [35138],
    [35237],
    [35487],
    [35830],
    [35869],
    [36033],
    [36134],
    [36320],
    [36399],
    [36487],
    [36586],
    [36676],
    [36692],
    [36786],
    [37077],
    [37594],
    [37596],
    [37786],
    [37982],
    [38475],
    [38791],
    [39083],
    [39258],
    [39487],
    [39822],
    [40116],
    [40125],
    [41000],
    [41018],
    [41256],
    [41305],
    [41361],
    [41447],
    [41449],
    [41512],
    [41604],
    [42041],
    [42274],
    [42368],
    [42696],
    [42767],
    [42804],
    [42854],
    [42944],
    [42989],
    [43134],
    [43144],
    [43189],
    [43521],
    [43782],
    [44082],
    [44162],
    [44270],
    [44308],
    [44479],
    [44524],
    [44965],
    [45114],
    [45301],
    [45382],
    [45443],
    [45472],
    [45488],
    [45507],
    [45564],
    [45662],
    [46265],
    [46267],
    [46275],
    [46295],
    [46462],
    [46468],
    [46576],
    [46694],
    [47093],
    [47384],
    [47389],
    [47446],
    [47552],
    [47686],
    [47744],
    [47916],
    [48064],
    [48167],
    [48392],
    [48471],
    [48664],
    [48701],
    [49021],
    [49193],
    [49236],
    [49550],
    [49694],
    [49806],
    [49824],
    [50001],
    [50256],
    [0],
    [1],
];

impl KrakeClient {
    pub fn new(
        path_to_file: &str,
        bearer: &str,
    ) -> Result<KrakeClient, Box<dyn Error + Send + Sync>> {
        let tokenizer = Arc::new(Tokenizer::from_file(path_to_file)?);
        let client = reqwest::Client::new();

        Ok(KrakeClient {
            bearer: Arc::new(bearer.to_string()),
            tokenizer,
            client,
            last_request_time: Arc::new(ArcSwap::new(Arc::new(SystemTime::now()))),
            semaphore: Arc::new(Semaphore::const_new(1)),
        })
    }

    fn create_json_for_request(&self, input: &str) -> serde_json::Value {
        let bad_tokens = BAD_TOKENS;
        let json = json!(
            {
                "input": input,
                "model": "krake-v2",
                "parameters": {
                    "temperature": 1.33,
                    "max_length": 40,
                    "min_length": 1,
                    "top_p": 0.88,
                    "top_a": 0.085,
                    "typical_p": 0.965,
                    "tail_free_sampling": 0.937,
                    "repetition_penalty": 1.05,
                    "repetition_penalty_range": 560,
                    "repetition_penalty_frequency": 0,
                    "repetition_penalty_presence": 0,
                    "bad_words_ids": bad_tokens,
                    "generate_until_sentence": true,
                    "use_cache": false,
                    "use_string": false,
                    "return_full_text": false,
                    "prefix": "general_crossgenre",
                    "order": [
                        3,
                        4,
                        5,
                        2,
                        0
                    ]
                }
            }
        );
        json
    }

    pub async fn generate(&self, text: &str) -> Result<String, BridgeError> {
        let response = {
            let _ = self.semaphore.acquire().await?;
            let elapsed = self.last_request_time.load().elapsed()?;

            if elapsed < Duration::from_secs(1) {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            let input = self.encode(text)?;
            let json = self.create_json_for_request(&input);

            let url = "https://api.novelai.net/ai/generate";
            let response = self
                .client
                .post(url)
                .header("Authorization", format!("Bearer {}", self.bearer))
                .header("Content-Type", "application/json")
                .json(&json)
                .send()
                .await?
                .json::<KrakeResponse>()
                .await?;

            self.last_request_time.store(Arc::new(SystemTime::now()));
            response
        };

        Ok(self.decode(&response.output)?)
    }

    pub fn encode(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let tokens = self.tokenizer.encode_char_offsets(input, true)?;

        let bytes: Vec<_> = tokens
            .get_ids()
            .iter()
            .flat_map(|x| (*x as u16).to_le_bytes().into_iter())
            .collect();
        let input = base64::encode(&bytes);

        Ok(input)
    }

    pub fn decode(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let bytes = base64::decode(input)?;

        let results = bytes
            .chunks_exact(2)
            .map(|x| u16::from_le_bytes([x[0], x[1]]) as u32)
            .collect::<Vec<_>>();

        let decoded_str = self.tokenizer.decode(results, false)?;
        Ok(decoded_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_correctly() {
        let client = KrakeClient::new("tokenizers/pile_tokenizer.json", "").unwrap();

        let expected = "PwMGB2wFsgHnGyAA3QJPDVYB8EPaQoMBmFB/B/0A7RP1JMsCuQIvSUcB/QD8QQ8AfAETIT4Cyw2wAfYKGQG9Eg8AWBN3CGABAQZAAd8K6w0jEJcBsgK4AzkBWChbAVYEmQL4EVICBg46AVUCHQRyAvYD+Q9oAo02";
        let string = client.decode(&expected).unwrap();
        let encoded = client.encode(&string).unwrap();

        assert_eq!(expected, encoded);
    }

    #[test]
    fn should_decode_correctly() {
        let client = KrakeClient::new("tokenizers/pile_tokenizer.json", "").unwrap();

        let string = "PwMGB2wFsgHnGyAA3QJPDVYB8EPaQoMBmFB/B/0A7RP1JMsCuQIvSUcB/QD8QQ8AfAETIT4Cyw2wAfYKGQG9Eg8AWBN3CGABAQZAAd8K6w0jEJcBsgK4AzkBWChbAVYEmQL4EVICBg46AVUCHQRyAvYD+Q9oAo02";
        let decoded = client.decode(string).unwrap();

        assert_eq!(" This particular day's duty? It began with washing dishes at dawn until the sun rose into its glory on the horizon. The sky had turned from black to blue. Even though it might be considered late morning by some people (such as those who sleep so soundly they don't even hear their alarm", decoded);
    }
}
