/* to-do
make hotkeys for letters to type
allow the user to set colours for letters?
do not show zeros in the beginning
make entry field for typing
when letter is grey, apply to all letters, incl keyboard
*/

use yew::prelude::*;

use std::collections::HashMap;
//use hotkey;

pub struct Home {
    attempt: Vec<String>,
    attempts: Vec<Vec<String>>,
    results_wordle: Vec<Vec<String>>,
    results_master: Vec<Vec<u32>>,
    word: Vec<String>,
    occurrences: HashMap<String, u32>,
    letterstyles: HashMap<String, u32>,
}

pub enum Msg {
    Letter(String),
    Style(usize, usize),
    Enter,
    Backspace,
}

impl Home {
    fn input(&mut self, letter: String) {
        if self.attempt.len() < 5 {
            self.attempt.push(letter);
        }
    }

    fn check_word(&mut self) -> bool {
        let mut attempt = String::from("");
        for i in 0..5 {
            attempt.push_str(&self.attempt[i]);
        }
        for word in WORDS_RAW {
            if &attempt == word {
                return true;
            }
        }
        false
    }

    fn set_style(&mut self, a: usize, l: usize) {
        if a < self.attempts.len() {
            let letter = self.attempts[a][l].clone();
            let count = self.letterstyles.entry(letter).or_insert(0);
            *count = ((*count + 1) % 4) as u32;
        }
    }

    fn guessletter_style(&self, a: usize, l: usize) -> String {
        if a < self.attempts.len() {
            let letter = self.attempts[a][l].clone();
            match self.letterstyles.get(&letter) {
                Some(1) => format!(
                    "border-radius:4px;top: {}px; left:{}px; background-color: Orange;",
                    10 + 90 * (a % 6),
                    65 + 50 * l + 315 * (a as f64 / 6 as f64) as usize
                ),
                Some(2) => format!(
                    "border-radius:4px;top: {}px; left:{}px; background-color: Green;",
                    10 + 90 * (a % 6),
                    65 + 50 * l + 315 * (a as f64 / 6 as f64) as usize
                ),
                Some(3) => format!(
                    "border-radius:4px;top: {}px; left:{}px; background-color: Grey;",
                    10 + 90 * (a % 6),
                    65 + 50 * l + 315 * (a as f64 / 6 as f64) as usize
                ),
                _ => format!(
                    "border-radius:4px;top: {}px; left:{}px; background-color: White;",
                    10 + 90 * (a % 6),
                    65 + 50 * l + 315 * (a as f64 / 6 as f64) as usize
                ), // includes 0 as default case
            }
        } else {
            format!(
                "border-radius:4px;top: {}px; left:{}px; background-color: White;",
                10 + 90 * (a % 6),
                65 + 50 * l + 315 * (a as f64 / 6 as f64) as usize
            )
        }
    }

    fn keyboardletter_style(&self, letter: String) -> String {
        match self.letterstyles.get(&letter) {
            Some(1) => format!("background-color: Orange;"),
            Some(2) => format!("background-color: Green;"),
            Some(3) => format!("background-color: Grey;"),
            _ => format!("background-color: White;"), // includes 0 as default case
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: u32,
}

impl Component for Home {
    type Message = Msg;
    type Properties = GameProps;
    fn create(ctx: &Context<Self>) -> Self {
        let vec = Vec::new();
        let vec2 = Vec::new();
        let res1 = Vec::new();
        let res2 = Vec::new();
        let mut word = Vec::new();
        let word_id = ctx.props().id % WORDS_RAW.len() as u32;
        log::info!("{}", word_id);

        for j in 0..5 {
            word.push(
                WORDS_RAW[word_id as usize]
                    .chars()
                    .nth(j)
                    .unwrap()
                    .to_string(),
            );
        }

        //log::debug!("test");
        //&words.clone()[word_id];

        let mut occurrences = HashMap::new();
        for i in 0..5 {
            let count = occurrences.entry(word[i].clone()).or_insert(0);
            *count += 1;
        }

        let letterstyles = HashMap::from([
            ("A".to_string(), 0),
            ("B".to_string(), 0),
            ("C".to_string(), 0),
            ("D".to_string(), 0),
            ("E".to_string(), 0),
            ("F".to_string(), 0),
            ("G".to_string(), 0),
            ("H".to_string(), 0),
            ("I".to_string(), 0),
            ("J".to_string(), 0),
            ("K".to_string(), 0),
            ("L".to_string(), 0),
            ("M".to_string(), 0),
            ("N".to_string(), 0),
            ("O".to_string(), 0),
            ("P".to_string(), 0),
            ("Q".to_string(), 0),
            ("R".to_string(), 0),
            ("S".to_string(), 0),
            ("T".to_string(), 0),
            ("U".to_string(), 0),
            ("v".to_string(), 0),
            ("W".to_string(), 0),
            ("X".to_string(), 0),
            ("Y".to_string(), 0),
            ("Z".to_string(), 0),
        ]);

        let s = Self {
            attempt: vec,
            attempts: vec2,
            results_wordle: res1,
            results_master: res2,
            word: word, //.to_vec(),
            occurrences: occurrences,
            letterstyles: letterstyles,
        };

        /*
        let mut hk = hotkey::Listener::new();
        hk.register_hotkey('a' as u32,
            || s.link.callback(|_| Msg::A),
        )
        .unwrap();
        hk.listen();
        */
        return s;
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!(
            "word: {}{}{}{}{}",
            self.word[0],
            self.word[1],
            self.word[2],
            self.word[3],
            self.word[4]
        );
        match msg {
            Msg::Letter(l) => self.input(l),
            Msg::Style(a, l) => self.set_style(a, l),
            Msg::Enter => {
                if self.attempt.len() == 5 {
                    if self.check_word() {
                        self.results_wordle.push(vec![String::from("Gray"); 5]);
                        self.results_master.push(vec![0; 2]);
                        let mut checked = HashMap::new();

                        let idx = self.attempts.len();
                        // first check exact matches
                        for i in 0..5 {
                            if self.attempt[i] == self.word[i] {
                                let count = checked.entry(self.word[i].clone()).or_insert(0);
                                *count += 1;
                                self.results_wordle[idx][i] = String::from("MediumSeaGreen");
                                self.results_master[idx][0] += 1;
                            }
                        }
                        /*
                        for i in 0..5 { // for each letter in the attempt
                            for j in 0..5 { // for each letter in the answer
                                if self.attempt[i] == self.word[j] {
                                    if i == j {
                                        let count = checked.entry(self.word[j].clone()).or_insert(0);
                                        *count += 1;
                                        self.results_wordle[idx][i] = String::from("MediumSeaGreen");
                                        self.results_master[idx][0] += 1;
                                    }
                                }
                            }
                        }
                        */

                        // now check correct letters in wrong place
                        for i in 0..5 {
                            // for each letter in the attempt
                            for j in 0..5 {
                                // for each letter in the answer
                                // cond. 1 & 2: letter from attempt appears in solution at a different location
                                // cond. 3: letter (in solution) has been counted fewer times than it occurs (mostly to not count a letter that has already been counted above?)
                                if self.attempt[i] == self.word[j]
                                    && j != i
                                    && checked.get(&self.attempt[i])
                                        < self.occurrences.get(&self.word[j])
                                {
                                    let count = checked.entry(self.attempt[i].clone()).or_insert(0);
                                    *count += 1;
                                    self.results_wordle[idx][i] = String::from("Orange"); // need to handle repeated letters where only one is correct
                                    self.results_master[idx][1] += 1;
                                    break; // if a match is found, go to next letter in the attempt - each letter can only count for one!
                                }
                            }
                        }
                        //if self.results_master[idx][0] + self.results_master[idx][1] == 0 { // hints for user to block out letters that are certainly not in it
                        //}

                        self.attempts.push(self.attempt.clone());
                        self.attempt = Vec::new();
                    } else {
                        log::debug!("not a word");
                    }
                }
            }
            Msg::Backspace => {
                let l = self.attempt.len();
                if l > 0 {
                    self.attempt.remove(l - 1);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut letters = vec![vec!["".to_string(); 5]; 12];
        //let mut results = vec![vec![String::from("LightGrey"); 2]; 6];
        let mut results = vec![vec![0; 2]; 12];
        //log::debug!("n attempts: {}", self.attempts.len());
        for i in 0..self.attempts.len() {
            for j in 0..5 {
                letters[i][j] = self.attempts[i][j].clone();
                //results[i][j] = self.results[i][j].clone();
            }
            results[i][0] = self.results_master[i][0];
            results[i][1] = self.results_master[i][1];
        }
        let get_hints = |a: usize, l: usize| -> String {
            if self.attempts.len() <= a {
                String::from("")
            } else {
                format!("{}", results[a][l])
            }
        };
        for i in 0..self.attempt.len() {
            letters[self.attempts.len()][i] = self.attempt[i].clone();
        }
        //let s: String = "border-radius:4px;top: 10px; left:65px; background-color:LightGrey;".to_owned();
        //let slice: &str = &s[..];

        //log::debug!("col0: {}", results[0][0]);
        html! {
            <div>
                //<label class="field" style=slice>{letters[0][0].clone()}</label>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(0, 0))} style={self.guessletter_style(0, 0)}>{letters[0][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(0, 1))} style={self.guessletter_style(0, 1)}>{letters[0][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(0, 2))} style={self.guessletter_style(0, 2)}>{letters[0][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(0, 3))} style={self.guessletter_style(0, 3)}>{letters[0][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(0, 4))} style={self.guessletter_style(0, 4)}>{letters[0][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 10px; left:315px;">{get_hints(0, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 40px; left:315px;">{get_hints(0, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(1, 0))} style={self.guessletter_style(1, 0)}>{letters[1][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(1, 1))} style={self.guessletter_style(1, 1)}>{letters[1][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(1, 2))} style={self.guessletter_style(1, 2)}>{letters[1][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(1, 3))} style={self.guessletter_style(1, 3)}>{letters[1][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(1, 4))} style={self.guessletter_style(1, 4)}>{letters[1][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 100px; left:315px;">{get_hints(1, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 130px; left:315px;">{get_hints(1, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(2, 0))} style={self.guessletter_style(2, 0)}>{letters[2][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(2, 1))} style={self.guessletter_style(2, 1)}>{letters[2][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(2, 2))} style={self.guessletter_style(2, 2)}>{letters[2][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(2, 3))} style={self.guessletter_style(2, 3)}>{letters[2][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(2, 4))} style={self.guessletter_style(2, 4)}>{letters[2][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 190px; left:315px;">{get_hints(2, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 220px; left:315px;">{get_hints(2, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(3, 0))} style={self.guessletter_style(3, 0)}>{letters[3][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(3, 1))} style={self.guessletter_style(3, 1)}>{letters[3][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(3, 2))} style={self.guessletter_style(3, 2)}>{letters[3][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(3, 3))} style={self.guessletter_style(3, 3)}>{letters[3][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(3, 4))} style={self.guessletter_style(3, 4)}>{letters[3][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 280px; left:315px;">{get_hints(3, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 310px; left:315px;">{get_hints(3, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(4, 0))} style={self.guessletter_style(4, 0)}>{letters[4][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(4, 1))} style={self.guessletter_style(4, 1)}>{letters[4][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(4, 2))} style={self.guessletter_style(4, 2)}>{letters[4][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(4, 3))} style={self.guessletter_style(4, 3)}>{letters[4][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(4, 4))} style={self.guessletter_style(4, 4)}>{letters[4][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 370px; left:315px;">{get_hints(4, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 400px; left:315px;">{get_hints(4, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(5, 0))} style={self.guessletter_style(5, 0)}>{letters[5][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(5, 1))} style={self.guessletter_style(5, 1)}>{letters[5][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(5, 2))} style={self.guessletter_style(5, 2)}>{letters[5][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(5, 3))} style={self.guessletter_style(5, 3)}>{letters[5][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(5, 4))} style={self.guessletter_style(5, 4)}>{letters[5][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 460px; left:315px;">{get_hints(5, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 490px; left:315px;">{get_hints(5, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(6, 0))} style={self.guessletter_style(6, 0)}>{letters[6][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(6, 1))} style={self.guessletter_style(6, 1)}>{letters[6][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(6, 2))} style={self.guessletter_style(6, 2)}>{letters[6][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(6, 3))} style={self.guessletter_style(6, 3)}>{letters[6][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(6, 4))} style={self.guessletter_style(6, 4)}>{letters[6][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 10px; left:630px;">{get_hints(6, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 40px; left:630px;">{get_hints(6, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(7, 0))} style={self.guessletter_style(7, 0)}>{letters[7][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(7, 1))} style={self.guessletter_style(7, 1)}>{letters[7][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(7, 2))} style={self.guessletter_style(7, 2)}>{letters[7][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(7, 3))} style={self.guessletter_style(7, 3)}>{letters[7][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(7, 4))} style={self.guessletter_style(7, 4)}>{letters[7][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 100px; left:630px;">{get_hints(7, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 130px; left:630px;">{get_hints(7, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(8, 0))} style={self.guessletter_style(8, 0)}>{letters[8][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(8, 1))} style={self.guessletter_style(8, 1)}>{letters[8][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(8, 2))} style={self.guessletter_style(8, 2)}>{letters[8][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(8, 3))} style={self.guessletter_style(8, 3)}>{letters[8][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(8, 4))} style={self.guessletter_style(8, 4)}>{letters[8][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 190px; left:630px;">{get_hints(8, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 220px; left:630px;">{get_hints(8, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(9, 0))} style={self.guessletter_style(9, 0)}>{letters[9][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(9, 1))} style={self.guessletter_style(9, 1)}>{letters[9][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(9, 2))} style={self.guessletter_style(9, 2)}>{letters[9][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(9, 3))} style={self.guessletter_style(9, 3)}>{letters[9][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(9, 4))} style={self.guessletter_style(9, 4)}>{letters[9][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 280px; left:630px;">{get_hints(9, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 310px; left:630px;">{get_hints(9, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(10, 0))} style={self.guessletter_style(10, 0)}>{letters[10][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(10, 1))} style={self.guessletter_style(10, 1)}>{letters[10][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(10, 2))} style={self.guessletter_style(10, 2)}>{letters[10][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(10, 3))} style={self.guessletter_style(10, 3)}>{letters[10][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(10, 4))} style={self.guessletter_style(10, 4)}>{letters[10][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 370px; left:630px;">{get_hints(10, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 400px; left:630px;">{get_hints(10, 1)}</label>

                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(11, 0))} style={self.guessletter_style(11, 0)}>{letters[11][0].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(11, 1))} style={self.guessletter_style(11, 1)}>{letters[11][1].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(11, 2))} style={self.guessletter_style(11, 2)}>{letters[11][2].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(11, 3))} style={self.guessletter_style(11, 3)}>{letters[11][3].clone()}</button>
                <button class="field" onclick={ctx.link().callback(|_| Msg::Style(11, 4))} style={self.guessletter_style(11, 4)}>{letters[11][4].clone()}</button>
                <label class="corr1" style="border-radius:4px;top: 460px; left:630px;">{get_hints(11, 0)}</label>
                <label class="corr2" style="border-radius:4px;top: 490px; left:630px;">{get_hints(11, 1)}</label>

                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("Q".to_string()))} style={format!("border-radius:4px;top: 600px; left:30px; {}", self.keyboardletter_style("Q".to_string()))}>{"q"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("W".to_string()))} style={format!("border-radius:4px;top: 600px; left:60px; {}", self.keyboardletter_style("W".to_string()))}>{"w"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("E".to_string()))} style={format!("border-radius:4px;top: 600px; left:90px; {}", self.keyboardletter_style("E".to_string()))}>{"e"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("R".to_string()))} style={format!("border-radius:4px;top: 600px; left:120px; {}", self.keyboardletter_style("R".to_string()))}>{"r"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("T".to_string()))} style={format!("border-radius:4px;top: 600px; left:150px; {}", self.keyboardletter_style("T".to_string()))}>{"t"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("Y".to_string()))} style={format!("border-radius:4px;top: 600px; left:180px; {}", self.keyboardletter_style("Y".to_string()))}>{"y"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("U".to_string()))} style={format!("border-radius:4px;top: 600px; left:210px; {}", self.keyboardletter_style("U".to_string()))}>{"u"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("I".to_string()))} style={format!("border-radius:4px;top: 600px; left:240px; {}", self.keyboardletter_style("I".to_string()))}>{"i"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("O".to_string()))} style={format!("border-radius:4px;top: 600px; left:270px; {}", self.keyboardletter_style("O".to_string()))}>{"o"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("P".to_string()))} style={format!("border-radius:4px;top: 600px; left:300px; {}", self.keyboardletter_style("P".to_string()))}>{"p"}</button>

                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("A".to_string()))} style={format!("border-radius:4px;top: 640px; left:45px; {}", self.keyboardletter_style("A".to_string()))}>{"a"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("S".to_string()))} style={format!("border-radius:4px;top: 640px; left:75px; {}", self.keyboardletter_style("S".to_string()))}>{"s"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("D".to_string()))} style={format!("border-radius:4px;top: 640px; left:105px; {}", self.keyboardletter_style("D".to_string()))}>{"d"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("F".to_string()))} style={format!("border-radius:4px;top: 640px; left:135px; {}", self.keyboardletter_style("F".to_string()))}>{"f"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("G".to_string()))} style={format!("border-radius:4px;top: 640px; left:165px; {}", self.keyboardletter_style("G".to_string()))}>{"g"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("H".to_string()))} style={format!("border-radius:4px;top: 640px; left:195px; {}", self.keyboardletter_style("H".to_string()))}>{"h"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("J".to_string()))} style={format!("border-radius:4px;top: 640px; left:225px; {}", self.keyboardletter_style("J".to_string()))}>{"j"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("K".to_string()))} style={format!("border-radius:4px;top: 640px; left:255px; {}", self.keyboardletter_style("K".to_string()))}>{"k"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("L".to_string()))} style={format!("border-radius:4px;top: 640px; left:285px; {}", self.keyboardletter_style("L".to_string()))}>{"l"}</button>

                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("Z".to_string()))} style={format!("border-radius:4px;top: 680px; left:60px; {}", self.keyboardletter_style("Z".to_string()))}>{"z"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("X".to_string()))} style={format!("border-radius:4px;top: 680px; left:90px; {}", self.keyboardletter_style("X".to_string()))}>{"x"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("C".to_string()))} style={format!("border-radius:4px;top: 680px; left:120px; {}", self.keyboardletter_style("C".to_string()))}>{"c"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("V".to_string()))} style={format!("border-radius:4px;top: 680px; left:150px; {}", self.keyboardletter_style("V".to_string()))}>{"v"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("B".to_string()))} style={format!("border-radius:4px;top: 680px; left:180px; {}", self.keyboardletter_style("B".to_string()))}>{"b"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("N".to_string()))} style={format!("border-radius:4px;top: 680px; left:210px; {}", self.keyboardletter_style("N".to_string()))}>{"n"}</button>
                <button class="btn" onclick={ctx.link().callback(|_| Msg::Letter("M".to_string()))} style={format!("border-radius:4px;top: 680px; left:240px; {}", self.keyboardletter_style("M".to_string()))}>{"m"}</button>

                <button class="btn2" onclick={ctx.link().callback(|_| Msg::Enter)} style="border-radius:4px;top: 680px; left:18px;width=60px">{"Enter"}</button>
                <button class="btn2" onclick={ctx.link().callback(|_| Msg::Backspace)} style="border-radius:4px;top: 680px; left:270px;width=60px">{"<-"}</button>
                <div class="instructions" style="top:30px;left:700px;width=500px;white-space=normal;">{"How Masterword-clues work: The two boxes next to each attempt tell you how many letters occur in exactly the guessed location (top) and how many letters occur in a different location (bottom).\nTo help the player keep track, each letter can be coloured in green, orange, grey, or white (default). These colours are cycled when clicking on any occurrence of the letter in a guess. Be aware that any letter can globally only have one single colour."}</div>

            </div>
        }
    }
}

const WORDS_RAW: &[&'static str] = &[
    "AAHED", "AALII", "AARGH", "AARTI", "ABACA", "ABACI", "ABACK", "ABACS", "ABAFT", "ABAKA",
    "ABAMP", "ABAND", "ABASE", "ABASH", "ABASK", "ABATE", "ABAYA", "ABBAS", "ABBED", "ABBES",
    "ABBEY", "ABBOT", "ABCEE", "ABEAM", "ABEAR", "ABELE", "ABETS", "ABHOR", "ABIDE", "ABIES",
    "ABLED", "ABLER", "ABLES", "ABLET", "ABLOW", "ABMHO", "ABODE", "ABOHM", "ABOIL", "ABOMA",
    "ABOON", "ABORD", "ABORE", "ABORT", "ABOUT", "ABOVE", "ABRAM", "ABRAY", "ABRIM", "ABRIN",
    "ABRIS", "ABSEY", "ABSIT", "ABUNA", "ABUNE", "ABUSE", "ABUTS", "ABUZZ", "ABYES", "ABYSM",
    "ABYSS", "ACAIS", "ACARI", "ACCAS", "ACCOY", "ACERB", "ACERS", "ACETA", "ACHED", "ACHES",
    "ACHOO", "ACIDS", "ACIDY", "ACING", "ACINI", "ACKEE", "ACKER", "ACMES", "ACMIC", "ACNED",
    "ACNES", "ACOCK", "ACOLD", "ACORN", "ACRED", "ACRES", "ACRID", "ACTED", "ACTIN", "ACTON",
    "ACTOR", "ACUTE", "ACYLS", "ADAGE", "ADAPT", "ADAWS", "ADAYS", "ADDAX", "ADDED", "ADDER",
    "ADDIO", "ADDLE", "ADEEM", "ADEPT", "ADHAN", "ADIEU", "ADIOS", "ADITS", "ADMAN", "ADMEN",
    "ADMIN", "ADMIT", "ADMIX", "ADOBE", "ADOBO", "ADOPT", "ADORE", "ADORN", "ADOWN", "ADOZE",
    "ADRAD", "ADRED", "ADSUM", "ADUKI", "ADULT", "ADUNC", "ADUST", "ADVEW", "ADYTA", "ADZED",
    "ADZES", "AECIA", "AEDES", "AEGIS", "AEONS", "AERIE", "AEROS", "AESIR", "AFALD", "AFARA",
    "AFARS", "AFEAR", "AFFIX", "AFIRE", "AFLAJ", "AFOOT", "AFORE", "AFOUL", "AFRIT", "AFROS",
    "AFTER", "AGAIN", "AGAMA", "AGAMI", "AGAPE", "AGARS", "AGAST", "AGATE", "AGAVE", "AGAZE",
    "AGENE", "AGENT", "AGERS", "AGGER", "AGGIE", "AGGRI", "AGGRO", "AGGRY", "AGHAS", "AGILA",
    "AGILE", "AGING", "AGIOS", "AGISM", "AGIST", "AGITA", "AGLEE", "AGLET", "AGLEY", "AGLOO",
    "AGLOW", "AGLUS", "AGMAS", "AGOGE", "AGONE", "AGONS", "AGONY", "AGOOD", "AGORA", "AGREE",
    "AGRIA", "AGRIN", "AGUED", "AGUES", "AGUTI", "AHEAD", "AHEAP", "AHENT", "AHIGH", "AHIND",
    "AHING", "AHINT", "AHOLD", "AHULL", "AHURU", "AIDED", "AIDER", "AIDES", "AIDOI", "AIDOS",
    "AIERY", "AIGAS", "AILED", "AIMED", "AIMER", "AINEE", "AINGA", "AIOLI", "AIRED", "AIRER",
    "AIRNS", "AIRTH", "AIRTS", "AISLE", "AITCH", "AITUS", "AIVER", "AIZLE", "AJIVA", "AJUGA",
    "AJWAN", "AKEES", "AKELA", "AKENE", "AKING", "AKITA", "AKKAS", "ALAAP", "ALACK", "ALAMO",
    "ALAND", "ALANE", "ALANG", "ALANS", "ALANT", "ALAPA", "ALAPS", "ALARM", "ALARY", "ALATE",
    "ALAYS", "ALBAS", "ALBEE", "ALBUM", "ALCID", "ALCOS", "ALDEA", "ALDER", "ALDOL", "ALECK",
    "ALECS", "ALEFS", "ALEFT", "ALEPH", "ALERT", "ALEWS", "ALEYE", "ALFAS", "ALGAE", "ALGAL",
    "ALGAS", "ALGID", "ALGIN", "ALGOR", "ALGUM", "ALIAS", "ALIBI", "ALIEN", "ALIFS", "ALIGN",
    "ALIKE", "ALINE", "ALIST", "ALIVE", "ALIYA", "ALKIE", "ALKOS", "ALKYD", "ALKYL", "ALLAY",
    "ALLEE", "ALLEL", "ALLEY", "ALLIS", "ALLOD", "ALLOT", "ALLOW", "ALLOY", "ALLYL", "ALMAH",
    "ALMAS", "ALMEH", "ALMES", "ALMUD", "ALMUG", "ALODS", "ALOED", "ALOES", "ALOFT", "ALOHA",
    "ALOIN", "ALONE", "ALONG", "ALOOF", "ALOUD", "ALOWE", "ALPHA", "ALTAR", "ALTER", "ALTHO",
    "ALTOS", "ALULA", "ALUMS", "ALURE", "ALWAY", "AMAHS", "AMAIN", "AMASS", "AMATE", "AMAUT",
    "AMAZE", "AMBAN", "AMBER", "AMBIT", "AMBLE", "AMBOS", "AMBRY", "AMEBA", "AMEER", "AMEND",
    "AMENE", "AMENS", "AMENT", "AMIAS", "AMICE", "AMICI", "AMIDE", "AMIDO", "AMIDS", "AMIES",
    "AMIGA", "AMIGO", "AMINE", "AMINO", "AMINS", "AMIRS", "AMISS", "AMITY", "AMLAS", "AMMAN",
    "AMMON", "AMMOS", "AMNIA", "AMNIC", "AMNIO", "AMOKS", "AMOLE", "AMONG", "AMORT", "AMOUR",
    "AMOVE", "AMOWT", "AMPED", "AMPLE", "AMPLY", "AMPUL", "AMRIT", "AMUCK", "AMUSE", "AMYLS",
    "ANANA", "ANATA", "ANCHO", "ANCLE", "ANCON", "ANDRO", "ANEAR", "ANELE", "ANENT", "ANGAS",
    "ANGEL", "ANGER", "ANGLE", "ANGLO", "ANGRY", "ANGST", "ANIGH", "ANILE", "ANILS", "ANIMA",
    "ANIME", "ANIMI", "ANION", "ANISE", "ANKER", "ANKHS", "ANKLE", "ANKUS", "ANLAS", "ANNAL",
    "ANNAS", "ANNAT", "ANNEX", "ANNOY", "ANNUL", "ANOAS", "ANODE", "ANOLE", "ANOMY", "ANSAE",
    "ANTAE", "ANTAR", "ANTAS", "ANTED", "ANTES", "ANTIC", "ANTIS", "ANTRA", "ANTRE", "ANTSY",
    "ANVIL", "ANYON", "AORTA", "APACE", "APAGE", "APAID", "APART", "APAYD", "APAYS", "APEAK",
    "APEEK", "APERS", "APERT", "APERY", "APGAR", "APHID", "APHIS", "APIAN", "APING", "APIOL",
    "APISH", "APISM", "APNEA", "APODE", "APODS", "APOOP", "APORT", "APPAL", "APPAY", "APPEL",
    "APPLE", "APPLY", "APPRO", "APPUI", "APPUY", "APRES", "APRON", "APSES", "APSIS", "APSOS",
    "APTED", "APTER", "APTLY", "AQUAE", "AQUAS", "ARABA", "ARAKS", "ARAME", "ARARS", "ARBAS",
    "ARBOR", "ARCED", "ARCUS", "ARDEB", "ARDOR", "ARDRI", "AREAD", "AREAE", "AREAL", "AREAR",
    "AREAS", "ARECA", "AREDD", "AREDE", "AREFY", "AREIC", "ARENA", "ARENE", "AREPA", "ARERE",
    "ARETE", "ARETS", "ARETT", "ARGAL", "ARGAN", "ARGIL", "ARGLE", "ARGOL", "ARGON", "ARGOT",
    "ARGUE", "ARGUS", "ARHAT", "ARIAS", "ARIEL", "ARIKI", "ARILS", "ARIOT", "ARISE", "ARISH",
    "ARKED", "ARLED", "ARLES", "ARMED", "ARMER", "ARMET", "ARMIL", "ARMOR", "ARNAS", "ARNUT",
    "AROBA", "AROHA", "AROID", "AROMA", "AROSE", "ARPAS", "ARPEN", "ARRAH", "ARRAS", "ARRAY",
    "ARRET", "ARRIS", "ARROW", "ARSED", "ARSES", "ARSEY", "ARSIS", "ARSON", "ARTAL", "ARTEL",
    "ARTIC", "ARTIS", "ARTSY", "ARUHE", "ARUMS", "ARVAL", "ARVOS", "ARYLS", "ASANA", "ASCOT",
    "ASCUS", "ASDIC", "ASHED", "ASHEN", "ASHES", "ASHET", "ASIDE", "ASKED", "ASKER", "ASKEW",
    "ASKOI", "ASKOS", "ASPEN", "ASPER", "ASPIC", "ASPIS", "ASPRO", "ASSAI", "ASSAM", "ASSAY",
    "ASSES", "ASSET", "ASSEZ", "ASSOT", "ASTER", "ASTIR", "ASTUN", "ASWAY", "ASWIM", "ASYLA",
    "ATAPS", "ATAXY", "ATIGI", "ATILT", "ATIMY", "ATLAS", "ATMAN", "ATMAS", "ATOCS", "ATOKE",
    "ATOKS", "ATOLL", "ATOMS", "ATOMY", "ATONE", "ATONY", "ATOPY", "ATRIA", "ATRIP", "ATTAP",
    "ATTAR", "ATTIC", "ATUAS", "AUDAD", "AUDIO", "AUDIT", "AUGER", "AUGHT", "AUGUR", "AULAS",
    "AULIC", "AULOI", "AULOS", "AUMIL", "AUNES", "AUNTS", "AUNTY", "AURAE", "AURAL", "AURAR",
    "AURAS", "AUREI", "AURES", "AURIC", "AURIS", "AURUM", "AUTOS", "AUXIN", "AVAIL", "AVALE",
    "AVANT", "AVAST", "AVELS", "AVENS", "AVERS", "AVERT", "AVGAS", "AVIAN", "AVINE", "AVION",
    "AVISE", "AVISO", "AVIZE", "AVOID", "AVOWS", "AVYZE", "AWAIT", "AWAKE", "AWARD", "AWARE",
    "AWARN", "AWASH", "AWATO", "AWAVE", "AWAYS", "AWDLS", "AWEEL", "AWETO", "AWFUL", "AWING",
    "AWMRY", "AWNED", "AWNER", "AWOKE", "AWOLS", "AWORK", "AXELS", "AXIAL", "AXILE", "AXILS",
    "AXING", "AXIOM", "AXION", "AXITE", "AXLED", "AXLES", "AXMAN", "AXMEN", "AXOID", "AXONE",
    "AXONS", "AYAHS", "AYELP", "AYGRE", "AYINS", "AYONT", "AYRES", "AYRIE", "AZANS", "AZIDE",
    "AZIDO", "AZINE", "AZLON", "AZOIC", "AZOLE", "AZONS", "AZOTE", "AZOTH", "AZUKI", "AZURE",
    "AZURN", "AZURY", "AZYGY", "AZYME", "AZYMS", "BAAED", "BAALS", "BABAS", "BABEL", "BABES",
    "BABKA", "BABOO", "BABUL", "BABUS", "BACCA", "BACCO", "BACCY", "BACHA", "BACHS", "BACKS",
    "BACON", "BADDY", "BADGE", "BADLY", "BAELS", "BAFFS", "BAFFY", "BAFTS", "BAGEL", "BAGGY",
    "BAGHS", "BAGIE", "BAHTS", "BAHUT", "BAILS", "BAIRN", "BAITH", "BAITS", "BAIZA", "BAIZE",
    "BAJAN", "BAJRA", "BAJRI", "BAJUS", "BAKED", "BAKEN", "BAKER", "BAKES", "BAKRA", "BALAS",
    "BALDS", "BALDY", "BALED", "BALER", "BALES", "BALKS", "BALKY", "BALLS", "BALLY", "BALMS",
    "BALMY", "BALOO", "BALSA", "BALTI", "BALUN", "BALUS", "BAMBI", "BANAK", "BANAL", "BANCO",
    "BANCS", "BANDA", "BANDH", "BANDS", "BANDY", "BANED", "BANES", "BANGS", "BANIA", "BANJO",
    "BANKS", "BANNS", "BANTS", "BANTU", "BANTY", "BAPUS", "BARBE", "BARBS", "BARBY", "BARCA",
    "BARDE", "BARDO", "BARDS", "BARDY", "BARED", "BARER", "BARES", "BARFS", "BARGE", "BARIC",
    "BARKS", "BARKY", "BARMS", "BARMY", "BARNS", "BARNY", "BARON", "BARPS", "BARRA", "BARRE",
    "BARRO", "BARRY", "BARYE", "BASAL", "BASAN", "BASED", "BASER", "BASES", "BASHO", "BASIC",
    "BASIL", "BASIN", "BASIS", "BASKS", "BASON", "BASSE", "BASSI", "BASSO", "BASSY", "BASTA",
    "BASTE", "BASTI", "BASTO", "BASTS", "BATCH", "BATED", "BATES", "BATHE", "BATHS", "BATIK",
    "BATON", "BATTA", "BATTS", "BATTU", "BATTY", "BAUDS", "BAUKS", "BAULK", "BAURS", "BAVIN",
    "BAWDS", "BAWDY", "BAWLS", "BAWNS", "BAWRS", "BAWTY", "BAYED", "BAYES", "BAYLE", "BAYOU",
    "BAYTS", "BAZAR", "BAZOO", "BEACH", "BEADS", "BEADY", "BEAKS", "BEAKY", "BEAMS", "BEAMY",
    "BEANO", "BEANS", "BEANY", "BEARD", "BEARE", "BEARS", "BEAST", "BEATH", "BEATS", "BEATY",
    "BEAUS", "BEAUT", "BEAUX", "BEBOP", "BECAP", "BECKE", "BECKS", "BEDAD", "BEDEL", "BEDES",
    "BEDEW", "BEDIM", "BEDYE", "BEECH", "BEEDI", "BEEFS", "BEEFY", "BEEPS", "BEERS", "BEERY",
    "BEETS", "BEFIT", "BEFOG", "BEGAD", "BEGAN", "BEGAR", "BEGAT", "BEGEM", "BEGET", "BEGIN",
    "BEGOT", "BEGUM", "BEGUN", "BEIGE", "BEIGY", "BEING", "BEKAH", "BELAH", "BELAR", "BELAY",
    "BELCH", "BELEE", "BELGA", "BELIE", "BELLE", "BELLS", "BELLY", "BELON", "BELOW", "BELTS",
    "BEMAD", "BEMAS", "BEMIX", "BEMUD", "BENCH", "BENDS", "BENDY", "BENES", "BENET", "BENIS",
    "BENNE", "BENNI", "BENNY", "BENTO", "BENTS", "BENTY", "BEPAT", "BERAY", "BERES", "BERET",
    "BERGS", "BERKO", "BERKS", "BERME", "BERMS", "BEROB", "BERRY", "BERTH", "BERYL", "BESAT",
    "BESAW", "BESEE", "BESES", "BESET", "BESIT", "BESOM", "BESOT", "BESTI", "BESTS", "BETAS",
    "BETED", "BETEL", "BETES", "BETHS", "BETID", "BETON", "BETTA", "BETTY", "BEVEL", "BEVER",
    "BEVOR", "BEVUE", "BEVVY", "BEWET", "BEWIG", "BEZEL", "BEZES", "BEZIL", "BHAJI", "BHANG",
    "BHELS", "BHOOT", "BHUNA", "BHUTS", "BIALI", "BIALY", "BIBBS", "BIBLE", "BICCY", "BICEP",
    "BICES", "BIDDY", "BIDED", "BIDER", "BIDES", "BIDET", "BIDIS", "BIDON", "BIELD", "BIERS",
    "BIFFO", "BIFFS", "BIFFY", "BIFID", "BIGAE", "BIGGS", "BIGGY", "BIGHA", "BIGHT", "BIGLY",
    "BIGOS", "BIGOT", "BIJOU", "BIKED", "BIKER", "BIKES", "BIKIE", "BILBO", "BILBY", "BILED",
    "BILES", "BILGE", "BILGY", "BILKS", "BILLS", "BILLY", "BIMAH", "BIMAS", "BIMBO", "BINAL",
    "BINDI", "BINDS", "BINER", "BINES", "BINGE", "BINGO", "BINGS", "BINGY", "BINIT", "BINKS",
    "BINTS", "BIOGS", "BIOME", "BIONT", "BIOTA", "BIPED", "BIPOD", "BIRCH", "BIRDS", "BIRKS",
    "BIRLE", "BIRLS", "BIROS", "BIRRS", "BIRSE", "BIRSY", "BIRTH", "BISES", "BISKS", "BISON",
    "BITCH", "BITER", "BITES", "BITOS", "BITOU", "BITSY", "BITTE", "BITTS", "BITTY", "BIVIA",
    "BIVVY", "BIZES", "BIZZO", "BIZZY", "BLABS", "BLACK", "BLADE", "BLADS", "BLADY", "BLAER",
    "BLAES", "BLAFF", "BLAGS", "BLAHS", "BLAIN", "BLAME", "BLAMS", "BLAND", "BLANK", "BLARE",
    "BLART", "BLASE", "BLASH", "BLAST", "BLATE", "BLATS", "BLATT", "BLAUD", "BLAWN", "BLAWS",
    "BLAYS", "BLAZE", "BLEAK", "BLEAR", "BLEAT", "BLEBS", "BLEED", "BLEEP", "BLEES", "BLEND",
    "BLENT", "BLERT", "BLESS", "BLEST", "BLETS", "BLEYS", "BLIMP", "BLIMY", "BLIND", "BLING",
    "BLINI", "BLINK", "BLINS", "BLINY", "BLIPS", "BLISS", "BLIST", "BLITE", "BLITZ", "BLIVE",
    "BLOAT", "BLOBS", "BLOCK", "BLOCS", "BLOGS", "BLOKE", "BLOND", "BLOOD", "BLOOM", "BLOOP",
    "BLORE", "BLOTS", "BLOWN", "BLOWS", "BLOWY", "BLUBS", "BLUDE", "BLUDY", "BLUED", "BLUER",
    "BLUES", "BLUET", "BLUEY", "BLUFF", "BLUID", "BLUME", "BLUNK", "BLUNT", "BLURB", "BLURS",
    "BLURT", "BLUSH", "BLYPE", "BOABS", "BOAKS", "BOARD", "BOARS", "BOART", "BOAST", "BOATS",
    "BOBAC", "BOBAK", "BOBAS", "BOBBY", "BOBOL", "BOCCA", "BOCCE", "BOCCI", "BOCHE", "BOCKS",
    "BODED", "BODES", "BODGE", "BODLE", "BOEPS", "BOETS", "BOEUF", "BOFFO", "BOFFS", "BOGAN",
    "BOGEY", "BOGGY", "BOGIE", "BOGLE", "BOGUS", "BOHEA", "BOHOS", "BOILS", "BOING", "BOINK",
    "BOITE", "BOKED", "BOKES", "BOKOS", "BOLAR", "BOLAS", "BOLDS", "BOLES", "BOLIX", "BOLLS",
    "BOLOS", "BOLTS", "BOLUS", "BOMAS", "BOMBE", "BOMBO", "BOMBS", "BONCE", "BONDS", "BONED",
    "BONER", "BONES", "BONEY", "BONGO", "BONGS", "BONIE", "BONKS", "BONNE", "BONNY", "BONUS",
    "BONZA", "BONZE", "BOOBS", "BOOBY", "BOODY", "BOOED", "BOOFY", "BOOGY", "BOOHS", "BOOKS",
    "BOOKY", "BOOLS", "BOOMS", "BOOMY", "BOONG", "BOONS", "BOORD", "BOORS", "BOOSE", "BOOST",
    "BOOTH", "BOOTS", "BOOTY", "BOOZE", "BOOZY", "BORAK", "BORAL", "BORAS", "BORAX", "BORDE",
    "BORDS", "BORED", "BOREE", "BOREL", "BORER", "BORES", "BORGO", "BORIC", "BORKS", "BORMS",
    "BORNA", "BORNE", "BORON", "BORTS", "BORTY", "BORTZ", "BOSKS", "BOSKY", "BOSOM", "BOSON",
    "BOSSY", "BOSUN", "BOTAS", "BOTCH", "BOTEL", "BOTHY", "BOTTE", "BOTTS", "BOTTY", "BOUGE",
    "BOUGH", "BOUKS", "BOULE", "BOULT", "BOUND", "BOUNS", "BOURD", "BOURG", "BOURN", "BOUSE",
    "BOUSY", "BOUTS", "BOVID", "BOWAT", "BOWED", "BOWEL", "BOWER", "BOWES", "BOWET", "BOWIE",
    "BOWLS", "BOWNE", "BOWRS", "BOWSE", "BOXED", "BOXEN", "BOXER", "BOXES", "BOYAR", "BOYAU",
    "BOYED", "BOYFS", "BOYGS", "BOYLA", "BOYOS", "BOYSY", "BOZOS", "BRAAI", "BRACE", "BRACH",
    "BRACK", "BRACT", "BRADS", "BRAES", "BRAGS", "BRAID", "BRAIL", "BRAIN", "BRAKE", "BRAKS",
    "BRAKY", "BRAME", "BRAND", "BRANK", "BRANS", "BRANT", "BRASH", "BRASS", "BRAST", "BRATS",
    "BRAVA", "BRAVE", "BRAVI", "BRAVO", "BRAWL", "BRAWN", "BRAWS", "BRAXY", "BRAYS", "BRAZA",
    "BRAZE", "BREAD", "BREAK", "BREAM", "BREDE", "BREDS", "BREED", "BREEM", "BREER", "BREES",
    "BREID", "BREIS", "BREME", "BRENS", "BRENT", "BRERE", "BRERS", "BREVE", "BREWS", "BREYS",
    "BRIAR", "BRIBE", "BRICK", "BRIDE", "BRIEF", "BRIER", "BRIES", "BRIGS", "BRIKS", "BRILL",
    "BRIMS", "BRINE", "BRING", "BRINK", "BRINS", "BRINY", "BRIOS", "BRISE", "BRISK", "BRISS",
    "BRITH", "BRITS", "BRITT", "BRIZE", "BROAD", "BROCH", "BROCK", "BRODS", "BROGH", "BROGS",
    "BROIL", "BROKE", "BROME", "BROMO", "BRONC", "BROND", "BROOD", "BROOK", "BROOL", "BROOM",
    "BROOS", "BROSE", "BROSY", "BROTH", "BROWN", "BROWS", "BRUGH", "BRUIN", "BRUIT", "BRULE",
    "BRUME", "BRUNG", "BRUNT", "BRUSH", "BRUSK", "BRUST", "BRUTE", "BRUTS", "BUATS", "BUAZE",
    "BUBAL", "BUBAS", "BUBBA", "BUBBY", "BUBUS", "BUCHU", "BUCKO", "BUCKS", "BUCKU", "BUDAS",
    "BUDDY", "BUDGE", "BUDIS", "BUDOS", "BUFFA", "BUFFE", "BUFFI", "BUFFO", "BUFFS", "BUFFY",
    "BUFOS", "BUGGY", "BUGLE", "BUHLS", "BUHRS", "BUIKS", "BUILD", "BUILT", "BUIST", "BUKES",
    "BULBS", "BULGE", "BULGY", "BULKS", "BULKY", "BULLA", "BULLS", "BULLY", "BULSE", "BUMBO",
    "BUMFS", "BUMPH", "BUMPS", "BUMPY", "BUNAS", "BUNCE", "BUNCH", "BUNCO", "BUNDE", "BUNDH",
    "BUNDS", "BUNDT", "BUNDU", "BUNDY", "BUNGS", "BUNGY", "BUNIA", "BUNJE", "BUNJY", "BUNKO",
    "BUNKS", "BUNNS", "BUNNY", "BUNTS", "BUNTY", "BUNYA", "BUOYS", "BUPPY", "BURAN", "BURAS",
    "BURBS", "BURDS", "BURET", "BURGH", "BURGS", "BURIN", "BURKA", "BURKE", "BURKS", "BURLS",
    "BURLY", "BURNS", "BURNT", "BUROO", "BURPS", "BURQA", "BURRO", "BURRS", "BURRY", "BURSA",
    "BURSE", "BURST", "BUSBY", "BUSED", "BUSES", "BUSHY", "BUSKS", "BUSKY", "BUSSU", "BUSTI",
    "BUSTS", "BUSTY", "BUTCH", "BUTEO", "BUTES", "BUTLE", "BUTTE", "BUTTS", "BUTTY", "BUTUT",
    "BUTYL", "BUXOM", "BUYER", "BUZZY", "BWANA", "BWAZI", "BYDED", "BYDES", "BYKED", "BYKES",
    "BYLAW", "BYRES", "BYRLS", "BYSSI", "BYTES", "BYWAY", "CAAED", "CABAL", "CABAS", "CABBY",
    "CABER", "CABIN", "CABLE", "CABOB", "CABOC", "CABRE", "CACAO", "CACAS", "CACHE", "CACKY",
    "CACTI", "CADDY", "CADEE", "CADES", "CADET", "CADGE", "CADGY", "CADIE", "CADIS", "CADRE",
    "CAECA", "CAESE", "CAFES", "CAFFS", "CAGED", "CAGER", "CAGES", "CAGEY", "CAGOT", "CAHOW",
    "CAIDS", "CAINS", "CAIRD", "CAIRN", "CAJON", "CAJUN", "CAKED", "CAKES", "CAKEY", "CALFS",
    "CALID", "CALIF", "CALIX", "CALKS", "CALLA", "CALLS", "CALMS", "CALMY", "CALOS", "CALPA",
    "CALPS", "CALVE", "CALYX", "CAMAN", "CAMAS", "CAMEL", "CAMEO", "CAMES", "CAMIS", "CAMOS",
    "CAMPI", "CAMPO", "CAMPS", "CAMPY", "CAMUS", "CANAL", "CANDY", "CANED", "CANEH", "CANER",
    "CANES", "CANGS", "CANID", "CANNA", "CANNS", "CANNY", "CANOE", "CANON", "CANSO", "CANST",
    "CANTO", "CANTS", "CANTY", "CAPAS", "CAPED", "CAPER", "CAPES", "CAPHS", "CAPIZ", "CAPLE",
    "CAPON", "CAPOS", "CAPOT", "CAPUL", "CAPUT", "CARAP", "CARAT", "CARBO", "CARBS", "CARBY",
    "CARDI", "CARDS", "CARDY", "CARED", "CARER", "CARES", "CARET", "CAREX", "CARGO", "CARKS",
    "CARLE", "CARLS", "CARNS", "CARNY", "CAROB", "CAROL", "CAROM", "CARPI", "CARPS", "CARRS",
    "CARRY", "CARSE", "CARTA", "CARTE", "CARTS", "CARVE", "CARVY", "CASAS", "CASCO", "CASED",
    "CASES", "CASKS", "CASKY", "CASTE", "CASTS", "CASUS", "CATCH", "CATER", "CATES", "CATTY",
    "CAUDA", "CAUKS", "CAULD", "CAULK", "CAULS", "CAUMS", "CAUPS", "CAUSA", "CAUSE", "CAVAS",
    "CAVED", "CAVEL", "CAVER", "CAVES", "CAVIE", "CAVIL", "CAWED", "CAWKS", "CAXON", "CEASE",
    "CEAZE", "CEBID", "CECAL", "CECUM", "CEDAR", "CEDED", "CEDER", "CEDES", "CEDIS", "CEIBA",
    "CEILI", "CEILS", "CELEB", "CELLA", "CELLI", "CELLO", "CELLS", "CELOM", "CELTS", "CENSE",
    "CENTO", "CENTS", "CENTU", "CEORL", "CEPES", "CERCI", "CERED", "CERES", "CERGE", "CERIA",
    "CERIC", "CERNE", "CEROS", "CERTS", "CESSE", "CESTA", "CESTI", "CETES", "CETYL", "CHACE",
    "CHACK", "CHACO", "CHADO", "CHADS", "CHAFE", "CHAFF", "CHAFT", "CHAIN", "CHAIR", "CHAIS",
    "CHALK", "CHALS", "CHAMP", "CHAMS", "CHANG", "CHANK", "CHANT", "CHAOS", "CHAPE", "CHAPS",
    "CHAPT", "CHARA", "CHARD", "CHARE", "CHARK", "CHARM", "CHARR", "CHARS", "CHART", "CHARY",
    "CHASE", "CHASM", "CHATS", "CHAVE", "CHAVS", "CHAWK", "CHAWS", "CHAYA", "CHAYS", "CHEAP",
    "CHEAT", "CHECK", "CHEEK", "CHEEP", "CHEER", "CHEFS", "CHEKA", "CHELA", "CHELP", "CHEMO",
    "CHERE", "CHERT", "CHESS", "CHEST", "CHETH", "CHEVY", "CHEWS", "CHEWY", "CHIAO", "CHIAS",
    "CHIBS", "CHICA", "CHICH", "CHICK", "CHICO", "CHICS", "CHIDE", "CHIEF", "CHIEL", "CHIKS",
    "CHILD", "CHILE", "CHILI", "CHILL", "CHIMB", "CHIME", "CHIMO", "CHIMP", "CHINA", "CHINE",
    "CHINK", "CHINO", "CHINS", "CHIPS", "CHIRK", "CHIRL", "CHIRM", "CHIRO", "CHIRP", "CHIRR",
    "CHIRT", "CHIRU", "CHITS", "CHIVE", "CHIVS", "CHIVY", "CHIZZ", "CHOCK", "CHOCO", "CHOCS",
    "CHODE", "CHOGS", "CHOIR", "CHOKE", "CHOKO", "CHOKY", "CHOLA", "CHOLI", "CHOLO", "CHOMP",
    "CHONS", "CHOOF", "CHOOK", "CHOOM", "CHOPS", "CHORD", "CHORE", "CHOSE", "CHOTA", "CHOTT",
    "CHOUT", "CHOUX", "CHOWK", "CHOWS", "CHUBS", "CHUCK", "CHUFA", "CHUFF", "CHUGS", "CHUMP",
    "CHUMS", "CHUNK", "CHURL", "CHURN", "CHURR", "CHUSE", "CHUTE", "CHYLE", "CHYME", "CHYND",
    "CIAOS", "CIBOL", "CIDED", "CIDER", "CIDES", "CIELS", "CIGAR", "CIGGY", "CILIA", "CILLS",
    "CIMAR", "CIMEX", "CINCH", "CINCT", "CINES", "CIONS", "CIPPI", "CIRCA", "CIRCS", "CIRES",
    "CIRLS", "CIRRI", "CISCO", "CISSY", "CISTS", "CITAL", "CITED", "CITER", "CITES", "CIVES",
    "CIVET", "CIVIC", "CIVIE", "CIVIL", "CIVVY", "CLACH", "CLACK", "CLADE", "CLADS", "CLAES",
    "CLAGS", "CLAIM", "CLAME", "CLAMP", "CLAMS", "CLANG", "CLANK", "CLANS", "CLAPS", "CLAPT",
    "CLARO", "CLART", "CLARY", "CLASH", "CLASP", "CLASS", "CLAST", "CLATS", "CLAUT", "CLAVE",
    "CLAVI", "CLAWS", "CLAYS", "CLEAN", "CLEAR", "CLEAT", "CLECK", "CLEEK", "CLEEP", "CLEFS",
    "CLEFT", "CLEGS", "CLEIK", "CLEMS", "CLEPE", "CLEPT", "CLERK", "CLEVE", "CLEWS", "CLICK",
    "CLIED", "CLIES", "CLIFF", "CLIFT", "CLIMB", "CLIME", "CLINE", "CLING", "CLINK", "CLINT",
    "CLIPE", "CLIPS", "CLIPT", "CLOAK", "CLOAM", "CLOCK", "CLODS", "CLOFF", "CLOGS", "CLOKE",
    "CLOMB", "CLOMP", "CLONE", "CLONK", "CLONS", "CLOOP", "CLOOT", "CLOPS", "CLOSE", "CLOTE",
    "CLOTH", "CLOTS", "CLOUD", "CLOUR", "CLOUS", "CLOUT", "CLOVE", "CLOWN", "CLOWS", "CLOYE",
    "CLOYS", "CLOZE", "CLUBS", "CLUCK", "CLUED", "CLUES", "CLUMP", "CLUNG", "CLUNK", "CLYPE",
    "CNIDA", "COACH", "COACT", "COALA", "COALS", "COALY", "COAPT", "COARB", "COAST", "COATE",
    "COATI", "COATS", "COBBS", "COBBY", "COBIA", "COBLE", "COBRA", "COBZA", "COCAS", "COCCI",
    "COCCO", "COCKS", "COCKY", "COCOA", "COCOS", "CODAS", "CODEC", "CODED", "CODEN", "CODER",
    "CODES", "CODEX", "CODON", "COEDS", "COFFS", "COGIE", "COGON", "COGUE", "COHAB", "COHOE",
    "COHOG", "COHOS", "COIFS", "COIGN", "COILS", "COINS", "COIRS", "COITS", "COKED", "COKES",
    "COLAS", "COLBY", "COLDS", "COLED", "COLES", "COLEY", "COLIC", "COLIN", "COLLS", "COLLY",
    "COLOG", "COLON", "COLOR", "COLTS", "COLZA", "COMAE", "COMAL", "COMAS", "COMBE", "COMBI",
    "COMBO", "COMBS", "COMBY", "COMER", "COMES", "COMET", "COMFY", "COMIC", "COMIX", "COMMA",
    "COMMO", "COMMS", "COMMY", "COMPO", "COMPS", "COMPT", "COMTE", "COMUS", "CONCH", "CONDO",
    "CONED", "CONES", "CONEY", "CONFS", "CONGA", "CONGE", "CONGO", "CONIA", "CONIC", "CONIN",
    "CONKS", "CONKY", "CONNE", "CONNS", "CONTE", "CONTO", "CONUS", "CONVO", "COOCH", "COOED",
    "COOEE", "COOER", "COOEY", "COOFS", "COOKS", "COOKY", "COOLS", "COOLY", "COOMB", "COOMS",
    "COOMY", "COONS", "COOPS", "COOPT", "COOST", "COOTS", "COOZE", "COPAL", "COPAY", "COPED",
    "COPEN", "COPER", "COPES", "COPPY", "COPRA", "COPSE", "COPSY", "CORAL", "CORAM", "CORBE",
    "CORBY", "CORDS", "CORED", "CORER", "CORES", "COREY", "CORGI", "CORIA", "CORKS", "CORKY",
    "CORMS", "CORNI", "CORNO", "CORNS", "CORNU", "CORNY", "CORPS", "CORSE", "CORSO", "COSEC",
    "COSED", "COSES", "COSET", "COSEY", "COSIE", "COSTA", "COSTE", "COSTS", "COTAN", "COTED",
    "COTES", "COTHS", "COTTA", "COTTS", "COUCH", "COUDE", "COUGH", "COULD", "COUNT", "COUPE",
    "COUPS", "COURB", "COURD", "COURE", "COURS", "COURT", "COUTH", "COVED", "COVEN", "COVER",
    "COVES", "COVET", "COVEY", "COVIN", "COWAL", "COWAN", "COWED", "COWER", "COWKS", "COWLS",
    "COWPS", "COWRY", "COXAE", "COXAL", "COXED", "COXES", "COYED", "COYER", "COYLY", "COYPU",
    "COZED", "COZEN", "COZES", "COZEY", "COZIE", "CRAAL", "CRABS", "CRACK", "CRAFT", "CRAGS",
    "CRAIC", "CRAIG", "CRAKE", "CRAME", "CRAMP", "CRAMS", "CRANE", "CRANK", "CRANS", "CRAPE",
    "CRAPS", "CRAPY", "CRARE", "CRASH", "CRASS", "CRATE", "CRAVE", "CRAWL", "CRAWS", "CRAYS",
    "CRAZE", "CRAZY", "CREAK", "CREAM", "CREDO", "CREDS", "CREED", "CREEK", "CREEL", "CREEP",
    "CREES", "CREME", "CREMS", "CRENA", "CREPE", "CREPT", "CREPY", "CRESS", "CREST", "CREWE",
    "CREWS", "CRIBS", "CRICK", "CRIED", "CRIER", "CRIES", "CRIME", "CRIMP", "CRIMS", "CRINE",
    "CRIOS", "CRIPE", "CRISE", "CRISP", "CRITH", "CRITS", "CROAK", "CROCI", "CROCK", "CROCS",
    "CROFT", "CROGS", "CROMB", "CROME", "CRONE", "CRONK", "CRONY", "CROOK", "CROOL", "CROON",
    "CROPS", "CRORE", "CROSS", "CROST", "CROUP", "CROUT", "CROWD", "CROWN", "CROWS", "CROZE",
    "CRUCK", "CRUDE", "CRUDS", "CRUDY", "CRUEL", "CRUES", "CRUET", "CRUMB", "CRUMP", "CRUOR",
    "CRURA", "CRUSE", "CRUSH", "CRUST", "CRUSY", "CRUVE", "CRWTH", "CRYPT", "CTENE", "CUBBY",
    "CUBEB", "CUBED", "CUBER", "CUBES", "CUBIC", "CUBIT", "CUDDY", "CUFFO", "CUFFS", "CUIFS",
    "CUING", "CUISH", "CUITS", "CUKES", "CULCH", "CULET", "CULEX", "CULLS", "CULLY", "CULMS",
    "CULPA", "CULTI", "CULTS", "CULTY", "CUMEC", "CUMIN", "CUNDY", "CUNEI", "CUNTS", "CUPEL",
    "CUPID", "CUPPA", "CUPPY", "CURAT", "CURBS", "CURCH", "CURDS", "CURDY", "CURED", "CURER",
    "CURES", "CURET", "CURFS", "CURIA", "CURIE", "CURIO", "CURLI", "CURLS", "CURLY", "CURNS",
    "CURNY", "CURRS", "CURRY", "CURSE", "CURSI", "CURST", "CURVE", "CURVY", "CUSEC", "CUSHY",
    "CUSKS", "CUSPS", "CUSSO", "CUTCH", "CUTER", "CUTES", "CUTEY", "CUTIE", "CUTIN", "CUTIS",
    "CUTTO", "CUTTY", "CUTUP", "CUVEE", "CWTCH", "CYANO", "CYANS", "CYBER", "CYCAD", "CYCAS",
    "CYCLE", "CYCLO", "CYDER", "CYLIX", "CYMAE", "CYMAR", "CYMAS", "CYMES", "CYMOL", "CYNIC",
    "CYSTS", "CYTES", "CYTON", "CZARS", "DABBA", "DACES", "DACHA", "DACKS", "DADAH", "DADAS",
    "DADDY", "DADOS", "DAFFS", "DAFFY", "DAGGA", "DAGGY", "DAGOS", "DAHLS", "DAILY", "DAINE",
    "DAINT", "DAIRY", "DAISY", "DAKER", "DALED", "DALES", "DALIS", "DALLE", "DALLY", "DALTS",
    "DAMAN", "DAMAR", "DAMES", "DAMME", "DAMNS", "DAMPS", "DAMPY", "DANCE", "DANDY", "DANGS",
    "DANIO", "DANKS", "DANNY", "DANTS", "DARAF", "DARBS", "DARCY", "DARED", "DARER", "DARES",
    "DARGA", "DARGS", "DARIC", "DARIS", "DARKS", "DARKY", "DARNS", "DARRE", "DARTS", "DARZI",
    "DASHI", "DASHY", "DATAL", "DATED", "DATER", "DATES", "DATOS", "DATTO", "DATUM", "DAUBE",
    "DAUBS", "DAUBY", "DAUDS", "DAULT", "DAUNT", "DAURS", "DAUTS", "DAVEN", "DAVIT", "DAWAH",
    "DAWDS", "DAWED", "DAWEN", "DAWKS", "DAWNS", "DAWTS", "DAYAN", "DAYCH", "DAYNT", "DAZED",
    "DAZER", "DAZES", "DEADS", "DEAIR", "DEALS", "DEALT", "DEANS", "DEARE", "DEARN", "DEARS",
    "DEARY", "DEASH", "DEATH", "DEAVE", "DEAWS", "DEAWY", "DEBAG", "DEBAR", "DEBBY", "DEBEL",
    "DEBES", "DEBIT", "DEBTS", "DEBUD", "DEBUG", "DEBUS", "DEBUT", "DEBYE", "DECAD", "DECAF",
    "DECAL", "DECAY", "DECKO", "DECKS", "DECOR", "DECOS", "DECOY", "DECRY", "DEDAL", "DEEDS",
    "DEEDY", "DEELY", "DEEMS", "DEENS", "DEEPS", "DEERE", "DEERS", "DEETS", "DEEVE", "DEEVS",
    "DEFAT", "DEFER", "DEFFO", "DEFIS", "DEFOG", "DEGAS", "DEGUM", "DEICE", "DEIDS", "DEIFY",
    "DEIGN", "DEILS", "DEISM", "DEIST", "DEITY", "DEKED", "DEKES", "DEKKO", "DELAY", "DELED",
    "DELES", "DELFS", "DELFT", "DELIS", "DELLS", "DELLY", "DELOS", "DELPH", "DELTA", "DELTS",
    "DELVE", "DEMAN", "DEMES", "DEMIC", "DEMIT", "DEMOB", "DEMON", "DEMOS", "DEMPT", "DEMUR",
    "DENAR", "DENAY", "DENES", "DENET", "DENIM", "DENIS", "DENSE", "DENTS", "DEOXY", "DEPOT",
    "DEPTH", "DERAT", "DERAY", "DERBY", "DERED", "DERES", "DERIG", "DERMA", "DERMS", "DERNS",
    "DEROS", "DERRO", "DERRY", "DERTH", "DERVS", "DESEX", "DESHI", "DESKS", "DESSE", "DETER",
    "DETOX", "DEUCE", "DEVAS", "DEVEL", "DEVIL", "DEVON", "DEVOT", "DEWAN", "DEWAR", "DEWAX",
    "DEWED", "DEXES", "DEXIE", "DHAKS", "DHALS", "DHOBI", "DHOLE", "DHOLL", "DHOLS", "DHOTI",
    "DHOWS", "DHUTI", "DIACT", "DIALS", "DIARY", "DIAZO", "DIBBS", "DICED", "DICER", "DICES",
    "DICEY", "DICHT", "DICKS", "DICKY", "DICOT", "DICTA", "DICTS", "DICTY", "DIDDY", "DIDIE",
    "DIDOS", "DIDST", "DIEBS", "DIENE", "DIETS", "DIFFS", "DIGHT", "DIGIT", "DIKAS", "DIKED",
    "DIKER", "DIKES", "DIKEY", "DILDO", "DILLI", "DILLS", "DILLY", "DIMER", "DIMES", "DIMLY",
    "DIMPS", "DINAR", "DINED", "DINER", "DINES", "DINGE", "DINGO", "DINGS", "DINGY", "DINIC",
    "DINKS", "DINKY", "DINNA", "DINOS", "DINTS", "DIODE", "DIOLS", "DIOTA", "DIPPY", "DIPSO",
    "DIRAM", "DIRER", "DIRGE", "DIRKE", "DIRKS", "DIRLS", "DIRTS", "DIRTY", "DISAS", "DISCI",
    "DISCO", "DISCS", "DISHY", "DISKS", "DISME", "DITAL", "DITAS", "DITCH", "DITED", "DITES",
    "DITSY", "DITTO", "DITTS", "DITTY", "DITZY", "DIVAN", "DIVAS", "DIVED", "DIVER", "DIVES",
    "DIVIS", "DIVOT", "DIVVY", "DIWAN", "DIXIE", "DIXIT", "DIZEN", "DIZZY", "DJINN", "DJINS",
    "DOABS", "DOATS", "DOBBY", "DOBIE", "DOBLA", "DOBRA", "DOBRO", "DOCHT", "DOCKS", "DOCOS",
    "DODDY", "DODGE", "DODGY", "DODOS", "DOEKS", "DOERS", "DOEST", "DOETH", "DOFFS", "DOGES",
    "DOGEY", "DOGGO", "DOGGY", "DOGIE", "DOGMA", "DOHYO", "DOILT", "DOILY", "DOING", "DOITS",
    "DOJOS", "DOLCE", "DOLCI", "DOLED", "DOLES", "DOLIA", "DOLLS", "DOLLY", "DOLMA", "DOLOR",
    "DOLOS", "DOLTS", "DOMAL", "DOMED", "DOMES", "DOMIC", "DONAH", "DONAS", "DONEE", "DONER",
    "DONGA", "DONGS", "DONKO", "DONNA", "DONNE", "DONNY", "DONOR", "DONSY", "DONUT", "DOOBS",
    "DOODY", "DOOKS", "DOOLE", "DOOLS", "DOOLY", "DOOMS", "DOOMY", "DOONA", "DOORN", "DOORS",
    "DOOZY", "DOPAS", "DOPED", "DOPER", "DOPES", "DOPEY", "DORAD", "DORBA", "DORBS", "DOREE",
    "DORIC", "DORIS", "DORKS", "DORKY", "DORMS", "DORMY", "DORPS", "DORRS", "DORSA", "DORSE",
    "DORTS", "DORTY", "DOSED", "DOSEH", "DOSER", "DOSES", "DOTAL", "DOTED", "DOTER", "DOTES",
    "DOTTY", "DOUAR", "DOUBT", "DOUCE", "DOUCS", "DOUGH", "DOUKS", "DOULA", "DOUMA", "DOUMS",
    "DOUPS", "DOURA", "DOUSE", "DOUTS", "DOVED", "DOVEN", "DOVER", "DOVES", "DOVIE", "DOWAR",
    "DOWDS", "DOWDY", "DOWED", "DOWEL", "DOWER", "DOWIE", "DOWLE", "DOWLS", "DOWLY", "DOWNA",
    "DOWNS", "DOWNY", "DOWPS", "DOWRY", "DOWSE", "DOWTS", "DOXIE", "DOYEN", "DOYLY", "DOZED",
    "DOZEN", "DOZER", "DOZES", "DRABS", "DRACK", "DRACO", "DRAFF", "DRAFT", "DRAGS", "DRAIL",
    "DRAIN", "DRAKE", "DRAMA", "DRAMS", "DRANK", "DRANT", "DRAPE", "DRAPS", "DRATS", "DRAVE",
    "DRAWL", "DRAWN", "DRAWS", "DRAYS", "DREAD", "DREAM", "DREAR", "DRECK", "DREED", "DREES",
    "DREGS", "DREKS", "DRENT", "DRERE", "DRESS", "DREST", "DREYS", "DRIBS", "DRICE", "DRIED",
    "DRIER", "DRIES", "DRIFT", "DRILL", "DRILY", "DRINK", "DRIPS", "DRIPT", "DRIVE", "DROID",
    "DROIL", "DROIT", "DROLE", "DROLL", "DROME", "DRONE", "DRONY", "DROOB", "DROOG", "DROOK",
    "DROOL", "DROOP", "DROPS", "DROPT", "DROSS", "DROUK", "DROVE", "DROWN", "DROWS", "DRUBS",
    "DRUGS", "DRUID", "DRUMS", "DRUNK", "DRUPE", "DRUSE", "DRUSY", "DRUXY", "DRYAD", "DRYER",
    "DRYLY", "DSOBO", "DSOMO", "DUADS", "DUALS", "DUANS", "DUARS", "DUBBO", "DUCAL", "DUCAT",
    "DUCES", "DUCHY", "DUCKS", "DUCKY", "DUCTS", "DUDDY", "DUDED", "DUDES", "DUELS", "DUETS",
    "DUETT", "DUFFS", "DUFUS", "DUING", "DUITS", "DUKAS", "DUKED", "DUKES", "DUKKA", "DULES",
    "DULIA", "DULLS", "DULLY", "DULSE", "DUMAS", "DUMBO", "DUMBS", "DUMKA", "DUMKY", "DUMMY",
    "DUMPS", "DUMPY", "DUNAM", "DUNCE", "DUNCH", "DUNES", "DUNGS", "DUNGY", "DUNKS", "DUNNO",
    "DUNNY", "DUNSH", "DUNTS", "DUOMI", "DUOMO", "DUPED", "DUPER", "DUPES", "DUPLE", "DUPLY",
    "DUPPY", "DURAL", "DURAS", "DURED", "DURES", "DURGY", "DURNS", "DUROC", "DUROS", "DUROY",
    "DURRA", "DURRS", "DURRY", "DURST", "DURUM", "DURZI", "DUSKS", "DUSKY", "DUSTS", "DUSTY",
    "DUTCH", "DUVET", "DUXES", "DWAAL", "DWALE", "DWALM", "DWAMS", "DWANG", "DWARF", "DWAUM",
    "DWEEB", "DWELL", "DWELT", "DWILE", "DWINE", "DYADS", "DYERS", "DYING", "DYKED", "DYKES",
    "DYKEY", "DYNEL", "DYNES", "DZHOS", "EAGER", "EAGLE", "EAGRE", "EALES", "EANED", "EARDS",
    "EARED", "EARLS", "EARLY", "EARNS", "EARST", "EARTH", "EASED", "EASEL", "EASER", "EASES",
    "EASLE", "EASTS", "EATEN", "EATER", "EATHE", "EAVED", "EAVES", "EBBED", "EBBET", "EBONS",
    "EBONY", "EBOOK", "ECADS", "ECHED", "ECHES", "ECHOS", "ECLAT", "ECRUS", "EDEMA", "EDGED",
    "EDGER", "EDGES", "EDICT", "EDIFY", "EDILE", "EDITS", "EDUCE", "EDUCT", "EEJIT", "EERIE",
    "EEVEN", "EEVNS", "EFFED", "EGADS", "EGERS", "EGEST", "EGGAR", "EGGED", "EGGER", "EGMAS",
    "EGRET", "EHING", "EIDER", "EIDOS", "EIGHT", "EIGNE", "EIKED", "EIKON", "EILDS", "EISEL",
    "EJECT", "EKING", "EKKAS", "ELAIN", "ELAND", "ELANS", "ELATE", "ELBOW", "ELCHI", "ELDER",
    "ELDIN", "ELECT", "ELEGY", "ELEMI", "ELFED", "ELFIN", "ELIAD", "ELIDE", "ELINT", "ELITE",
    "ELMEN", "ELOGE", "ELOGY", "ELOIN", "ELOPE", "ELOPS", "ELPEE", "ELSIN", "ELUDE", "ELUTE",
    "ELVAN", "ELVER", "ELVES", "EMACS", "EMAIL", "EMBAR", "EMBAY", "EMBED", "EMBER", "EMBOG",
    "EMBOW", "EMBOX", "EMBUS", "EMCEE", "EMEER", "EMEND", "EMERY", "EMEUS", "EMIRS", "EMITS",
    "EMMAS", "EMMER", "EMMET", "EMMEW", "EMMYS", "EMONG", "EMOTE", "EMOVE", "EMPTS", "EMPTY",
    "EMULE", "EMURE", "EMYDE", "EMYDS", "ENACT", "ENARM", "ENATE", "ENDED", "ENDER", "ENDEW",
    "ENDOW", "ENDUE", "ENEMA", "ENEMY", "ENEWS", "ENFIX", "ENIAC", "ENJOY", "ENLIT", "ENMEW",
    "ENNOG", "ENNUI", "ENOKI", "ENOLS", "ENORM", "ENOWS", "ENROL", "ENSEW", "ENSKY", "ENSUE",
    "ENTER", "ENTIA", "ENTRY", "ENURE", "ENVOI", "ENVOY", "ENZYM", "EORLS", "EOSIN", "EPACT",
    "EPEES", "EPHAH", "EPHAS", "EPHOD", "EPHOR", "EPICS", "EPOCH", "EPODE", "EPOPT", "EPOXY",
    "EPRIS", "EPROM", "EQUAL", "EQUID", "EQUIP", "ERASE", "ERBIA", "ERECT", "EREVS", "ERGON",
    "ERGOS", "ERGOT", "ERICA", "ERICK", "ERICS", "ERING", "ERNED", "ERNES", "ERODE", "EROSE",
    "ERRED", "ERROR", "ERSES", "ERUCT", "ERUGO", "ERUPT", "ERUVS", "ERVEN", "ERVIL", "ESCAR",
    "ESCOT", "ESILE", "ESKAR", "ESKER", "ESNES", "ESSAY", "ESSES", "ESTER", "ESTOC", "ESTOP",
    "ESTRO", "ETAGE", "ETAPE", "ETATS", "ETENS", "ETHAL", "ETHER", "ETHIC", "ETHOS", "ETHYL",
    "ETNAS", "ETTIN", "ETTLE", "ETUDE", "ETUIS", "ETWEE", "ETYMA", "EUGHS", "EUKED", "EUPAD",
    "EUROS", "EUSOL", "EVADE", "EVENS", "EVENT", "EVERT", "EVERY", "EVETS", "EVHOE", "EVICT",
    "EVILS", "EVITE", "EVOHE", "EVOKE", "EWERS", "EWEST", "EWHOW", "EWKED", "EXACT", "EXALT",
    "EXAMS", "EXCEL", "EXEAT", "EXECS", "EXEEM", "EXEME", "EXERT", "EXIES", "EXILE", "EXINE",
    "EXING", "EXIST", "EXITS", "EXODE", "EXONS", "EXPAT", "EXPEL", "EXPOS", "EXTOL", "EXTRA",
    "EXUDE", "EXULS", "EXULT", "EXURB", "EYASS", "EYERS", "EYING", "EYOTS", "EYRAS", "EYRES",
    "EYRIE", "EYRIR", "FABLE", "FACED", "FACER", "FACES", "FACET", "FACIA", "FACTS", "FADDY",
    "FADED", "FADER", "FADES", "FADGE", "FADOS", "FAENA", "FAERY", "FAFFS", "FAGGY", "FAGIN",
    "FAGOT", "FAIKS", "FAILS", "FAINE", "FAINS", "FAINT", "FAIRS", "FAIRY", "FAITH", "FAKED",
    "FAKER", "FAKES", "FAKEY", "FAKIR", "FALAJ", "FALLS", "FALSE", "FAMED", "FAMES", "FANAL",
    "FANCY", "FANDS", "FANES", "FANGA", "FANGO", "FANGS", "FANKS", "FANNY", "FANON", "FANOS",
    "FANUM", "FAQIR", "FARAD", "FARCE", "FARCI", "FARCY", "FARDS", "FARED", "FARER", "FARES",
    "FARLE", "FARLS", "FARMS", "FAROS", "FARSE", "FARTS", "FASCI", "FASTI", "FASTS", "FATAL",
    "FATED", "FATES", "FATLY", "FATSO", "FATTY", "FATWA", "FAUGH", "FAULD", "FAULT", "FAUNA",
    "FAUNS", "FAURD", "FAUTS", "FAUVE", "FAVAS", "FAVEL", "FAVER", "FAVES", "FAVOR", "FAVUS",
    "FAWNS", "FAWNY", "FAXED", "FAXES", "FAYED", "FAYER", "FAYNE", "FAYRE", "FAZED", "FAZES",
    "FEALS", "FEARE", "FEARS", "FEASE", "FEAST", "FEATS", "FEAZE", "FECAL", "FECES", "FECHT",
    "FECIT", "FECKS", "FEDEX", "FEEBS", "FEEDS", "FEELS", "FEENS", "FEERS", "FEESE", "FEEZE",
    "FEHME", "FEIGN", "FEINT", "FEIST", "FELID", "FELLA", "FELLS", "FELLY", "FELON", "FELTS",
    "FELTY", "FEMAL", "FEMES", "FEMME", "FEMMY", "FEMUR", "FENCE", "FENDS", "FENDY", "FENIS",
    "FENKS", "FENNY", "FENTS", "FEODS", "FEOFF", "FERAL", "FERER", "FERES", "FERIA", "FERLY",
    "FERMI", "FERMS", "FERNS", "FERNY", "FERRY", "FESSE", "FESTA", "FESTS", "FESTY", "FETAL",
    "FETAS", "FETCH", "FETED", "FETES", "FETID", "FETOR", "FETTA", "FETTS", "FETUS", "FETWA",
    "FEUAR", "FEUDS", "FEUED", "FEVER", "FEWER", "FEYED", "FEYER", "FEYLY", "FEZES", "FEZZY",
    "FIARS", "FIATS", "FIBER", "FIBRE", "FIBRO", "FICES", "FICHE", "FICHU", "FICIN", "FICOS",
    "FICUS", "FIDGE", "FIDOS", "FIEFS", "FIELD", "FIEND", "FIENT", "FIERE", "FIERS", "FIERY",
    "FIEST", "FIFED", "FIFER", "FIFES", "FIFTH", "FIFTY", "FIGHT", "FIGOS", "FIKED", "FIKES",
    "FILAR", "FILCH", "FILED", "FILER", "FILES", "FILET", "FILLE", "FILLO", "FILLS", "FILLY",
    "FILMI", "FILMS", "FILMY", "FILOS", "FILTH", "FILUM", "FINAL", "FINCA", "FINCH", "FINDS",
    "FINED", "FINER", "FINES", "FINIS", "FINKS", "FINNY", "FINOS", "FIORD", "FIQUE", "FIRED",
    "FIRER", "FIRES", "FIRIE", "FIRKS", "FIRMS", "FIRNS", "FIRRY", "FIRST", "FIRTH", "FISCS",
    "FISHY", "FISKS", "FISTS", "FISTY", "FITCH", "FITLY", "FITNA", "FITTE", "FITTS", "FIVER",
    "FIVES", "FIXED", "FIXER", "FIXES", "FIXIT", "FIZZY", "FJELD", "FJORD", "FLABS", "FLACK",
    "FLAFF", "FLAGS", "FLAIL", "FLAIR", "FLAKE", "FLAKS", "FLAKY", "FLAME", "FLAMM", "FLAMS",
    "FLAMY", "FLANK", "FLANS", "FLAPS", "FLARE", "FLARY", "FLASH", "FLASK", "FLATS", "FLAWN",
    "FLAWS", "FLAWY", "FLAXY", "FLAYS", "FLEAM", "FLEAS", "FLECK", "FLEER", "FLEES", "FLEET",
    "FLEGS", "FLEME", "FLESH", "FLEWS", "FLEXO", "FLEYS", "FLICK", "FLICS", "FLIED", "FLIER",
    "FLIES", "FLIMP", "FLIMS", "FLING", "FLINT", "FLIPS", "FLIRS", "FLIRT", "FLISK", "FLITE",
    "FLITS", "FLITT", "FLOAT", "FLOCK", "FLOCS", "FLOES", "FLOGS", "FLONG", "FLOOD", "FLOOR",
    "FLOPS", "FLORA", "FLORS", "FLORY", "FLOSH", "FLOSS", "FLOTA", "FLOTE", "FLOUR", "FLOUT",
    "FLOWN", "FLOWS", "FLUBS", "FLUED", "FLUES", "FLUEY", "FLUFF", "FLUID", "FLUKE", "FLUKY",
    "FLUME", "FLUMP", "FLUNG", "FLUNK", "FLUOR", "FLURR", "FLUSH", "FLUTE", "FLUTY", "FLUYT",
    "FLYBY", "FLYER", "FLYPE", "FLYTE", "FOALS", "FOAMS", "FOAMY", "FOCAL", "FOCUS", "FOEHN",
    "FOGEY", "FOGGY", "FOGIE", "FOGLE", "FOHNS", "FOIDS", "FOILS", "FOINS", "FOIST", "FOLDS",
    "FOLEY", "FOLIA", "FOLIC", "FOLIE", "FOLIO", "FOLKS", "FOLKY", "FOLLY", "FOMES", "FONDA",
    "FONDS", "FONDU", "FONLY", "FONTS", "FOODS", "FOODY", "FOOLS", "FOOTS", "FOOTY", "FORAM",
    "FORAY", "FORBS", "FORBY", "FORCE", "FORDO", "FORDS", "FOREL", "FORES", "FOREX", "FORGE",
    "FORGO", "FORKS", "FORKY", "FORME", "FORMS", "FORTE", "FORTH", "FORTS", "FORTY", "FORUM",
    "FORZA", "FORZE", "FOSSA", "FOSSE", "FOUAT", "FOUDS", "FOUER", "FOUET", "FOULE", "FOULS",
    "FOUND", "FOUNT", "FOURS", "FOUTH", "FOVEA", "FOWLS", "FOWTH", "FOXED", "FOXES", "FOXIE",
    "FOYER", "FOYLE", "FOYNE", "FRABS", "FRACK", "FRACT", "FRAGS", "FRAIL", "FRAIM", "FRAME",
    "FRANC", "FRANK", "FRAPE", "FRAPS", "FRASS", "FRATE", "FRATI", "FRATS", "FRAUD", "FRAUS",
    "FRAYS", "FREAK", "FREED", "FREER", "FREES", "FREET", "FREIT", "FREMD", "FRENA", "FREON",
    "FRERE", "FRESH", "FRETS", "FRIAR", "FRIBS", "FRIED", "FRIER", "FRIES", "FRIGS", "FRILL",
    "FRISE", "FRISK", "FRIST", "FRITH", "FRITS", "FRITT", "FRITZ", "FRIZE", "FRIZZ", "FROCK",
    "FROES", "FROGS", "FROND", "FRONS", "FRONT", "FRORE", "FRORN", "FRORY", "FROSH", "FROST",
    "FROTH", "FROWN", "FROWS", "FROWY", "FROZE", "FRUGS", "FRUIT", "FRUMP", "FRUSH", "FRUST",
    "FRYER", "FUBAR", "FUBBY", "FUBSY", "FUCKS", "FUCUS", "FUDDY", "FUDGE", "FUELS", "FUERO",
    "FUFFS", "FUFFY", "FUGAL", "FUGGY", "FUGIE", "FUGIO", "FUGLE", "FUGLY", "FUGUE", "FUGUS",
    "FUJIS", "FULLS", "FULLY", "FUMED", "FUMER", "FUMES", "FUMET", "FUNDI", "FUNDS", "FUNDY",
    "FUNGI", "FUNGO", "FUNGS", "FUNKS", "FUNKY", "FUNNY", "FURAL", "FURAN", "FURCA", "FURLS",
    "FUROL", "FUROR", "FURRS", "FURRY", "FURTH", "FURZE", "FURZY", "FUSED", "FUSEE", "FUSEL",
    "FUSES", "FUSIL", "FUSSY", "FUSTS", "FUSTY", "FUTON", "FUZED", "FUZEE", "FUZES", "FUZIL",
    "FUZZY", "FYCES", "FYKED", "FYKES", "FYLES", "FYRDS", "FYTTE", "GABBY", "GABLE", "GADDI",
    "GADES", "GADGE", "GADID", "GADIS", "GADJE", "GADJO", "GADSO", "GAFFE", "GAFFS", "GAGED",
    "GAGER", "GAGES", "GAIDS", "GAILY", "GAINS", "GAIRS", "GAITS", "GAITT", "GAJOS", "GALAH",
    "GALAS", "GALAX", "GALEA", "GALES", "GALLS", "GALLY", "GALOP", "GALUT", "GALVO", "GAMAS",
    "GAMAY", "GAMBA", "GAMBE", "GAMBO", "GAMBS", "GAMED", "GAMER", "GAMES", "GAMEY", "GAMIC",
    "GAMIN", "GAMMA", "GAMME", "GAMMY", "GAMPS", "GAMUT", "GANCH", "GANDY", "GANEF", "GANEV",
    "GANGS", "GANJA", "GANOF", "GANTS", "GAOLS", "GAPED", "GAPER", "GAPES", "GAPOS", "GAPPY",
    "GARBE", "GARBO", "GARBS", "GARDA", "GARIS", "GARNI", "GARRE", "GARTH", "GARUM", "GASES",
    "GASPS", "GASPY", "GASSY", "GASTS", "GATED", "GATER", "GATES", "GATHS", "GATOR", "GAUCY",
    "GAUDS", "GAUDY", "GAUGE", "GAUJE", "GAULT", "GAUMS", "GAUMY", "GAUNT", "GAUPS", "GAURS",
    "GAUSS", "GAUZE", "GAUZY", "GAVEL", "GAVOT", "GAWCY", "GAWDS", "GAWKS", "GAWKY", "GAWPS",
    "GAWSY", "GAYAL", "GAYER", "GAYLY", "GAZAL", "GAZAR", "GAZED", "GAZER", "GAZES", "GAZON",
    "GAZOO", "GEALS", "GEANS", "GEARE", "GEARS", "GEATS", "GEBUR", "GECKO", "GECKS", "GEEKS",
    "GEEKY", "GEEPS", "GEESE", "GEEST", "GEIST", "GEITS", "GELDS", "GELEE", "GELID", "GELLY",
    "GELTS", "GEMEL", "GEMMA", "GEMMY", "GEMOT", "GENAL", "GENAS", "GENES", "GENET", "GENIC",
    "GENIE", "GENII", "GENIP", "GENNY", "GENOA", "GENOM", "GENRE", "GENRO", "GENTS", "GENTY",
    "GENUA", "GENUS", "GEODE", "GEOID", "GERAH", "GERBE", "GERES", "GERLE", "GERMS", "GERMY",
    "GERNE", "GESSE", "GESSO", "GESTE", "GESTS", "GETAS", "GETUP", "GEUMS", "GEYAN", "GEYER",
    "GHAST", "GHATS", "GHAUT", "GHAZI", "GHEES", "GHEST", "GHOST", "GHOUL", "GHYLL", "GIANT",
    "GIBED", "GIBEL", "GIBER", "GIBES", "GIBLI", "GIBUS", "GIDDY", "GIFTS", "GIGAS", "GIGHE",
    "GIGOT", "GIGUE", "GILAS", "GILDS", "GILET", "GILLS", "GILLY", "GILPY", "GILTS", "GIMEL",
    "GIMME", "GIMPS", "GIMPY", "GINGE", "GINGS", "GINKS", "GINNY", "GINZO", "GIPON", "GIPPO",
    "GIPPY", "GIPSY", "GIRDS", "GIRLS", "GIRLY", "GIRNS", "GIRON", "GIROS", "GIRRS", "GIRSH",
    "GIRTH", "GIRTS", "GISMO", "GISMS", "GISTS", "GITES", "GIUST", "GIVED", "GIVEN", "GIVER",
    "GIVES", "GIZMO", "GLACE", "GLADE", "GLADS", "GLADY", "GLAIK", "GLAIR", "GLAMS", "GLAND",
    "GLANS", "GLARE", "GLARY", "GLASS", "GLAUM", "GLAUR", "GLAZE", "GLAZY", "GLEAM", "GLEAN",
    "GLEBA", "GLEBE", "GLEBY", "GLEDE", "GLEDS", "GLEED", "GLEEK", "GLEES", "GLEET", "GLEIS",
    "GLENS", "GLENT", "GLEYS", "GLIAL", "GLIAS", "GLIBS", "GLIDE", "GLIFF", "GLIFT", "GLIKE",
    "GLIME", "GLIMS", "GLINT", "GLISK", "GLITS", "GLITZ", "GLOAM", "GLOAT", "GLOBE", "GLOBI",
    "GLOBS", "GLOBY", "GLODE", "GLOGG", "GLOMS", "GLOOM", "GLOOP", "GLOPS", "GLORY", "GLOSS",
    "GLOST", "GLOUT", "GLOVE", "GLOWS", "GLOZE", "GLUED", "GLUER", "GLUES", "GLUEY", "GLUGS",
    "GLUME", "GLUMS", "GLUON", "GLUTE", "GLUTS", "GLYPH", "GNARL", "GNARR", "GNARS", "GNASH",
    "GNATS", "GNAWN", "GNAWS", "GNOME", "GNOWS", "GOADS", "GOAFS", "GOALS", "GOARY", "GOATS",
    "GOATY", "GOBAN", "GOBBI", "GOBBO", "GOBBY", "GOBOS", "GODET", "GODLY", "GODSO", "GOELS",
    "GOERS", "GOETY", "GOFER", "GOFFS", "GOGGA", "GOGOS", "GOIER", "GOING", "GOLDS", "GOLDY",
    "GOLEM", "GOLES", "GOLFS", "GOLLY", "GOLPE", "GOLPS", "GOMBO", "GOMER", "GOMPA", "GONAD",
    "GONEF", "GONER", "GONGS", "GONIA", "GONIF", "GONKS", "GONNA", "GONOF", "GONYS", "GONZO",
    "GOOBY", "GOODS", "GOODY", "GOOEY", "GOOFS", "GOOFY", "GOOGS", "GOOKS", "GOOKY", "GOOLD",
    "GOOLS", "GOOLY", "GOONS", "GOONY", "GOOPS", "GOOPY", "GOORS", "GOORY", "GOOSE", "GOOSY",
    "GOPAK", "GOPIK", "GORAL", "GORAS", "GORED", "GORES", "GORGE", "GORIS", "GORMS", "GORMY",
    "GORPS", "GORSE", "GORSY", "GOSHT", "GOSSE", "GOTHS", "GOTTA", "GOUGE", "GOUKS", "GOURA",
    "GOURD", "GOUTS", "GOUTY", "GOWAN", "GOWDS", "GOWFS", "GOWKS", "GOWLS", "GOWNS", "GOXES",
    "GOYIM", "GRAAL", "GRABS", "GRACE", "GRADE", "GRADS", "GRAFF", "GRAFT", "GRAIL", "GRAIN",
    "GRAIP", "GRAMA", "GRAME", "GRAMP", "GRAMS", "GRANA", "GRAND", "GRANS", "GRANT", "GRAPE",
    "GRAPH", "GRAPY", "GRASP", "GRASS", "GRATE", "GRAVE", "GRAVS", "GRAVY", "GRAYS", "GRAZE",
    "GREAT", "GREBE", "GRECE", "GREED", "GREEK", "GREEN", "GREES", "GREET", "GREGE", "GREGO",
    "GREIN", "GRENS", "GRENZ", "GRESE", "GREVE", "GREWS", "GREYS", "GRICE", "GRIDE", "GRIDS",
    "GRIEF", "GRIFF", "GRIFT", "GRIGS", "GRIKE", "GRILL", "GRIME", "GRIMY", "GRIND", "GRINS",
    "GRIOT", "GRIPE", "GRIPS", "GRIPT", "GRIPY", "GRISE", "GRIST", "GRISY", "GRITH", "GRITS",
    "GRIZE", "GROAN", "GROAT", "GRODY", "GROGS", "GROIN", "GROKS", "GROMA", "GRONE", "GROOF",
    "GROOM", "GROPE", "GROSS", "GROSZ", "GROTS", "GROUF", "GROUP", "GROUT", "GROVE", "GROWL",
    "GROWN", "GROWS", "GRUBS", "GRUED", "GRUEL", "GRUES", "GRUFE", "GRUFF", "GRUME", "GRUMP",
    "GRUNT", "GRYCE", "GRYDE", "GRYKE", "GRYPE", "GRYPT", "GUACO", "GUANA", "GUANO", "GUANS",
    "GUARD", "GUARS", "GUAVA", "GUCKS", "GUCKY", "GUDES", "GUESS", "GUEST", "GUFFS", "GUGAS",
    "GUIDE", "GUIDS", "GUILD", "GUILE", "GUILT", "GUIMP", "GUIRO", "GUISE", "GULAG", "GULAR",
    "GULAS", "GULCH", "GULES", "GULFS", "GULFY", "GULLS", "GULLY", "GULPH", "GULPS", "GULPY",
    "GUMBO", "GUMMA", "GUMMY", "GUMPS", "GUNDY", "GUNGE", "GUNGY", "GUNKS", "GUNKY", "GUNNY",
    "GUPPY", "GURGE", "GURLS", "GURLY", "GURNS", "GURRY", "GURSH", "GURUS", "GUSHY", "GUSLA",
    "GUSLE", "GUSLI", "GUSSY", "GUSTO", "GUSTS", "GUSTY", "GUTSY", "GUTTA", "GUTTY", "GUYED",
    "GUYLE", "GUYOT", "GUYSE", "GWINE", "GYALS", "GYBED", "GYBES", "GYELD", "GYMPS", "GYNAE",
    "GYNIE", "GYNNY", "GYOZA", "GYPPO", "GYPPY", "GYPSY", "GYRAL", "GYRED", "GYRES", "GYRON",
    "GYROS", "GYRUS", "GYTES", "GYVED", "GYVES", "HAAFS", "HAARS", "HABIT", "HABLE", "HABUS",
    "HACEK", "HACKS", "HADAL", "HADED", "HADES", "HADJI", "HADST", "HAEMS", "HAETS", "HAFFS",
    "HAFIZ", "HAFTS", "HAGGS", "HAHAS", "HAICK", "HAIKA", "HAIKS", "HAIKU", "HAILS", "HAILY",
    "HAINS", "HAINT", "HAIRS", "HAIRY", "HAITH", "HAJES", "HAJIS", "HAJJI", "HAKAM", "HAKAS",
    "HAKEA", "HAKES", "HAKIM", "HAKUS", "HALAL", "HALED", "HALER", "HALES", "HALFA", "HALFS",
    "HALID", "HALLO", "HALLS", "HALMA", "HALMS", "HALON", "HALOS", "HALSE", "HALTS", "HALVA",
    "HALVE", "HAMAL", "HAMBA", "HAMED", "HAMES", "HAMMY", "HAMZA", "HANAP", "HANCE", "HANCH",
    "HANDS", "HANDY", "HANGI", "HANGS", "HANKS", "HANKY", "HANSA", "HANSE", "HANTS", "HAOLE",
    "HAOMA", "HAPAX", "HAPLY", "HAPPY", "HAPUS", "HARAM", "HARDS", "HARDY", "HARED", "HAREM",
    "HARES", "HARIM", "HARKS", "HARLS", "HARMS", "HARNS", "HAROS", "HARPS", "HARPY", "HARRY",
    "HARSH", "HARTS", "HASHY", "HASKS", "HASPS", "HASTA", "HASTE", "HASTY", "HATCH", "HATED",
    "HATER", "HATES", "HATHA", "HAUDS", "HAUFS", "HAUGH", "HAULD", "HAULM", "HAULS", "HAULT",
    "HAUNT", "HAUSE", "HAUTE", "HAVEN", "HAVER", "HAVES", "HAVOC", "HAWED", "HAWKS", "HAWMS",
    "HAWSE", "HAYED", "HAYER", "HAYEY", "HAYLE", "HAZAN", "HAZED", "HAZEL", "HAZER", "HAZES",
    "HEADS", "HEADY", "HEALD", "HEALS", "HEAME", "HEAPS", "HEAPY", "HEARD", "HEARE", "HEARS",
    "HEART", "HEAST", "HEATH", "HEATS", "HEAVE", "HEAVY", "HEBEN", "HEBES", "HECHT", "HECKS",
    "HEDER", "HEDGE", "HEDGY", "HEEDS", "HEEDY", "HEELS", "HEEZE", "HEFTE", "HEFTS", "HEFTY",
    "HEIDS", "HEIGH", "HEILS", "HEIRS", "HEIST", "HEJAB", "HEJRA", "HELED", "HELES", "HELIO",
    "HELIX", "HELLO", "HELLS", "HELMS", "HELOS", "HELOT", "HELPS", "HELVE", "HEMAL", "HEMES",
    "HEMIC", "HEMIN", "HEMPS", "HEMPY", "HENCE", "HENDS", "HENGE", "HENNA", "HENNY", "HENRY",
    "HENTS", "HEPAR", "HERBS", "HERBY", "HERDS", "HERES", "HERLS", "HERMA", "HERMS", "HERNS",
    "HEROE", "HERON", "HEROS", "HERRY", "HERSE", "HERTZ", "HERYE", "HESPS", "HESTS", "HETES",
    "HETHS", "HEUCH", "HEUGH", "HEVEA", "HEWED", "HEWER", "HEWGH", "HEXAD", "HEXED", "HEXER",
    "HEXES", "HEXYL", "HEYED", "HIANT", "HICKS", "HIDED", "HIDER", "HIDES", "HIEMS", "HIGHS",
    "HIGHT", "HIJAB", "HIJRA", "HIKED", "HIKER", "HIKES", "HIKOI", "HILAR", "HILCH", "HILLO",
    "HILLS", "HILLY", "HILTS", "HILUM", "HILUS", "HIMBO", "HINAU", "HINDS", "HINGE", "HINGS",
    "HINKY", "HINNY", "HINTS", "HIOIS", "HIPLY", "HIPPO", "HIPPY", "HIRED", "HIREE", "HIRER",
    "HIRES", "HISSY", "HISTS", "HITCH", "HITHE", "HIVED", "HIVER", "HIVES", "HIZEN", "HOAED",
    "HOAGY", "HOARD", "HOARS", "HOARY", "HOAST", "HOBBY", "HOBOS", "HOCKS", "HOCUS", "HODAD",
    "HODJA", "HOERS", "HOGAN", "HOGEN", "HOGGS", "HOGHS", "HOHED", "HOICK", "HOIKS", "HOING",
    "HOISE", "HOIST", "HOKED", "HOKES", "HOKEY", "HOKIS", "HOKKU", "HOKUM", "HOLDS", "HOLED",
    "HOLES", "HOLEY", "HOLKS", "HOLLA", "HOLLO", "HOLLY", "HOLMS", "HOLON", "HOLTS", "HOMAS",
    "HOMED", "HOMER", "HOMES", "HOMEY", "HOMIE", "HOMME", "HOMOS", "HONAN", "HONDA", "HONDS",
    "HONED", "HONER", "HONES", "HONEY", "HONGI", "HONGS", "HONKS", "HONKY", "HONOR", "HOOCH",
    "HOODS", "HOODY", "HOOEY", "HOOFS", "HOOKA", "HOOKS", "HOOKY", "HOOLY", "HOONS", "HOOPS",
    "HOORD", "HOOSH", "HOOTS", "HOOTY", "HOOVE", "HOPED", "HOPER", "HOPES", "HOPPY", "HORAH",
    "HORAL", "HORAS", "HORDE", "HORIS", "HORME", "HORNS", "HORNY", "HORSE", "HORST", "HORSY",
    "HOSED", "HOSEL", "HOSEN", "HOSER", "HOSES", "HOSEY", "HOSTA", "HOSTS", "HOTCH", "HOTEL",
    "HOTEN", "HOTLY", "HOTTY", "HOUFF", "HOUFS", "HOUGH", "HOUND", "HOURI", "HOURS", "HOUSE",
    "HOUTS", "HOVEA", "HOVED", "HOVEL", "HOVEN", "HOVER", "HOVES", "HOWBE", "HOWDY", "HOWES",
    "HOWFF", "HOWFS", "HOWKS", "HOWLS", "HOWRE", "HOWSO", "HOXED", "HOXES", "HOYAS", "HOYED",
    "HOYLE", "HUBBY", "HUCKS", "HUDNA", "HUDUD", "HUERS", "HUFFS", "HUFFY", "HUGER", "HUGGY",
    "HUHUS", "HUIAS", "HULAS", "HULES", "HULKS", "HULKY", "HULLO", "HULLS", "HULLY", "HUMAN",
    "HUMAS", "HUMFS", "HUMIC", "HUMID", "HUMOR", "HUMPH", "HUMPS", "HUMPY", "HUMUS", "HUNCH",
    "HUNKS", "HUNKY", "HUNTS", "HURDS", "HURLS", "HURLY", "HURRA", "HURRY", "HURST", "HURTS",
    "HUSHY", "HUSKS", "HUSKY", "HUSOS", "HUSSY", "HUTCH", "HUTIA", "HUZZA", "HUZZY", "HWYLS",
    "HYDRA", "HYDRO", "HYENA", "HYENS", "HYING", "HYKES", "HYLAS", "HYLEG", "HYLES", "HYLIC",
    "HYMEN", "HYMNS", "HYNDE", "HYOID", "HYPED", "HYPER", "HYPES", "HYPHA", "HYPOS", "HYRAX",
    "HYSON", "HYTHE", "IAMBI", "IAMBS", "ICERS", "ICHED", "ICHES", "ICHOR", "ICIER", "ICILY",
    "ICING", "ICKER", "ICKLE", "ICONS", "ICTAL", "ICTIC", "ICTUS", "IDANT", "IDEAL", "IDEAS",
    "IDEES", "IDENT", "IDIOM", "IDIOT", "IDLED", "IDLER", "IDLES", "IDOLA", "IDOLS", "IDYLL",
    "IDYLS", "IFTAR", "IGAPO", "IGGED", "IGLOO", "IGLUS", "IHRAM", "IKANS", "IKATS", "IKONS",
    "ILEAC", "ILEAL", "ILEUM", "ILEUS", "ILIAC", "ILIAD", "ILIAL", "ILIUM", "ILLER", "ILLTH",
    "IMAGE", "IMAGO", "IMAMS", "IMARI", "IMAUM", "IMBAR", "IMBED", "IMBUE", "IMIDE", "IMIDO",
    "IMIDS", "IMINE", "IMINO", "IMMEW", "IMMIT", "IMMIX", "IMPED", "IMPEL", "IMPIS", "IMPLY",
    "IMPOT", "IMSHI", "IMSHY", "INANE", "INAPT", "INARM", "INBYE", "INCLE", "INCOG", "INCUR",
    "INCUS", "INCUT", "INDEW", "INDEX", "INDIE", "INDOL", "INDOW", "INDRI", "INDUE", "INEPT",
    "INERM", "INERT", "INFER", "INFIX", "INFOS", "INFRA", "INGAN", "INGLE", "INGOT", "INION",
    "INKED", "INKER", "INKLE", "INLAY", "INLET", "INNED", "INNER", "INORB", "INPUT", "INRUN",
    "INSET", "INTEL", "INTER", "INTIL", "INTIS", "INTRA", "INTRO", "INULA", "INURE", "INURN",
    "INUST", "INVAR", "INWIT", "IODIC", "IODID", "IODIN", "IONIC", "IOTAS", "IPPON", "IRADE",
    "IRATE", "IRIDS", "IRING", "IRKED", "IROKO", "IRONE", "IRONS", "IRONY", "ISBAS", "ISHES",
    "ISLED", "ISLES", "ISLET", "ISNAE", "ISSEI", "ISSUE", "ISTLE", "ITCHY", "ITEMS", "ITHER",
    "IVIED", "IVIES", "IVORY", "IXIAS", "IXORA", "IXTLE", "IZARD", "IZARS", "IZZAT", "JAAPS",
    "JABOT", "JACAL", "JACKS", "JACKY", "JADED", "JADES", "JAFAS", "JAGAS", "JAGER", "JAGGS",
    "JAGGY", "JAGIR", "JAGRA", "JAILS", "JAKES", "JAKEY", "JALAP", "JALOP", "JAMBE", "JAMBO",
    "JAMBS", "JAMBU", "JAMES", "JAMMY", "JANES", "JANNS", "JANNY", "JANTY", "JAPAN", "JAPED",
    "JAPER", "JAPES", "JARKS", "JARLS", "JARPS", "JARTA", "JARUL", "JASEY", "JASPE", "JASPS",
    "JATOS", "JAUKS", "JAUNT", "JAUPS", "JAVAS", "JAVEL", "JAWAN", "JAWED", "JAXIE", "JAZZY",
    "JEANS", "JEATS", "JEBEL", "JEDIS", "JEELS", "JEELY", "JEEPS", "JEERS", "JEFES", "JEFFS",
    "JEHAD", "JEHUS", "JELAB", "JELLO", "JELLS", "JELLY", "JEMBE", "JEMMY", "JENNY", "JERID",
    "JERKS", "JERKY", "JERRY", "JESSE", "JESTS", "JESUS", "JETES", "JETON", "JETTY", "JEUNE",
    "JEWED", "JEWEL", "JEWIE", "JHALA", "JIAOS", "JIBBS", "JIBED", "JIBER", "JIBES", "JIFFS",
    "JIFFY", "JIGGY", "JIGOT", "JIHAD", "JILLS", "JILTS", "JIMMY", "JIMPY", "JINGO", "JINKS",
    "JINNE", "JINNI", "JINNS", "JIRDS", "JIRGA", "JIRRE", "JISMS", "JIVED", "JIVER", "JIVES",
    "JIVEY", "JNANA", "JOBED", "JOBES", "JOCKO", "JOCKS", "JODEL", "JOEYS", "JOHNS", "JOINS",
    "JOINT", "JOIST", "JOKED", "JOKER", "JOKES", "JOKEY", "JOKOL", "JOLED", "JOLES", "JOLLS",
    "JOLLY", "JOLTS", "JOLTY", "JOMON", "JOMOS", "JONES", "JONGS", "JONTY", "JOOKS", "JORAM",
    "JORUM", "JOTAS", "JOTTY", "JOTUN", "JOUAL", "JOUGS", "JOUKS", "JOULE", "JOURS", "JOUST",
    "JOWAR", "JOWED", "JOWLS", "JOWLY", "JOYED", "JUBAS", "JUBES", "JUCOS", "JUDAS", "JUDGE",
    "JUDOS", "JUGAL", "JUGUM", "JUICE", "JUICY", "JUJUS", "JUKED", "JUKES", "JUKUS", "JULEP",
    "JUMAR", "JUMBO", "JUMBY", "JUMPS", "JUMPY", "JUNCO", "JUNKS", "JUNKY", "JUNTA", "JUNTO",
    "JUPES", "JUPON", "JURAL", "JURAT", "JUREL", "JUROR", "JUSTS", "JUTES", "JUTTY", "JUVES",
    "KAAMA", "KABAB", "KABAR", "KABOB", "KACHA", "KADES", "KADIS", "KAFIR", "KAGOS", "KAGUS",
    "KAHAL", "KAIAK", "KAIDS", "KAIES", "KAIFS", "KAIKA", "KAIKS", "KAILS", "KAIMS", "KAING",
    "KAINS", "KAKAS", "KAKIS", "KALAM", "KALES", "KALIF", "KALIS", "KALPA", "KAMAS", "KAMES",
    "KAMIK", "KAMIS", "KAMME", "KANAE", "KANAS", "KANDY", "KANEH", "KANES", "KANGA", "KANGS",
    "KANJI", "KANTS", "KANZU", "KAONS", "KAPAS", "KAPHS", "KAPOK", "KAPPA", "KAPUT", "KARAS",
    "KARAT", "KARKS", "KARMA", "KARNS", "KAROO", "KARRI", "KARST", "KARSY", "KARTS", "KARZY",
    "KASHA", "KASME", "KATAS", "KATIS", "KATTI", "KAUGH", "KAURI", "KAURU", "KAURY", "KAVAS",
    "KAWAS", "KAWAU", "KAWED", "KAYAK", "KAYLE", "KAYOS", "KAZIS", "KAZOO", "KBARS", "KEBAB",
    "KEBAR", "KEBOB", "KECKS", "KEDGE", "KEDGY", "KEECH", "KEEFS", "KEEKS", "KEELS", "KEENO",
    "KEENS", "KEEPS", "KEETS", "KEEVE", "KEFIR", "KEHUA", "KEIRS", "KELEP", "KELIM", "KELLS",
    "KELLY", "KELPS", "KELPY", "KELTS", "KELTY", "KEMBO", "KEMBS", "KEMPS", "KEMPT", "KEMPY",
    "KENAF", "KENCH", "KENDO", "KENOS", "KENTE", "KENTS", "KEPIS", "KERBS", "KEREL", "KERFS",
    "KERKY", "KERMA", "KERNE", "KERNS", "KEROS", "KERRY", "KERVE", "KESAR", "KESTS", "KETAS",
    "KETCH", "KETOL", "KEVEL", "KEVIL", "KEXES", "KEYED", "KHADI", "KHAFS", "KHAKI", "KHANS",
    "KHAPH", "KHATS", "KHAYA", "KHAZI", "KHEDA", "KHETH", "KHETS", "KHOJA", "KHORS", "KHOUM",
    "KHUDS", "KIAAT", "KIANG", "KIBBE", "KIBBI", "KIBEI", "KIBES", "KIBLA", "KICKS", "KICKY",
    "KIDDO", "KIDDY", "KIDEL", "KIDGE", "KIEFS", "KIERS", "KIEVE", "KIGHT", "KIKES", "KIKOI",
    "KILEY", "KILIM", "KILLS", "KILNS", "KILOS", "KILPS", "KILTS", "KILTY", "KIMBO", "KINAS",
    "KINDA", "KINDS", "KINDY", "KINES", "KINGS", "KININ", "KINKS", "KINKY", "KINOS", "KIORE",
    "KIOSK", "KIPES", "KIPPA", "KIPPS", "KIRBY", "KIRKS", "KIRNS", "KIRRI", "KISAN", "KISSY",
    "KISTS", "KITED", "KITER", "KITES", "KITHE", "KITHS", "KITTY", "KIVAS", "KIWIS", "KLANG",
    "KLAPS", "KLETT", "KLICK", "KLIEG", "KLIKS", "KLONG", "KLOOF", "KLUGE", "KLUTZ", "KNACK",
    "KNAGS", "KNAPS", "KNARL", "KNARS", "KNAUR", "KNAVE", "KNAWE", "KNEAD", "KNEED", "KNEEL",
    "KNEES", "KNELL", "KNELT", "KNIFE", "KNISH", "KNITS", "KNIVE", "KNOBS", "KNOCK", "KNOLL",
    "KNOPS", "KNOSP", "KNOTS", "KNOUT", "KNOWE", "KNOWN", "KNOWS", "KNUBS", "KNURL", "KNURR",
    "KNURS", "KNUTS", "KOALA", "KOANS", "KOAPS", "KOBAN", "KOBOS", "KOELS", "KOFFS", "KOFTA",
    "KOHAS", "KOHLS", "KOINE", "KOJIS", "KOKER", "KOKRA", "KOKUM", "KOLAS", "KOLOS", "KOMBU",
    "KONBU", "KONDO", "KONKS", "KOOKS", "KOOKY", "KOORI", "KOPEK", "KOPHS", "KOPJE", "KOPPA",
    "KORAI", "KORAS", "KORAT", "KORES", "KORMA", "KORUN", "KORUS", "KOSES", "KOTCH", "KOTOS",
    "KOTOW", "KOURA", "KRAAL", "KRABS", "KRAFT", "KRAIT", "KRANG", "KRANS", "KRANZ", "KRAUT",
    "KREEP", "KRENG", "KREWE", "KRILL", "KRONA", "KRONE", "KROON", "KRUBI", "KSARS", "KUDOS",
    "KUDUS", "KUDZU", "KUFIS", "KUGEL", "KUIAS", "KUKRI", "KUKUS", "KULAK", "KULAN", "KULAS",
    "KULFI", "KUMYS", "KURIS", "KURRE", "KURTA", "KURUS", "KUSSO", "KUTAS", "KUTCH", "KUTIS",
    "KUTUS", "KUZUS", "KVASS", "KVELL", "KWELA", "KYACK", "KYAKS", "KYANG", "KYARS", "KYATS",
    "KYBOS", "KYDST", "KYLES", "KYLIE", "KYLIN", "KYLIX", "KYLOE", "KYNDE", "KYNDS", "KYPES",
    "KYRIE", "KYTES", "KYTHE", "LAARI", "LABDA", "LABEL", "LABIA", "LABIS", "LABOR", "LABRA",
    "LACED", "LACER", "LACES", "LACET", "LACEY", "LACKS", "LADED", "LADEN", "LADER", "LADES",
    "LADLE", "LAERS", "LAEVO", "LAGAN", "LAGER", "LAHAR", "LAICH", "LAICS", "LAIDS", "LAIGH",
    "LAIKA", "LAIKS", "LAIRD", "LAIRS", "LAIRY", "LAITH", "LAITY", "LAKED", "LAKER", "LAKES",
    "LAKHS", "LAKIN", "LAKSA", "LALDY", "LALLS", "LAMAS", "LAMBS", "LAMBY", "LAMED", "LAMER",
    "LAMES", "LAMIA", "LAMMY", "LAMPS", "LANAI", "LANAS", "LANCE", "LANCH", "LANDE", "LANDS",
    "LANES", "LANKS", "LANKY", "LANTS", "LAPEL", "LAPIN", "LAPIS", "LAPJE", "LAPSE", "LARCH",
    "LARDS", "LARDY", "LAREE", "LARES", "LARGE", "LARGO", "LARIS", "LARKS", "LARKY", "LARNS",
    "LARUM", "LARVA", "LASED", "LASER", "LASES", "LASSI", "LASSO", "LASSU", "LASTS", "LATAH",
    "LATCH", "LATED", "LATEN", "LATER", "LATEX", "LATHE", "LATHI", "LATHS", "LATHY", "LATKE",
    "LATTE", "LAUAN", "LAUCH", "LAUDS", "LAUFS", "LAUGH", "LAUND", "LAURA", "LAVAS", "LAVED",
    "LAVER", "LAVES", "LAVRA", "LAWED", "LAWER", "LAWIN", "LAWKS", "LAWNS", "LAWNY", "LAXER",
    "LAXES", "LAXLY", "LAYED", "LAYER", "LAYIN", "LAYUP", "LAZAR", "LAZED", "LAZES", "LAZOS",
    "LAZZI", "LAZZO", "LEACH", "LEADS", "LEADY", "LEAFS", "LEAFY", "LEAKS", "LEAKY", "LEAMS",
    "LEANS", "LEANT", "LEANY", "LEAPS", "LEAPT", "LEARE", "LEARN", "LEARS", "LEARY", "LEASE",
    "LEASH", "LEAST", "LEATS", "LEAVE", "LEAVY", "LEAZE", "LEBEN", "LECCY", "LEDGE", "LEDGY",
    "LEDUM", "LEEAR", "LEECH", "LEEKS", "LEEPS", "LEERS", "LEERY", "LEESE", "LEETS", "LEFTE",
    "LEFTS", "LEFTY", "LEGAL", "LEGER", "LEGES", "LEGGE", "LEGGY", "LEGIT", "LEHRS", "LEHUA",
    "LEIRS", "LEISH", "LEMAN", "LEMED", "LEMEL", "LEMES", "LEMMA", "LEMON", "LEMUR", "LENDS",
    "LENES", "LENGS", "LENIS", "LENOS", "LENSE", "LENTI", "LENTO", "LEONE", "LEPER", "LEPID",
    "LEPRA", "LEPTA", "LERED", "LERES", "LERPS", "LESBO", "LESES", "LESTS", "LETCH", "LETHE",
    "LETUP", "LEUCH", "LEUCO", "LEUDS", "LEUGH", "LEVEE", "LEVEL", "LEVER", "LEVIN", "LEVIS",
    "LEWIS", "LEXES", "LEXIS", "LEZES", "LEZZA", "LEZZY", "LIANA", "LIANE", "LIANG", "LIARD",
    "LIARS", "LIART", "LIBEL", "LIBER", "LIBRA", "LIBRI", "LICHI", "LICHT", "LICIT", "LICKS",
    "LIDAR", "LIDOS", "LIEFS", "LIEGE", "LIENS", "LIERS", "LIEUS", "LIEVE", "LIFER", "LIFES",
    "LIFTS", "LIGAN", "LIGER", "LIGGE", "LIGHT", "LIGNE", "LIKED", "LIKEN", "LIKER", "LIKES",
    "LIKIN", "LILAC", "LILLS", "LILOS", "LILTS", "LIMAN", "LIMAS", "LIMAX", "LIMBA", "LIMBI",
    "LIMBO", "LIMBS", "LIMBY", "LIMED", "LIMEN", "LIMES", "LIMEY", "LIMIT", "LIMMA", "LIMNS",
    "LIMOS", "LIMPA", "LIMPS", "LINAC", "LINCH", "LINDS", "LINDY", "LINED", "LINEN", "LINER",
    "LINES", "LINEY", "LINGA", "LINGO", "LINGS", "LINGY", "LININ", "LINKS", "LINKY", "LINNS",
    "LINNY", "LINOS", "LINTS", "LINTY", "LINUM", "LINUX", "LIONS", "LIPID", "LIPIN", "LIPOS",
    "LIPPY", "LIRAS", "LIRKS", "LIROT", "LISKS", "LISLE", "LISPS", "LISTS", "LITAI", "LITAS",
    "LITED", "LITER", "LITES", "LITHE", "LITHO", "LITHS", "LITRE", "LIVED", "LIVEN", "LIVER",
    "LIVES", "LIVID", "LIVOR", "LIVRE", "LLAMA", "LLANO", "LOACH", "LOADS", "LOAFS", "LOAMS",
    "LOAMY", "LOANS", "LOAST", "LOATH", "LOAVE", "LOBAR", "LOBBY", "LOBED", "LOBES", "LOBOS",
    "LOBUS", "LOCAL", "LOCHS", "LOCKS", "LOCOS", "LOCUM", "LOCUS", "LODEN", "LODES", "LODGE",
    "LOESS", "LOFTS", "LOFTY", "LOGAN", "LOGES", "LOGGY", "LOGIA", "LOGIC", "LOGIE", "LOGIN",
    "LOGOI", "LOGON", "LOGOS", "LOHAN", "LOIDS", "LOINS", "LOIPE", "LOIRS", "LOKES", "LOLLS",
    "LOLLY", "LOLOG", "LOMAS", "LOMED", "LOMES", "LONER", "LONGA", "LONGE", "LONGS", "LOOBY",
    "LOOED", "LOOEY", "LOOFA", "LOOFS", "LOOIE", "LOOKS", "LOOMS", "LOONS", "LOONY", "LOOPS",
    "LOOPY", "LOORD", "LOOSE", "LOOTS", "LOPED", "LOPER", "LOPES", "LOPPY", "LORAL", "LORAN",
    "LORDS", "LORDY", "LOREL", "LORES", "LORIC", "LORIS", "LORRY", "LOSED", "LOSEL", "LOSEN",
    "LOSER", "LOSES", "LOSSY", "LOTAH", "LOTAS", "LOTES", "LOTIC", "LOTOS", "LOTTE", "LOTTO",
    "LOTUS", "LOUED", "LOUGH", "LOUIE", "LOUIS", "LOUMA", "LOUND", "LOUNS", "LOUPE", "LOUPS",
    "LOURE", "LOURS", "LOURY", "LOUSE", "LOUSY", "LOUTS", "LOVAT", "LOVED", "LOVER", "LOVES",
    "LOVEY", "LOWAN", "LOWED", "LOWER", "LOWES", "LOWLY", "LOWND", "LOWNE", "LOWNS", "LOWPS",
    "LOWRY", "LOWSE", "LOWTS", "LOXED", "LOXES", "LOYAL", "LOZEN", "LUACH", "LUAUS", "LUBED",
    "LUBES", "LUBRA", "LUCES", "LUCID", "LUCKS", "LUCKY", "LUCRE", "LUDES", "LUDIC", "LUDOS",
    "LUFFA", "LUFFS", "LUGED", "LUGER", "LUGES", "LULLS", "LULUS", "LUMAS", "LUMEN", "LUMME",
    "LUMMY", "LUMPS", "LUMPY", "LUNAR", "LUNAS", "LUNCH", "LUNES", "LUNET", "LUNGE", "LUNGI",
    "LUNGS", "LUNKS", "LUNTS", "LUPIN", "LUPUS", "LURCH", "LURED", "LURER", "LURES", "LUREX",
    "LURGI", "LURGY", "LURID", "LURKS", "LURRY", "LURVE", "LUSER", "LUSHY", "LUSKS", "LUSTS",
    "LUSTY", "LUSUS", "LUTEA", "LUTED", "LUTER", "LUTES", "LUVVY", "LUXES", "LWEIS", "LYAMS",
    "LYARD", "LYART", "LYASE", "LYCEA", "LYCEE", "LYCRA", "LYING", "LYMES", "LYMPH", "LYNCH",
    "LYNES", "LYRES", "LYRIC", "LYSED", "LYSES", "LYSIN", "LYSIS", "LYSOL", "LYSSA", "LYTED",
    "LYTES", "LYTHE", "LYTIC", "LYTTA", "MAAED", "MAARE", "MAARS", "MABES", "MACAW", "MACED",
    "MACER", "MACES", "MACHE", "MACHI", "MACHO", "MACHS", "MACKS", "MACLE", "MACON", "MACRO",
    "MADAM", "MADGE", "MADID", "MADLY", "MADRE", "MAFIA", "MAFIC", "MAGES", "MAGGS", "MAGIC",
    "MAGMA", "MAGOT", "MAGUS", "MAHOE", "MAHUA", "MAHWA", "MAIDS", "MAIKO", "MAIKS", "MAILE",
    "MAILL", "MAILS", "MAIMS", "MAINS", "MAIRE", "MAIRS", "MAISE", "MAIST", "MAIZE", "MAJOR",
    "MAKAR", "MAKER", "MAKES", "MAKIS", "MAKOS", "MALAM", "MALAR", "MALAS", "MALAX", "MALES",
    "MALIC", "MALIK", "MALIS", "MALLS", "MALMS", "MALMY", "MALTS", "MALTY", "MALVA", "MALWA",
    "MAMAS", "MAMBA", "MAMBO", "MAMEE", "MAMEY", "MAMIE", "MAMMA", "MAMMY", "MANAS", "MANAT",
    "MANDI", "MANED", "MANEH", "MANES", "MANET", "MANGA", "MANGE", "MANGO", "MANGS", "MANGY",
    "MANIA", "MANIC", "MANIS", "MANKY", "MANLY", "MANNA", "MANOR", "MANOS", "MANSE", "MANTA",
    "MANTO", "MANTY", "MANUL", "MANUS", "MAPAU", "MAPLE", "MAQUI", "MARAE", "MARAH", "MARAS",
    "MARCH", "MARCS", "MARDY", "MARES", "MARGE", "MARGS", "MARIA", "MARID", "MARKA", "MARKS",
    "MARLE", "MARLS", "MARLY", "MARMS", "MARON", "MAROR", "MARRI", "MARRY", "MARSE", "MARSH",
    "MARTS", "MARVY", "MASAS", "MASED", "MASER", "MASES", "MASHY", "MASKS", "MASON", "MASSA",
    "MASSE", "MASSY", "MASTS", "MASTY", "MASUS", "MATAI", "MATCH", "MATED", "MATER", "MATES",
    "MATEY", "MATHS", "MATIN", "MATLO", "MATTE", "MATTS", "MATZA", "MATZO", "MAUBY", "MAUDS",
    "MAULS", "MAUND", "MAURI", "MAUTS", "MAUVE", "MAVEN", "MAVIE", "MAVIN", "MAVIS", "MAWED",
    "MAWKS", "MAWKY", "MAWRS", "MAXED", "MAXES", "MAXIM", "MAXIS", "MAYAN", "MAYAS", "MAYBE",
    "MAYED", "MAYOR", "MAYOS", "MAYST", "MAZED", "MAZER", "MAZES", "MAZEY", "MAZUT", "MBIRA",
    "MEADS", "MEALS", "MEALY", "MEANE", "MEANS", "MEANT", "MEANY", "MEARE", "MEASE", "MEATH",
    "MEATS", "MEATY", "MEBOS", "MECCA", "MECKS", "MEDAL", "MEDIA", "MEDIC", "MEDII", "MEDLE",
    "MEEDS", "MEERS", "MEETS", "MEFFS", "MEINS", "MEINT", "MEINY", "MEITH", "MEKKA", "MELAS",
    "MELDS", "MELEE", "MELIC", "MELIK", "MELLS", "MELON", "MELTS", "MELTY", "MEMES", "MEMOS",
    "MENAD", "MENDS", "MENED", "MENES", "MENGE", "MENGS", "MENSA", "MENSE", "MENSH", "MENTA",
    "MENTO", "MENUS", "MEOUS", "MEOWS", "MERCH", "MERCS", "MERCY", "MERDE", "MERED", "MEREL",
    "MERER", "MERES", "MERGE", "MERIL", "MERIS", "MERIT", "MERKS", "MERLE", "MERLS", "MERRY",
    "MERSE", "MESAL", "MESAS", "MESEL", "MESES", "MESHY", "MESIC", "MESNE", "MESON", "MESSY",
    "MESTO", "METAL", "METED", "METER", "METES", "METHO", "METHS", "METIC", "METIF", "METIS",
    "METOL", "METRE", "METRO", "MEUSE", "MEVED", "MEVES", "MEWED", "MEWLS", "MEYNT", "MEZES",
    "MEZZE", "MEZZO", "MHORR", "MIAOU", "MIAOW", "MIASM", "MIAUL", "MICAS", "MICHE", "MICHT",
    "MICKS", "MICKY", "MICOS", "MICRA", "MICRO", "MIDDY", "MIDGE", "MIDGY", "MIDIS", "MIDST",
    "MIENS", "MIEVE", "MIFFS", "MIFFY", "MIFTY", "MIGGS", "MIGHT", "MIHIS", "MIKED", "MIKES",
    "MIKRA", "MILCH", "MILDS", "MILER", "MILES", "MILIA", "MILKO", "MILKS", "MILKY", "MILLE",
    "MILLS", "MILOR", "MILOS", "MILPA", "MILTS", "MILTY", "MILTZ", "MIMED", "MIMEO", "MIMER",
    "MIMES", "MIMIC", "MIMSY", "MINAE", "MINAR", "MINAS", "MINCE", "MINCY", "MINDS", "MINED",
    "MINER", "MINES", "MINGE", "MINGS", "MINGY", "MINIM", "MINIS", "MINKE", "MINKS", "MINNY",
    "MINOR", "MINOS", "MINTS", "MINTY", "MINUS", "MIRED", "MIRES", "MIREX", "MIRIN", "MIRKS",
    "MIRKY", "MIRLY", "MIRTH", "MIRVS", "MIRZA", "MISCH", "MISDO", "MISER", "MISES", "MISGO",
    "MISOS", "MISSA", "MISSY", "MISTS", "MISTY", "MITCH", "MITER", "MITES", "MITIS", "MITRE",
    "MITTS", "MIXED", "MIXEN", "MIXER", "MIXES", "MIXTE", "MIXUP", "MIZEN", "MIZZY", "MNEME",
    "MOANS", "MOATS", "MOBBY", "MOBES", "MOBIE", "MOBLE", "MOCHA", "MOCHS", "MOCHY", "MOCKS",
    "MODAL", "MODEL", "MODEM", "MODER", "MODES", "MODGE", "MODII", "MODUS", "MOERS", "MOFOS",
    "MOGGY", "MOGUL", "MOHEL", "MOHRS", "MOHUA", "MOHUR", "MOILS", "MOIRA", "MOIRE", "MOIST",
    "MOITS", "MOJOS", "MOKES", "MOKIS", "MOKOS", "MOLAL", "MOLAR", "MOLAS", "MOLDS", "MOLDY",
    "MOLES", "MOLLA", "MOLLS", "MOLLY", "MOLTO", "MOLTS", "MOMES", "MOMMA", "MOMMY", "MOMUS",
    "MONAD", "MONAL", "MONAS", "MONDE", "MONDO", "MONER", "MONEY", "MONGO", "MONGS", "MONIE",
    "MONKS", "MONOS", "MONTE", "MONTH", "MONTY", "MOOCH", "MOODS", "MOODY", "MOOED", "MOOKS",
    "MOOLA", "MOOLI", "MOOLS", "MOOLY", "MOONS", "MOONY", "MOOPS", "MOORS", "MOORY", "MOOSE",
    "MOOTS", "MOOVE", "MOPED", "MOPER", "MOPES", "MOPEY", "MOPPY", "MOPSY", "MOPUS", "MORAE",
    "MORAL", "MORAS", "MORAT", "MORAY", "MOREL", "MORES", "MORIA", "MORNE", "MORNS", "MORON",
    "MORPH", "MORRA", "MORRO", "MORSE", "MORTS", "MOSED", "MOSES", "MOSEY", "MOSKS", "MOSSO",
    "MOSSY", "MOSTE", "MOSTS", "MOTED", "MOTEL", "MOTEN", "MOTES", "MOTET", "MOTEY", "MOTHS",
    "MOTHY", "MOTIF", "MOTIS", "MOTOR", "MOTTE", "MOTTO", "MOTTS", "MOTTY", "MOTUS", "MOTZA",
    "MOUCH", "MOUES", "MOULD", "MOULS", "MOULT", "MOUND", "MOUNT", "MOUPS", "MOURN", "MOUSE",
    "MOUST", "MOUSY", "MOUTH", "MOVED", "MOVER", "MOVES", "MOVIE", "MOWAS", "MOWED", "MOWER",
    "MOWRA", "MOXAS", "MOXIE", "MOYAS", "MOYLE", "MOYLS", "MOZED", "MOZES", "MOZOS", "MPRET",
    "MUCHO", "MUCIC", "MUCID", "MUCIN", "MUCKS", "MUCKY", "MUCOR", "MUCRO", "MUCUS", "MUDDY",
    "MUDGE", "MUDIR", "MUDRA", "MUFFS", "MUFTI", "MUGGA", "MUGGS", "MUGGY", "MUHLY", "MUIDS",
    "MUILS", "MUIRS", "MUIST", "MUJIK", "MULCH", "MULCT", "MULED", "MULES", "MULEY", "MULGA",
    "MULLA", "MULLS", "MULSE", "MULSH", "MUMMS", "MUMMY", "MUMPS", "MUMSY", "MUMUS", "MUNCH",
    "MUNGA", "MUNGO", "MUNGS", "MUNIS", "MUNTS", "MUNTU", "MUONS", "MURAL", "MURAS", "MURED",
    "MURES", "MUREX", "MURID", "MURKS", "MURKY", "MURLS", "MURLY", "MURRA", "MURRE", "MURRI",
    "MURRS", "MURRY", "MURTI", "MURVA", "MUSAR", "MUSCA", "MUSED", "MUSER", "MUSES", "MUSET",
    "MUSHA", "MUSHY", "MUSIC", "MUSIT", "MUSKS", "MUSKY", "MUSOS", "MUSSE", "MUSSY", "MUSTH",
    "MUSTS", "MUSTY", "MUTCH", "MUTED", "MUTER", "MUTES", "MUTHA", "MUTIS", "MUTON", "MUTTS",
    "MUXED", "MUXES", "MUZZY", "MVULE", "MYALL", "MYLAR", "MYNAH", "MYNAS", "MYOID", "MYOMA",
    "MYOPE", "MYOPS", "MYOPY", "MYRRH", "MYSID", "MYTHI", "MYTHS", "MYTHY", "MYXOS", "MZEES",
    "NAAMS", "NAANS", "NABES", "NABIS", "NABKS", "NABLA", "NABOB", "NACHE", "NACHO", "NACRE",
    "NADAS", "NADIR", "NAEVE", "NAEVI", "NAFFS", "NAGAS", "NAGGY", "NAGOR", "NAHAL", "NAIAD",
    "NAIFS", "NAIKS", "NAILS", "NAIRA", "NAIRU", "NAIVE", "NAKED", "NAKER", "NAKFA", "NALAS",
    "NALED", "NALLA", "NAMED", "NAMER", "NAMES", "NAMMA", "NANAS", "NANCE", "NANCY", "NANDU",
    "NANNA", "NANNY", "NANUA", "NAPAS", "NAPED", "NAPES", "NAPOO", "NAPPA", "NAPPE", "NAPPY",
    "NARAS", "NARCO", "NARCS", "NARDS", "NARES", "NARIC", "NARIS", "NARKS", "NARKY", "NARRE",
    "NASAL", "NASHI", "NASTY", "NATAL", "NATCH", "NATES", "NATIS", "NATTY", "NAUCH", "NAUNT",
    "NAVAL", "NAVAR", "NAVEL", "NAVES", "NAVEW", "NAVVY", "NAWAB", "NAZES", "NAZIR", "NAZIS",
    "NEAFE", "NEALS", "NEAPS", "NEARS", "NEATH", "NEATS", "NEBEK", "NEBEL", "NECKS", "NEDDY",
    "NEEDS", "NEEDY", "NEELD", "NEELE", "NEEMB", "NEEMS", "NEEPS", "NEESE", "NEEZE", "NEGRO",
    "NEGUS", "NEIFS", "NEIGH", "NEIST", "NEIVE", "NELIS", "NELLY", "NEMAS", "NEMNS", "NEMPT",
    "NENES", "NEONS", "NEPER", "NEPIT", "NERAL", "NERDS", "NERDY", "NERKA", "NERKS", "NEROL",
    "NERTS", "NERTZ", "NERVE", "NERVY", "NESTS", "NETES", "NETOP", "NETTS", "NETTY", "NEUKS",
    "NEUME", "NEUMS", "NEVEL", "NEVER", "NEVES", "NEVUS", "NEWED", "NEWEL", "NEWER", "NEWIE",
    "NEWLY", "NEWSY", "NEWTS", "NEXTS", "NEXUS", "NGAIO", "NGANA", "NGATI", "NGOMA", "NGWEE",
    "NICAD", "NICER", "NICHE", "NICHT", "NICKS", "NICKY", "NICOL", "NIDAL", "NIDED", "NIDES",
    "NIDOR", "NIDUS", "NIECE", "NIEFS", "NIEVE", "NIFES", "NIFFS", "NIFFY", "NIFTY", "NIGER",
    "NIGHS", "NIGHT", "NIHIL", "NIKAU", "NILLS", "NIMBI", "NIMBS", "NIMPS", "NINES", "NINJA",
    "NINNY", "NINON", "NINTH", "NIPAS", "NIPPY", "NIQAB", "NIRLS", "NIRLY", "NISEI", "NISSE",
    "NISUS", "NITER", "NITES", "NITID", "NITON", "NITRE", "NITRO", "NITRY", "NITTY", "NIVAL",
    "NIXED", "NIXER", "NIXES", "NIXIE", "NIZAM", "NKOSI", "NOAHS", "NOBBY", "NOBLE", "NOBLY",
    "NOCKS", "NODAL", "NODDY", "NODES", "NODUS", "NOELS", "NOGGS", "NOHOW", "NOILS", "NOILY",
    "NOINT", "NOIRS", "NOISE", "NOISY", "NOLES", "NOLLS", "NOLOS", "NOMAD", "NOMAS", "NOMEN",
    "NOMES", "NOMIC", "NOMOI", "NOMOS", "NONAS", "NONCE", "NONES", "NONET", "NONGS", "NONIS",
    "NONNY", "NONYL", "NOOIT", "NOOKS", "NOOKY", "NOONS", "NOOPS", "NOOSE", "NOPAL", "NORIA",
    "NORIS", "NORKS", "NORMA", "NORMS", "NORTH", "NOSED", "NOSER", "NOSES", "NOSEY", "NOTAL",
    "NOTCH", "NOTED", "NOTER", "NOTES", "NOTUM", "NOULD", "NOULE", "NOULS", "NOUNS", "NOUNY",
    "NOUPS", "NOVAE", "NOVAS", "NOVEL", "NOVUM", "NOWAY", "NOWED", "NOWLS", "NOWTS", "NOWTY",
    "NOXAL", "NOXES", "NOYAU", "NOYED", "NOYES", "NUBBY", "NUBIA", "NUCHA", "NUDDY", "NUDER",
    "NUDES", "NUDGE", "NUDIE", "NUDZH", "NUFFS", "NUGAE", "NUKED", "NUKES", "NULLA", "NULLS",
    "NUMBS", "NUMEN", "NUNNY", "NURDS", "NURDY", "NURLS", "NURRS", "NURSE", "NUTSO", "NUTSY",
    "NUTTY", "NYAFF", "NYALA", "NYING", "NYLON", "NYMPH", "NYSSA", "OAKED", "OAKEN", "OAKER",
    "OAKUM", "OARED", "OASES", "OASIS", "OASTS", "OATEN", "OATER", "OATHS", "OAVES", "OBANG",
    "OBEAH", "OBELI", "OBESE", "OBEYS", "OBIAS", "OBIED", "OBIIT", "OBITS", "OBJET", "OBOES",
    "OBOLE", "OBOLI", "OBOLS", "OCCAM", "OCCUR", "OCEAN", "OCHER", "OCHES", "OCHRE", "OCHRY",
    "OCKER", "OCREA", "OCTAD", "OCTAL", "OCTAN", "OCTAS", "OCTET", "OCTYL", "OCULI", "ODAHS",
    "ODALS", "ODDER", "ODDLY", "ODEON", "ODEUM", "ODISM", "ODIST", "ODIUM", "ODORS", "ODOUR",
    "ODSOS", "ODYLE", "ODYLS", "OFAYS", "OFFAL", "OFFED", "OFFER", "OFLAG", "OFTEN", "OFTER",
    "OGAMS", "OGEES", "OGGIN", "OGHAM", "OGIVE", "OGLED", "OGLER", "OGLES", "OGMIC", "OGRES",
    "OHIAS", "OHING", "OHMIC", "OHONE", "OIDIA", "OILED", "OILER", "OINKS", "OINTS", "OJIME",
    "OKAPI", "OKAYS", "OKEHS", "OKRAS", "OKTAS", "OLDEN", "OLDER", "OLDIE", "OLEIC", "OLEIN",
    "OLENT", "OLEOS", "OLEUM", "OLIOS", "OLIVE", "OLLAS", "OLLAV", "OLLER", "OLLIE", "OLOGY",
    "OLPAE", "OLPES", "OMASA", "OMBER", "OMBRE", "OMBUS", "OMEGA", "OMENS", "OMERS", "OMITS",
    "OMLAH", "OMOVS", "OMRAH", "ONCER", "ONCES", "ONCET", "ONCUS", "ONELY", "ONERS", "ONERY",
    "ONION", "ONIUM", "ONKUS", "ONLAY", "ONNED", "ONSET", "ONTIC", "OOBIT", "OOHED", "OOMPH",
    "OONTS", "OOPED", "OORIE", "OOSES", "OOTID", "OOZED", "OOZES", "OPAHS", "OPALS", "OPENS",
    "OPEPE", "OPERA", "OPINE", "OPING", "OPIUM", "OPPOS", "OPSIN", "OPTED", "OPTER", "OPTIC",
    "ORACH", "ORACY", "ORALS", "ORANG", "ORANT", "ORATE", "ORBED", "ORBIT", "ORCAS", "ORCIN",
    "ORDER", "ORDOS", "OREAD", "ORFES", "ORGAN", "ORGIA", "ORGIC", "ORGUE", "ORIBI", "ORIEL",
    "ORIXA", "ORLES", "ORLON", "ORLOP", "ORMER", "ORNIS", "ORPIN", "ORRIS", "ORTHO", "ORVAL",
    "ORZOS", "OSCAR", "OSHAC", "OSIER", "OSMIC", "OSMOL", "OSSIA", "OSTIA", "OTAKU", "OTARY",
    "OTHER", "OTTAR", "OTTER", "OTTOS", "OUBIT", "OUCHT", "OUGHT", "OUIJA", "OULKS", "OUMAS",
    "OUNCE", "OUNDY", "OUPAS", "OUPED", "OUPHE", "OUPHS", "OURIE", "OUSEL", "OUSTS", "OUTBY",
    "OUTDO", "OUTED", "OUTER", "OUTGO", "OUTRE", "OUTRO", "OUZEL", "OUZOS", "OVALS", "OVARY",
    "OVATE", "OVELS", "OVENS", "OVERS", "OVERT", "OVINE", "OVIST", "OVOID", "OVOLI", "OVOLO",
    "OVULE", "OWCHE", "OWING", "OWLED", "OWLER", "OWLET", "OWNED", "OWNER", "OWRES", "OWRIE",
    "OWSEN", "OXBOW", "OXERS", "OXEYE", "OXIDE", "OXIDS", "OXIME", "OXIMS", "OXLIP", "OXTER",
    "OYERS", "OZEKI", "OZONE", "OZZIE", "PAALS", "PACAS", "PACED", "PACER", "PACES", "PACEY",
    "PACHA", "PACKS", "PACOS", "PACTA", "PACTS", "PADDY", "PADIS", "PADLE", "PADMA", "PADRE",
    "PADRI", "PAEAN", "PAEON", "PAGAN", "PAGED", "PAGER", "PAGES", "PAGLE", "PAGOD", "PAGRI",
    "PAIKS", "PAILS", "PAINS", "PAINT", "PAIRE", "PAIRS", "PAISA", "PAISE", "PAKKA", "PALAS",
    "PALAY", "PALEA", "PALED", "PALER", "PALES", "PALET", "PALKI", "PALLA", "PALLS", "PALLY",
    "PALMS", "PALMY", "PALPI", "PALPS", "PALSY", "PAMPA", "PANAX", "PANCE", "PANDA", "PANDS",
    "PANDY", "PANED", "PANEL", "PANES", "PANGA", "PANGS", "PANIC", "PANIM", "PANNE", "PANSY",
    "PANTO", "PANTS", "PANTY", "PAOLI", "PAOLO", "PAPAL", "PAPAS", "PAPAW", "PAPER", "PAPES",
    "PAPPI", "PAPPY", "PARAE", "PARAS", "PARCH", "PARDI", "PARDS", "PARDY", "PARED", "PAREO",
    "PARER", "PARES", "PAREU", "PAREV", "PARGE", "PARGO", "PARIS", "PARKA", "PARKI", "PARKS",
    "PARKY", "PARLE", "PARLY", "PAROL", "PARPS", "PARRA", "PARRS", "PARRY", "PARSE", "PARTI",
    "PARTS", "PARTY", "PARVE", "PARVO", "PASEO", "PASES", "PASHA", "PASHM", "PASPY", "PASSE",
    "PASTA", "PASTE", "PASTS", "PASTY", "PATCH", "PATED", "PATEN", "PATER", "PATES", "PATHS",
    "PATIN", "PATIO", "PATLY", "PATSY", "PATTE", "PATTY", "PATUS", "PAUAS", "PAULS", "PAUSE",
    "PAVAN", "PAVED", "PAVEN", "PAVER", "PAVES", "PAVID", "PAVIN", "PAVIS", "PAWAS", "PAWAW",
    "PAWED", "PAWER", "PAWKS", "PAWKY", "PAWLS", "PAWNS", "PAXES", "PAYED", "PAYEE", "PAYER",
    "PAYOR", "PAYSD", "PEACE", "PEACH", "PEAGE", "PEAGS", "PEAKS", "PEAKY", "PEALS", "PEANS",
    "PEARE", "PEARL", "PEARS", "PEART", "PEASE", "PEATS", "PEATY", "PEAVY", "PEAZE", "PEBAS",
    "PECAN", "PECHS", "PECKE", "PECKS", "PECKY", "PEDAL", "PEDES", "PEDRO", "PEECE", "PEEKS",
    "PEELS", "PEENS", "PEEOY", "PEEPE", "PEEPS", "PEERS", "PEERY", "PEEVE", "PEGGY", "PEGHS",
    "PEINS", "PEISE", "PEIZE", "PEKAN", "PEKES", "PEKIN", "PEKOE", "PELAS", "PELES", "PELFS",
    "PELLS", "PELMA", "PELON", "PELTA", "PELTS", "PENAL", "PENCE", "PENDS", "PENDU", "PENED",
    "PENES", "PENGO", "PENIE", "PENIS", "PENKS", "PENNA", "PENNE", "PENNI", "PENNY", "PENTS",
    "PEONS", "PEONY", "PEPLA", "PEPOS", "PEPPY", "PERAI", "PERCE", "PERCH", "PERDU", "PERDY",
    "PEREA", "PERES", "PERIL", "PERIS", "PERKS", "PERKY", "PERMS", "PERNS", "PERPS", "PERRY",
    "PERSE", "PERST", "PERTS", "PERVE", "PERVS", "PESKY", "PESOS", "PESTO", "PESTS", "PESTY",
    "PETAL", "PETAR", "PETER", "PETIT", "PETRE", "PETTI", "PETTO", "PETTY", "PEWEE", "PEWIT",
    "PEYSE", "PHAGE", "PHANG", "PHARE", "PHASE", "PHEER", "PHENE", "PHEON", "PHESE", "PHIAL",
    "PHLOX", "PHOCA", "PHOHS", "PHONE", "PHONO", "PHONS", "PHONY", "PHOTO", "PHOTS", "PHPHT",
    "PHUTS", "PHYLA", "PHYLE", "PIANO", "PIANS", "PIBAL", "PICAL", "PICAS", "PICCY", "PICKS",
    "PICKY", "PICOT", "PICRA", "PICUL", "PIECE", "PIEND", "PIERS", "PIERT", "PIETA", "PIETS",
    "PIETY", "PIEZO", "PIGGY", "PIGHT", "PIGMY", "PIING", "PIKAS", "PIKAU", "PIKED", "PIKER",
    "PIKES", "PIKEY", "PIKIS", "PIKUL", "PILAF", "PILAO", "PILAR", "PILAU", "PILAW", "PILCH",
    "PILEA", "PILED", "PILEI", "PILER", "PILES", "PILIS", "PILLS", "PILOT", "PILOW", "PILUM",
    "PILUS", "PIMAS", "PIMPS", "PINAS", "PINCH", "PINED", "PINES", "PINEY", "PINGO", "PINGS",
    "PINKO", "PINKS", "PINKY", "PINNA", "PINNY", "PINON", "PINOT", "PINTA", "PINTO", "PINTS",
    "PINUP", "PIONS", "PIONY", "PIOUS", "PIOYE", "PIOYS", "PIPAL", "PIPAS", "PIPED", "PIPER",
    "PIPES", "PIPET", "PIPIS", "PIPIT", "PIPPY", "PIPUL", "PIQUE", "PIRAI", "PIRLS", "PIRNS",
    "PIROG", "PISCO", "PISES", "PISKY", "PISOS", "PISTE", "PITAS", "PITCH", "PITHS", "PITHY",
    "PITON", "PITTA", "PIUMS", "PIVOT", "PIXEL", "PIXES", "PIXIE", "PIZED", "PIZES", "PIZZA",
    "PLAAS", "PLACE", "PLACK", "PLAGE", "PLAID", "PLAIN", "PLAIT", "PLANE", "PLANK", "PLANS",
    "PLANT", "PLAPS", "PLASH", "PLASM", "PLAST", "PLATE", "PLATS", "PLATY", "PLAYA", "PLAYS",
    "PLAZA", "PLEAD", "PLEAS", "PLEAT", "PLEBE", "PLEBS", "PLENA", "PLEON", "PLESH", "PLEWS",
    "PLICA", "PLIED", "PLIER", "PLIES", "PLIMS", "PLING", "PLINK", "PLOAT", "PLODS", "PLONG",
    "PLONK", "PLOOK", "PLOPS", "PLOTS", "PLOTZ", "PLOUK", "PLOWS", "PLOYS", "PLUCK", "PLUES",
    "PLUFF", "PLUGS", "PLUMB", "PLUME", "PLUMP", "PLUMS", "PLUMY", "PLUNK", "PLUSH", "PLYER",
    "POACH", "POAKA", "POAKE", "POBOY", "POCKS", "POCKY", "PODAL", "PODDY", "PODEX", "PODGE",
    "PODGY", "PODIA", "POEMS", "POEPS", "POESY", "POETS", "POGEY", "POGGE", "POGOS", "POILU",
    "POIND", "POINT", "POISE", "POKAL", "POKED", "POKER", "POKES", "POKEY", "POKIE", "POLAR",
    "POLED", "POLER", "POLES", "POLEY", "POLIO", "POLIS", "POLJE", "POLKA", "POLKS", "POLLS",
    "POLLY", "POLOS", "POLTS", "POLYP", "POLYS", "POMBE", "POMES", "POMMY", "POMOS", "POMPS",
    "PONCE", "PONCY", "PONDS", "PONES", "PONEY", "PONGA", "PONGO", "PONGS", "PONGY", "PONKS",
    "PONTS", "PONTY", "PONZU", "POOCH", "POODS", "POOED", "POOFS", "POOFY", "POOHS", "POOJA",
    "POOKA", "POOKS", "POOLS", "POONS", "POOPS", "POORI", "POORT", "POOTS", "POOVE", "POOVY",
    "POPES", "POPPA", "POPPY", "POPSY", "PORAE", "PORAL", "PORCH", "PORED", "PORER", "PORES",
    "PORGE", "PORGY", "PORKS", "PORKY", "PORNO", "PORNS", "PORNY", "PORTA", "PORTS", "PORTY",
    "POSED", "POSER", "POSES", "POSEY", "POSHO", "POSIT", "POSSE", "POSTS", "POTAE", "POTCH",
    "POTED", "POTES", "POTIN", "POTOO", "POTSY", "POTTO", "POTTS", "POTTY", "POUCH", "POUFF",
    "POUFS", "POUKE", "POUKS", "POULE", "POULP", "POULT", "POUND", "POUPE", "POUPT", "POURS",
    "POUTS", "POUTY", "POWAN", "POWER", "POWIN", "POWND", "POWNS", "POWNY", "POWRE", "POXED",
    "POXES", "POYNT", "POYOU", "POYSE", "POZZY", "PRAAM", "PRADS", "PRAHU", "PRAMS", "PRANA",
    "PRANG", "PRANK", "PRAOS", "PRASE", "PRATE", "PRATS", "PRATT", "PRATY", "PRAUS", "PRAWN",
    "PRAYS", "PREDY", "PREED", "PREEN", "PREES", "PREIF", "PREMS", "PREMY", "PRENT", "PREOP",
    "PREPS", "PRESA", "PRESE", "PRESS", "PREST", "PREVE", "PREXY", "PREYS", "PRIAL", "PRICE",
    "PRICK", "PRICY", "PRIDE", "PRIED", "PRIEF", "PRIER", "PRIES", "PRIGS", "PRILL", "PRIMA",
    "PRIME", "PRIMI", "PRIMO", "PRIMP", "PRIMS", "PRIMY", "PRINK", "PRINT", "PRION", "PRIOR",
    "PRISE", "PRISM", "PRISS", "PRIVY", "PRIZE", "PROAS", "PROBE", "PROBS", "PRODS", "PROEM",
    "PROFS", "PROGS", "PROIN", "PROKE", "PROLE", "PROLL", "PROMO", "PROMS", "PRONE", "PRONG",
    "PRONK", "PROOF", "PROPS", "PRORE", "PROSE", "PROSO", "PROSS", "PROST", "PROSY", "PROUD",
    "PROUL", "PROVE", "PROWL", "PROWS", "PROXY", "PROYN", "PRUDE", "PRUNE", "PRUNT", "PRUTA",
    "PRYER", "PRYSE", "PSALM", "PSEUD", "PSHAW", "PSION", "PSOAE", "PSOAI", "PSOAS", "PSORA",
    "PSYCH", "PSYOP", "PUBES", "PUBIC", "PUBIS", "PUCAN", "PUCER", "PUCES", "PUCKA", "PUCKS",
    "PUDDY", "PUDGE", "PUDGY", "PUDIC", "PUDOR", "PUDSY", "PUDUS", "PUERS", "PUFFS", "PUFFY",
    "PUGGY", "PUGIL", "PUHAS", "PUJAH", "PUJAS", "PUKED", "PUKER", "PUKES", "PUKKA", "PUKUS",
    "PULAO", "PULAS", "PULED", "PULER", "PULES", "PULIK", "PULIS", "PULKA", "PULKS", "PULLI",
    "PULLS", "PULMO", "PULPS", "PULPY", "PULSE", "PULUS", "PUMAS", "PUMIE", "PUMPS", "PUNAS",
    "PUNCE", "PUNCH", "PUNGA", "PUNGS", "PUNJI", "PUNKA", "PUNKS", "PUNKY", "PUNNY", "PUNTO",
    "PUNTS", "PUNTY", "PUPAE", "PUPAL", "PUPAS", "PUPIL", "PUPPY", "PUPUS", "PURDA", "PURED",
    "PUREE", "PURER", "PURES", "PURGE", "PURIM", "PURIN", "PURIS", "PURLS", "PURPY", "PURRS",
    "PURSE", "PURSY", "PURTY", "PUSES", "PUSHY", "PUSLE", "PUSSY", "PUTID", "PUTON", "PUTTI",
    "PUTTO", "PUTTS", "PUTTY", "PUZEL", "PYATS", "PYETS", "PYGAL", "PYGMY", "PYINS", "PYLON",
    "PYNED", "PYNES", "PYOID", "PYOTS", "PYRAL", "PYRAN", "PYRES", "PYREX", "PYRIC", "PYROS",
    "PYXED", "PYXES", "PYXIE", "PYXIS", "PZAZZ", "QADIS", "QAIDS", "QANAT", "QIBLA", "QOPHS",
    "QORMA", "QUACK", "QUADS", "QUAFF", "QUAGS", "QUAIL", "QUAIR", "QUAIS", "QUAKE", "QUAKY",
    "QUALE", "QUALM", "QUANT", "QUARE", "QUARK", "QUART", "QUASH", "QUASI", "QUASS", "QUATE",
    "QUATS", "QUAYD", "QUAYS", "QUBIT", "QUEAN", "QUEEN", "QUEER", "QUELL", "QUEME", "QUENA",
    "QUERN", "QUERY", "QUEST", "QUEUE", "QUEYN", "QUEYS", "QUICH", "QUICK", "QUIDS", "QUIET",
    "QUIFF", "QUILL", "QUILT", "QUIMS", "QUINA", "QUINE", "QUINO", "QUINS", "QUINT", "QUIPO",
    "QUIPS", "QUIPU", "QUIRE", "QUIRK", "QUIRT", "QUIST", "QUITE", "QUITS", "QUOAD", "QUODS",
    "QUOIF", "QUOIN", "QUOIT", "QUOLL", "QUONK", "QUOPS", "QUOTA", "QUOTE", "QUOTH", "QURSH",
    "QUYTE", "RABAT", "RABBI", "RABIC", "RABID", "RABIS", "RACED", "RACER", "RACES", "RACHE",
    "RACKS", "RACON", "RADAR", "RADGE", "RADII", "RADIO", "RADIX", "RADON", "RAFFS", "RAFTS",
    "RAGAS", "RAGDE", "RAGED", "RAGEE", "RAGER", "RAGES", "RAGGA", "RAGGS", "RAGGY", "RAGIS",
    "RAHED", "RAHUI", "RAIAS", "RAIDS", "RAIKS", "RAILE", "RAILS", "RAINE", "RAINS", "RAINY",
    "RAIRD", "RAISE", "RAITA", "RAITS", "RAJAH", "RAJAS", "RAJES", "RAKED", "RAKEE", "RAKER",
    "RAKES", "RAKIS", "RAKUS", "RALES", "RALLY", "RALPH", "RAMAL", "RAMEE", "RAMEN", "RAMET",
    "RAMIE", "RAMIN", "RAMIS", "RAMMY", "RAMPS", "RAMUS", "RANAS", "RANCE", "RANCH", "RANDS",
    "RANDY", "RANEE", "RANGE", "RANGI", "RANGY", "RANID", "RANIS", "RANKE", "RANKS", "RANTS",
    "RAPED", "RAPER", "RAPES", "RAPHE", "RAPID", "RAPPE", "RARED", "RAREE", "RARER", "RARES",
    "RARKS", "RASED", "RASER", "RASES", "RASPS", "RASPY", "RASSE", "RASTA", "RATAL", "RATAN",
    "RATAS", "RATCH", "RATED", "RATEL", "RATER", "RATES", "RATHA", "RATHE", "RATHS", "RATIO",
    "RATOO", "RATOS", "RATTY", "RATUS", "RAUNS", "RAUPO", "RAVED", "RAVEL", "RAVEN", "RAVER",
    "RAVES", "RAVIN", "RAWER", "RAWIN", "RAWLY", "RAWNS", "RAXED", "RAXES", "RAYAH", "RAYAS",
    "RAYED", "RAYLE", "RAYNE", "RAYON", "RAZED", "RAZEE", "RAZER", "RAZES", "RAZOO", "RAZOR",
    "REACH", "REACT", "READD", "READS", "READY", "REAKS", "REALM", "REALO", "REALS", "REAME",
    "REAMS", "REAMY", "REANS", "REAPS", "REARM", "REARS", "REAST", "REATA", "REATE", "REAVE",
    "REBAR", "REBBE", "REBEC", "REBEL", "REBID", "REBIT", "REBOP", "REBUS", "REBUT", "REBUY",
    "RECAL", "RECAP", "RECCE", "RECCO", "RECCY", "RECIT", "RECKS", "RECON", "RECTA", "RECTI",
    "RECTO", "RECUR", "RECUT", "REDAN", "REDDS", "REDDY", "REDED", "REDES", "REDIA", "REDID",
    "REDIP", "REDLY", "REDON", "REDOS", "REDOX", "REDRY", "REDUB", "REDUX", "REDYE", "REECH",
    "REEDE", "REEDS", "REEDY", "REEFS", "REEFY", "REEKS", "REEKY", "REELS", "REENS", "REEST",
    "REEVE", "REFED", "REFEL", "REFER", "REFFO", "REFIT", "REFIX", "REFLY", "REFRY", "REGAL",
    "REGAR", "REGES", "REGGO", "REGIE", "REGMA", "REGNA", "REGOS", "REGUR", "REHAB", "REHEM",
    "REIFS", "REIFY", "REIGN", "REIKI", "REIKS", "REINK", "REINS", "REIRD", "REIST", "REIVE",
    "REJIG", "REJON", "REKED", "REKES", "REKEY", "RELAX", "RELAY", "RELET", "RELIC", "RELIE",
    "RELIT", "REMAN", "REMAP", "REMEN", "REMET", "REMEX", "REMIT", "REMIX", "RENAL", "RENAY",
    "RENDS", "RENEW", "RENEY", "RENGA", "RENIG", "RENIN", "RENNE", "RENTE", "RENTS", "REOIL",
    "REPAY", "REPEG", "REPEL", "REPIN", "REPLA", "REPLY", "REPOS", "REPOT", "REPPS", "REPRO",
    "RERAN", "RERIG", "RERUN", "RESAT", "RESAW", "RESAY", "RESEE", "RESES", "RESET", "RESEW",
    "RESID", "RESIN", "RESIT", "RESOD", "RESOW", "RESTO", "RESTS", "RESTY", "RETAG", "RETAX",
    "RETCH", "RETEM", "RETES", "RETIA", "RETIE", "RETRO", "RETRY", "REUSE", "REVEL", "REVET",
    "REVIE", "REVUE", "REWAN", "REWAX", "REWED", "REWET", "REWIN", "REWON", "REWTH", "REXES",
    "RHEAS", "RHEME", "RHEUM", "RHIES", "RHIME", "RHINE", "RHINO", "RHODY", "RHOMB", "RHONE",
    "RHUMB", "RHYME", "RHYNE", "RHYTA", "RIALS", "RIANT", "RIATA", "RIBAS", "RIBBY", "RIBES",
    "RICED", "RICER", "RICES", "RICEY", "RICHT", "RICIN", "RICKS", "RIDER", "RIDES", "RIDGE",
    "RIDGY", "RIELS", "RIEMS", "RIEVE", "RIFER", "RIFFS", "RIFLE", "RIFTE", "RIFTS", "RIFTY",
    "RIGGS", "RIGHT", "RIGID", "RIGOL", "RIGOR", "RILED", "RILES", "RILEY", "RILLE", "RILLS",
    "RIMAE", "RIMED", "RIMER", "RIMES", "RIMUS", "RINDS", "RINDY", "RINES", "RINGS", "RINKS",
    "RINSE", "RIOJA", "RIOTS", "RIPED", "RIPEN", "RIPER", "RIPES", "RIPPS", "RISEN", "RISER",
    "RISES", "RISHI", "RISKS", "RISKY", "RISPS", "RISUS", "RITES", "RITTS", "RITZY", "RIVAL",
    "RIVAS", "RIVED", "RIVEL", "RIVEN", "RIVER", "RIVES", "RIVET", "RIVOS", "RIYAL", "RIZAS",
    "ROACH", "ROADS", "ROAMS", "ROANS", "ROARS", "ROARY", "ROAST", "ROATE", "ROBED", "ROBES",
    "ROBIN", "ROBLE", "ROBOT", "ROCKS", "ROCKY", "RODED", "RODEO", "RODES", "ROGER", "ROGUE",
    "ROGUY", "ROILS", "ROILY", "ROINS", "ROIST", "ROJAK", "ROJIS", "ROKED", "ROKER", "ROKES",
    "ROLAG", "ROLES", "ROLFS", "ROLLS", "ROMAL", "ROMAN", "ROMAS", "ROMEO", "ROMPS", "RONDE",
    "RONDO", "RONEO", "RONES", "RONIN", "RONNE", "RONTE", "RONTS", "ROODS", "ROOFS", "ROOFY",
    "ROOKS", "ROOKY", "ROOMS", "ROOMY", "ROONS", "ROOPS", "ROOPY", "ROOSA", "ROOSE", "ROOST",
    "ROOTS", "ROOTY", "ROPED", "ROPER", "ROPES", "ROPEY", "ROQUE", "RORAL", "RORES", "RORIC",
    "RORID", "RORIE", "RORTS", "RORTY", "ROSED", "ROSES", "ROSET", "ROSHI", "ROSIN", "ROSIT",
    "ROSTI", "ROSTS", "ROTAL", "ROTAN", "ROTAS", "ROTCH", "ROTED", "ROTES", "ROTIS", "ROTLS",
    "ROTON", "ROTOR", "ROTOS", "ROTTE", "ROUEN", "ROUES", "ROUGE", "ROUGH", "ROULE", "ROULS",
    "ROUMS", "ROUND", "ROUPS", "ROUPY", "ROUSE", "ROUST", "ROUTE", "ROUTH", "ROUTS", "ROVED",
    "ROVEN", "ROVER", "ROVES", "ROWAN", "ROWDY", "ROWED", "ROWEL", "ROWEN", "ROWER", "ROWME",
    "ROWND", "ROWTH", "ROWTS", "ROYAL", "ROYNE", "ROYST", "ROZET", "ROZIT", "RUANA", "RUBAI",
    "RUBBY", "RUBEL", "RUBES", "RUBIN", "RUBLE", "RUBUS", "RUCHE", "RUCKS", "RUDAS", "RUDDS",
    "RUDDY", "RUDER", "RUDES", "RUDIE", "RUERS", "RUFFE", "RUFFS", "RUGAE", "RUGAL", "RUGBY",
    "RUGGY", "RUING", "RUINS", "RUKHS", "RULED", "RULER", "RULES", "RUMAL", "RUMBA", "RUMBO",
    "RUMEN", "RUMES", "RUMLY", "RUMMY", "RUMOR", "RUMPO", "RUMPS", "RUMPY", "RUNCH", "RUNDS",
    "RUNED", "RUNES", "RUNGS", "RUNIC", "RUNNY", "RUNTS", "RUNTY", "RUPEE", "RUPIA", "RURAL",
    "RURPS", "RURUS", "RUSAS", "RUSES", "RUSHY", "RUSKS", "RUSMA", "RUSSE", "RUSTS", "RUSTY",
    "RUTHS", "RUTIN", "RUTTY", "RYALS", "RYBAT", "RYKED", "RYKES", "RYMME", "RYNDS", "RYOTS",
    "RYPER", "SABAL", "SABED", "SABER", "SABES", "SABIN", "SABIR", "SABLE", "SABOT", "SABRA",
    "SABRE", "SACKS", "SACRA", "SADDO", "SADES", "SADHE", "SADHU", "SADIS", "SADLY", "SADOS",
    "SADZA", "SAFED", "SAFER", "SAFES", "SAGAS", "SAGER", "SAGES", "SAGGY", "SAGOS", "SAGUM",
    "SAHEB", "SAHIB", "SAICE", "SAICK", "SAICS", "SAIDS", "SAIGA", "SAILS", "SAIMS", "SAINE",
    "SAINS", "SAINT", "SAIRS", "SAIST", "SAITH", "SAJOU", "SAKAI", "SAKER", "SAKES", "SAKIA",
    "SAKIS", "SALAD", "SALAL", "SALEP", "SALES", "SALET", "SALIC", "SALIX", "SALLE", "SALLY",
    "SALMI", "SALOL", "SALON", "SALOP", "SALPA", "SALPS", "SALSA", "SALSE", "SALTO", "SALTS",
    "SALTY", "SALUE", "SALVE", "SALVO", "SAMAN", "SAMAS", "SAMBA", "SAMBO", "SAMEK", "SAMEL",
    "SAMEN", "SAMES", "SAMEY", "SAMFU", "SAMMY", "SAMPI", "SAMPS", "SANDS", "SANDY", "SANED",
    "SANER", "SANES", "SANGA", "SANGH", "SANGO", "SANGS", "SANKO", "SANSA", "SANTO", "SANTS",
    "SAPAN", "SAPID", "SAPOR", "SAPPY", "SARAN", "SARDS", "SARED", "SAREE", "SARGE", "SARGO",
    "SARIN", "SARIS", "SARKS", "SARKY", "SAROD", "SAROS", "SARUS", "SASER", "SASIN", "SASSE",
    "SASSY", "SATAI", "SATAY", "SATED", "SATEM", "SATES", "SATIN", "SATIS", "SATYR", "SAUBA",
    "SAUCE", "SAUCH", "SAUCY", "SAUGH", "SAULS", "SAULT", "SAUNA", "SAUNT", "SAURY", "SAUTE",
    "SAUTS", "SAVED", "SAVER", "SAVES", "SAVEY", "SAVIN", "SAVOR", "SAVOY", "SAVVY", "SAWAH",
    "SAWED", "SAWER", "SAXES", "SAYED", "SAYER", "SAYID", "SAYNE", "SAYON", "SAYST", "SAZES",
    "SCABS", "SCADS", "SCAFF", "SCAGS", "SCAIL", "SCALA", "SCALD", "SCALE", "SCALL", "SCALP",
    "SCALY", "SCAMP", "SCAMS", "SCAND", "SCANS", "SCANT", "SCAPA", "SCAPE", "SCAPI", "SCARE",
    "SCARF", "SCARP", "SCARS", "SCART", "SCARY", "SCATH", "SCATS", "SCATT", "SCAUD", "SCAUP",
    "SCAUR", "SCAWS", "SCEAT", "SCENA", "SCEND", "SCENE", "SCENT", "SCHAV", "SCHMO", "SCHUL",
    "SCHWA", "SCION", "SCLIM", "SCODY", "SCOFF", "SCOGS", "SCOLD", "SCONE", "SCOOG", "SCOOP",
    "SCOOT", "SCOPA", "SCOPE", "SCOPS", "SCORE", "SCORN", "SCOTS", "SCOUG", "SCOUP", "SCOUR",
    "SCOUT", "SCOWL", "SCOWP", "SCOWS", "SCRAB", "SCRAE", "SCRAG", "SCRAM", "SCRAN", "SCRAP",
    "SCRAT", "SCRAW", "SCRAY", "SCREE", "SCREW", "SCRIM", "SCRIP", "SCROD", "SCROG", "SCROW",
    "SCRUB", "SCRUM", "SCUBA", "SCUDI", "SCUDO", "SCUDS", "SCUFF", "SCUFT", "SCUGS", "SCULK",
    "SCULL", "SCULP", "SCULS", "SCUMS", "SCUPS", "SCURF", "SCURS", "SCUSE", "SCUTA", "SCUTE",
    "SCUTS", "SCUZZ", "SCYES", "SDAYN", "SDEIN", "SEALS", "SEAME", "SEAMS", "SEAMY", "SEANS",
    "SEARE", "SEARS", "SEASE", "SEATS", "SEAZE", "SEBUM", "SECCO", "SECHS", "SECTS", "SEDAN",
    "SEDER", "SEDES", "SEDGE", "SEDGY", "SEDUM", "SEEDS", "SEEDY", "SEEKS", "SEELD", "SEELS",
    "SEELY", "SEEMS", "SEEPS", "SEEPY", "SEERS", "SEFER", "SEGAR", "SEGNI", "SEGNO", "SEGOL",
    "SEGOS", "SEGUE", "SEIFS", "SEILS", "SEINE", "SEIRS", "SEISE", "SEISM", "SEITY", "SEIZE",
    "SEKOS", "SEKTS", "SELAH", "SELES", "SELFS", "SELLA", "SELLE", "SELLS", "SELVA", "SEMEE",
    "SEMEN", "SEMES", "SEMIE", "SEMIS", "SENAS", "SENDS", "SENGI", "SENNA", "SENOR", "SENSA",
    "SENSE", "SENSI", "SENTE", "SENTI", "SENTS", "SENVY", "SENZA", "SEPAD", "SEPAL", "SEPIA",
    "SEPIC", "SEPOY", "SEPTA", "SEPTS", "SERAC", "SERAI", "SERAL", "SERED", "SERER", "SERES",
    "SERFS", "SERGE", "SERIC", "SERIF", "SERIN", "SERKS", "SERON", "SEROW", "SERRA", "SERRE",
    "SERRS", "SERRY", "SERUM", "SERVE", "SERVO", "SESEY", "SESSA", "SETAE", "SETAL", "SETON",
    "SETTS", "SETUP", "SEVEN", "SEVER", "SEWAN", "SEWAR", "SEWED", "SEWEL", "SEWEN", "SEWER",
    "SEWIN", "SEXED", "SEXER", "SEXES", "SEXTO", "SEXTS", "SEYEN", "SHACK", "SHADE", "SHADS",
    "SHADY", "SHAFT", "SHAGS", "SHAHS", "SHAKE", "SHAKO", "SHAKT", "SHAKY", "SHALE", "SHALL",
    "SHALM", "SHALT", "SHALY", "SHAMA", "SHAME", "SHAMS", "SHAND", "SHANK", "SHANS", "SHAPE",
    "SHAPS", "SHARD", "SHARE", "SHARK", "SHARN", "SHARP", "SHASH", "SHAUL", "SHAVE", "SHAWL",
    "SHAWM", "SHAWN", "SHAWS", "SHAYA", "SHAYS", "SHCHI", "SHEAF", "SHEAL", "SHEAR", "SHEAS",
    "SHEDS", "SHEEL", "SHEEN", "SHEEP", "SHEER", "SHEET", "SHEIK", "SHELF", "SHELL", "SHEND",
    "SHENT", "SHEOL", "SHERD", "SHERE", "SHETS", "SHEVA", "SHEWN", "SHEWS", "SHIAI", "SHIED",
    "SHIEL", "SHIER", "SHIES", "SHIFT", "SHILL", "SHILY", "SHIMS", "SHINE", "SHINS", "SHINY",
    "SHIPS", "SHIRE", "SHIRK", "SHIRR", "SHIRS", "SHIRT", "SHISH", "SHISO", "SHIST", "SHITE",
    "SHITS", "SHIUR", "SHIVA", "SHIVE", "SHIVS", "SHLEP", "SHLUB", "SHMEK", "SHOAL", "SHOAT",
    "SHOCK", "SHOED", "SHOER", "SHOES", "SHOGI", "SHOGS", "SHOJI", "SHOLA", "SHONE", "SHOOK",
    "SHOOL", "SHOON", "SHOOS", "SHOOT", "SHOPE", "SHOPS", "SHORE", "SHORL", "SHORN", "SHORT",
    "SHOTE", "SHOTS", "SHOTT", "SHOUT", "SHOVE", "SHOWD", "SHOWN", "SHOWS", "SHOWY", "SHOYU",
    "SHRED", "SHREW", "SHRIS", "SHROW", "SHRUB", "SHRUG", "SHTIK", "SHTUM", "SHTUP", "SHUCK",
    "SHULE", "SHULN", "SHULS", "SHUNS", "SHUNT", "SHURA", "SHUSH", "SHUTE", "SHUTS", "SHWAS",
    "SHYER", "SHYLY", "SIALS", "SIBBS", "SIBYL", "SICES", "SICHT", "SICKO", "SICKS", "SIDAS",
    "SIDED", "SIDER", "SIDES", "SIDHA", "SIDHE", "SIDLE", "SIEGE", "SIELD", "SIENS", "SIENT",
    "SIETH", "SIEUR", "SIEVE", "SIFTS", "SIGHS", "SIGHT", "SIGIL", "SIGLA", "SIGMA", "SIGNA",
    "SIGNS", "SIJOS", "SIKAS", "SIKER", "SIKES", "SILDS", "SILED", "SILEN", "SILER", "SILES",
    "SILEX", "SILKS", "SILKY", "SILLS", "SILLY", "SILOS", "SILTS", "SILTY", "SILVA", "SIMAR",
    "SIMAS", "SIMBA", "SIMIS", "SIMPS", "SIMUL", "SINCE", "SINDS", "SINED", "SINES", "SINEW",
    "SINGE", "SINGS", "SINHS", "SINKS", "SINKY", "SINUS", "SIPED", "SIPES", "SIPPY", "SIRED",
    "SIREE", "SIREN", "SIRES", "SIRIH", "SIRIS", "SIROC", "SIRRA", "SIRUP", "SISAL", "SISES",
    "SISSY", "SISTS", "SITAR", "SITED", "SITES", "SITHE", "SITKA", "SITUP", "SITUS", "SIVER",
    "SIXER", "SIXES", "SIXMO", "SIXTE", "SIXTH", "SIXTY", "SIZAR", "SIZED", "SIZEL", "SIZER",
    "SIZES", "SKAGS", "SKAIL", "SKALD", "SKANK", "SKART", "SKATE", "SKATS", "SKATT", "SKAWS",
    "SKEAN", "SKEAR", "SKEED", "SKEEF", "SKEEN", "SKEER", "SKEES", "SKEET", "SKEGG", "SKEGS",
    "SKEIN", "SKELF", "SKELL", "SKELM", "SKELP", "SKENE", "SKENS", "SKEOS", "SKEPS", "SKERS",
    "SKETS", "SKEWS", "SKIDS", "SKIED", "SKIER", "SKIES", "SKIEY", "SKIFF", "SKILL", "SKIMO",
    "SKIMP", "SKIMS", "SKINK", "SKINS", "SKINT", "SKIOS", "SKIPS", "SKIRL", "SKIRR", "SKIRT",
    "SKITE", "SKITS", "SKIVE", "SKIVY", "SKLIM", "SKOAL", "SKOFF", "SKOLS", "SKOOL", "SKORT",
    "SKOSH", "SKRAN", "SKRIK", "SKUAS", "SKUGS", "SKULK", "SKULL", "SKUNK", "SKYED", "SKYER",
    "SKYEY", "SKYFS", "SKYRE", "SKYRS", "SKYTE", "SLABS", "SLACK", "SLADE", "SLAES", "SLAGS",
    "SLAID", "SLAIN", "SLAKE", "SLAMS", "SLANE", "SLANG", "SLANK", "SLANT", "SLAPS", "SLART",
    "SLASH", "SLATE", "SLATS", "SLATY", "SLAVE", "SLAWS", "SLAYS", "SLEDS", "SLEEK", "SLEEP",
    "SLEER", "SLEET", "SLEPT", "SLEWS", "SLEYS", "SLICE", "SLICK", "SLIDE", "SLIER", "SLILY",
    "SLIME", "SLIMS", "SLIMY", "SLING", "SLINK", "SLIPE", "SLIPS", "SLIPT", "SLISH", "SLITS",
    "SLIVE", "SLOAN", "SLOBS", "SLOES", "SLOGS", "SLOID", "SLOJD", "SLOOM", "SLOOP", "SLOOT",
    "SLOPE", "SLOPS", "SLOPY", "SLORM", "SLOSH", "SLOTH", "SLOTS", "SLOVE", "SLOWS", "SLOYD",
    "SLUBB", "SLUBS", "SLUED", "SLUES", "SLUFF", "SLUGS", "SLUIT", "SLUMP", "SLUMS", "SLUNG",
    "SLUNK", "SLURB", "SLURP", "SLURS", "SLUSE", "SLUSH", "SLUTS", "SLYER", "SLYLY", "SLYPE",
    "SMAAK", "SMACK", "SMAIK", "SMALL", "SMALM", "SMALT", "SMARM", "SMART", "SMASH", "SMAZE",
    "SMEAR", "SMEEK", "SMEES", "SMELL", "SMELT", "SMERK", "SMEWS", "SMILE", "SMIRK", "SMIRR",
    "SMIRS", "SMITE", "SMITH", "SMITS", "SMOCK", "SMOGS", "SMOKE", "SMOKO", "SMOKY", "SMOLT",
    "SMOOR", "SMOOT", "SMORE", "SMOTE", "SMOUT", "SMOWT", "SMUGS", "SMURS", "SMUSH", "SMUTS",
    "SNABS", "SNACK", "SNAFU", "SNAGS", "SNAIL", "SNAKE", "SNAKY", "SNAPS", "SNARE", "SNARF",
    "SNARK", "SNARL", "SNARS", "SNARY", "SNASH", "SNATH", "SNAWS", "SNEAD", "SNEAK", "SNEAP",
    "SNEBS", "SNECK", "SNEDS", "SNEED", "SNEER", "SNEES", "SNELL", "SNIBS", "SNICK", "SNIDE",
    "SNIES", "SNIFF", "SNIFT", "SNIGS", "SNIPE", "SNIPS", "SNIPY", "SNIRT", "SNITS", "SNOBS",
    "SNODS", "SNOEK", "SNOEP", "SNOGS", "SNOKE", "SNOOD", "SNOOK", "SNOOL", "SNOOP", "SNOOT",
    "SNORE", "SNORT", "SNOTS", "SNOUT", "SNOWK", "SNOWS", "SNOWY", "SNUBS", "SNUCK", "SNUFF",
    "SNUGS", "SNUSH", "SNYES", "SOAKS", "SOAPS", "SOAPY", "SOARE", "SOARS", "SOAVE", "SOBAS",
    "SOBER", "SOCAS", "SOCKO", "SOCKS", "SOCLE", "SODAS", "SODDY", "SODIC", "SODOM", "SOFAR",
    "SOFAS", "SOFTA", "SOFTS", "SOFTY", "SOGER", "SOGGY", "SOILS", "SOILY", "SOJAS", "SOKAH",
    "SOKEN", "SOKES", "SOKOL", "SOLAH", "SOLAN", "SOLAR", "SOLAS", "SOLDE", "SOLDI", "SOLDO",
    "SOLDS", "SOLED", "SOLEI", "SOLER", "SOLES", "SOLID", "SOLON", "SOLOS", "SOLUM", "SOLUS",
    "SOLVE", "SOMAN", "SOMAS", "SONAR", "SONCE", "SONDE", "SONES", "SONGS", "SONIC", "SONLY",
    "SONNE", "SONNY", "SONSE", "SONSY", "SOOEY", "SOOKS", "SOOLE", "SOOLS", "SOOMS", "SOOPS",
    "SOOTE", "SOOTH", "SOOTS", "SOOTY", "SOPHS", "SOPHY", "SOPOR", "SOPPY", "SOPRA", "SORAL",
    "SORAS", "SORBO", "SORBS", "SORDA", "SORDO", "SORDS", "SORED", "SOREE", "SOREL", "SORER",
    "SORES", "SOREX", "SORGO", "SORNS", "SORRA", "SORRY", "SORTA", "SORTS", "SORUS", "SOTHS",
    "SOTOL", "SOUCE", "SOUCT", "SOUGH", "SOUKS", "SOULS", "SOUMS", "SOUND", "SOUPS", "SOUPY",
    "SOURS", "SOUSE", "SOUTH", "SOUTS", "SOWAR", "SOWCE", "SOWED", "SOWER", "SOWFF", "SOWFS",
    "SOWLE", "SOWLS", "SOWMS", "SOWND", "SOWNE", "SOWPS", "SOWSE", "SOWTH", "SOYAS", "SOYLE",
    "SOYUZ", "SOZIN", "SPACE", "SPACY", "SPADE", "SPADO", "SPAED", "SPAER", "SPAES", "SPAGS",
    "SPAHI", "SPAIL", "SPAIN", "SPAIT", "SPAKE", "SPALD", "SPALE", "SPALL", "SPALT", "SPAMS",
    "SPANE", "SPANG", "SPANK", "SPANS", "SPARD", "SPARE", "SPARK", "SPARS", "SPART", "SPASM",
    "SPATE", "SPATS", "SPAUL", "SPAWL", "SPAWN", "SPAWS", "SPAYD", "SPAYS", "SPAZA", "SPAZZ",
    "SPEAK", "SPEAL", "SPEAN", "SPEAR", "SPEAT", "SPECK", "SPECS", "SPEED", "SPEEL", "SPEER",
    "SPEIL", "SPEIR", "SPEKS", "SPELD", "SPELK", "SPELL", "SPELT", "SPEND", "SPENT", "SPEOS",
    "SPERM", "SPETS", "SPEUG", "SPEWS", "SPEWY", "SPIAL", "SPICA", "SPICE", "SPICK", "SPICS",
    "SPICY", "SPIDE", "SPIED", "SPIEL", "SPIER", "SPIES", "SPIFF", "SPIFS", "SPIKE", "SPIKS",
    "SPIKY", "SPILE", "SPILL", "SPILT", "SPIMS", "SPINA", "SPINE", "SPINK", "SPINS", "SPINY",
    "SPIRE", "SPIRT", "SPIRY", "SPITE", "SPITS", "SPITZ", "SPIVS", "SPLAT", "SPLAY", "SPLIT",
    "SPODE", "SPODS", "SPOIL", "SPOKE", "SPOOF", "SPOOK", "SPOOL", "SPOOM", "SPOON", "SPOOR",
    "SPOOT", "SPORE", "SPORT", "SPOSH", "SPOTS", "SPOUT", "SPRAD", "SPRAG", "SPRAT", "SPRAY",
    "SPRED", "SPREE", "SPREW", "SPRIG", "SPRIT", "SPROD", "SPROG", "SPRUE", "SPRUG", "SPUDS",
    "SPUED", "SPUER", "SPUES", "SPUGS", "SPULE", "SPUME", "SPUMY", "SPUNK", "SPURN", "SPURS",
    "SPURT", "SPUTA", "SPYAL", "SPYRE", "SQUAB", "SQUAD", "SQUAT", "SQUAW", "SQUEG", "SQUIB",
    "SQUID", "SQUIT", "SQUIZ", "STABS", "STACK", "STADE", "STAFF", "STAGE", "STAGS", "STAGY",
    "STAID", "STAIG", "STAIN", "STAIR", "STAKE", "STALE", "STALK", "STALL", "STAMP", "STAND",
    "STANE", "STANG", "STANK", "STAPH", "STAPS", "STARE", "STARK", "STARN", "STARR", "STARS",
    "START", "STASH", "STATE", "STATS", "STAUN", "STAVE", "STAWS", "STAYS", "STEAD", "STEAK",
    "STEAL", "STEAM", "STEAN", "STEAR", "STEDD", "STEDE", "STEDS", "STEED", "STEEK", "STEEL",
    "STEEM", "STEEN", "STEEP", "STEER", "STEIL", "STEIN", "STELA", "STELE", "STELL", "STEME",
    "STEMS", "STEND", "STENO", "STENS", "STENT", "STEPS", "STEPT", "STERE", "STERN", "STETS",
    "STEWS", "STEWY", "STICH", "STICK", "STIED", "STIES", "STIFF", "STILB", "STILE", "STILL",
    "STILT", "STIME", "STIMS", "STIMY", "STING", "STINK", "STINT", "STIPA", "STIPE", "STIRE",
    "STIRK", "STIRP", "STIRS", "STIVE", "STIVY", "STOAE", "STOAI", "STOAS", "STOAT", "STOBS",
    "STOCK", "STOEP", "STOGY", "STOIC", "STOIT", "STOKE", "STOLE", "STOLN", "STOMA", "STOMP",
    "STOND", "STONE", "STONG", "STONK", "STONN", "STONY", "STOOD", "STOOK", "STOOL", "STOOP",
    "STOOR", "STOPE", "STOPS", "STOPT", "STORE", "STORK", "STORM", "STORY", "STOSS", "STOTS",
    "STOTT", "STOUN", "STOUP", "STOUR", "STOUT", "STOVE", "STOWN", "STOWP", "STOWS", "STRAD",
    "STRAE", "STRAG", "STRAP", "STRAW", "STRAY", "STREP", "STREW", "STRIA", "STRIG", "STRIP",
    "STROP", "STROW", "STROY", "STRUM", "STRUT", "STUBS", "STUCK", "STUDS", "STUDY", "STUFF",
    "STULL", "STULM", "STUMM", "STUMP", "STUMS", "STUNG", "STUNK", "STUNS", "STUNT", "STUPA",
    "STUPE", "STURE", "STURT", "STYED", "STYES", "STYLE", "STYLI", "STYLO", "STYME", "STYMY",
    "STYRE", "STYTE", "SUAVE", "SUBAH", "SUBAS", "SUBBY", "SUBER", "SUBHA", "SUCCI", "SUCKS",
    "SUCKY", "SUCRE", "SUDDS", "SUDOR", "SUDSY", "SUEDE", "SUENT", "SUERS", "SUETS", "SUETY",
    "SUGAN", "SUGAR", "SUGHS", "SUIDS", "SUING", "SUINT", "SUITE", "SUITS", "SUJEE", "SUKHS",
    "SULCI", "SULFA", "SULFO", "SULKS", "SULKY", "SULLY", "SULUS", "SUMAC", "SUMMA", "SUMOS",
    "SUMPH", "SUMPS", "SUNKS", "SUNNA", "SUNNS", "SUNNY", "SUNUP", "SUPER", "SUPES", "SUPRA",
    "SURAH", "SURAL", "SURAS", "SURAT", "SURDS", "SURED", "SURER", "SURES", "SURFS", "SURFY",
    "SURGE", "SURGY", "SURLY", "SURRA", "SUSES", "SUSHI", "SUSUS", "SUTOR", "SUTRA", "SUTTA",
    "SWABS", "SWACK", "SWADS", "SWAGE", "SWAGS", "SWAIL", "SWAIN", "SWALE", "SWALY", "SWAMI",
    "SWAMP", "SWAMY", "SWANG", "SWANK", "SWANS", "SWAPS", "SWAPT", "SWARD", "SWARE", "SWARF",
    "SWARM", "SWART", "SWASH", "SWATH", "SWATS", "SWAYL", "SWAYS", "SWEAL", "SWEAR", "SWEAT",
    "SWEDE", "SWEED", "SWEEL", "SWEEP", "SWEER", "SWEES", "SWEET", "SWEIR", "SWELL", "SWELT",
    "SWEPT", "SWERF", "SWEYS", "SWIES", "SWIFT", "SWIGS", "SWILL", "SWIMS", "SWINE", "SWING",
    "SWINK", "SWIPE", "SWIRE", "SWIRL", "SWISH", "SWISS", "SWITH", "SWITS", "SWIVE", "SWIZZ",
    "SWOBS", "SWOLN", "SWONE", "SWOON", "SWOOP", "SWOPS", "SWOPT", "SWORD", "SWORE", "SWORN",
    "SWOTS", "SWOUN", "SWUNG", "SYBBE", "SYBIL", "SYBOE", "SYBOW", "SYCEE", "SYCES", "SYENS",
    "SYKER", "SYKES", "SYLIS", "SYLPH", "SYLVA", "SYMAR", "SYNCH", "SYNCS", "SYNDS", "SYNED",
    "SYNES", "SYNOD", "SYNTH", "SYPED", "SYPES", "SYPHS", "SYRAH", "SYREN", "SYRUP", "SYSOP",
    "SYTHE", "SYVER", "TAALS", "TAATA", "TABBY", "TABER", "TABES", "TABID", "TABIS", "TABLA",
    "TABLE", "TABOO", "TABOR", "TABUN", "TABUS", "TACAN", "TACES", "TACET", "TACHE", "TACHO",
    "TACHS", "TACIT", "TACKS", "TACKY", "TACOS", "TACTS", "TAELS", "TAFFY", "TAFIA", "TAGGY",
    "TAGMA", "TAHAS", "TAHOU", "TAHRS", "TAIGA", "TAIGS", "TAILS", "TAINS", "TAINT", "TAIRA",
    "TAISH", "TAITS", "TAJES", "TAKAS", "TAKEN", "TAKER", "TAKES", "TAKHI", "TAKIN", "TAKIS",
    "TALAK", "TALAQ", "TALAR", "TALAS", "TALCS", "TALCY", "TALEA", "TALER", "TALES", "TALKS",
    "TALKY", "TALLS", "TALLY", "TALMA", "TALON", "TALPA", "TALUK", "TALUS", "TAMAL", "TAMED",
    "TAMER", "TAMES", "TAMIN", "TAMIS", "TAMMY", "TAMPS", "TANAS", "TANGA", "TANGI", "TANGO",
    "TANGS", "TANGY", "TANHS", "TANKA", "TANKS", "TANKY", "TANNA", "TANSY", "TANTI", "TANTO",
    "TAPAS", "TAPED", "TAPEN", "TAPER", "TAPES", "TAPET", "TAPIR", "TAPIS", "TAPPA", "TAPUS",
    "TARAS", "TARDO", "TARDY", "TARED", "TARES", "TARGE", "TARNS", "TAROC", "TAROK", "TAROS",
    "TAROT", "TARPS", "TARRE", "TARRY", "TARSI", "TARTS", "TARTY", "TASAR", "TASER", "TASKS",
    "TASSE", "TASTE", "TASTY", "TATAR", "TATER", "TATES", "TATHS", "TATIE", "TATOU", "TATTS",
    "TATTY", "TATUS", "TAUBE", "TAULD", "TAUNT", "TAUON", "TAUPE", "TAUTS", "TAVAH", "TAVAS",
    "TAVER", "TAWAI", "TAWAS", "TAWED", "TAWER", "TAWIE", "TAWNY", "TAWSE", "TAWTS", "TAXED",
    "TAXER", "TAXES", "TAXIS", "TAXOL", "TAXON", "TAXOR", "TAXUS", "TAYRA", "TAZZA", "TAZZE",
    "TEACH", "TEADE", "TEADS", "TEAED", "TEAKS", "TEALS", "TEAMS", "TEARS", "TEARY", "TEASE",
    "TEATS", "TEAZE", "TECHS", "TECHY", "TECTA", "TEDDY", "TEELS", "TEEMS", "TEEND", "TEENE",
    "TEENS", "TEENY", "TEERS", "TEETH", "TEFFS", "TEGGS", "TEGUA", "TEGUS", "TEHRS", "TEIID",
    "TEILS", "TEIND", "TELAE", "TELCO", "TELES", "TELEX", "TELIA", "TELIC", "TELLS", "TELLY",
    "TELOI", "TELOS", "TEMED", "TEMES", "TEMPI", "TEMPO", "TEMPS", "TEMPT", "TEMSE", "TENCH",
    "TENDS", "TENDU", "TENES", "TENET", "TENGE", "TENIA", "TENNE", "TENNO", "TENNY", "TENON",
    "TENOR", "TENSE", "TENTH", "TENTS", "TENTY", "TENUE", "TEPAL", "TEPAS", "TEPEE", "TEPID",
    "TEPOY", "TERAI", "TERAS", "TERCE", "TEREK", "TERES", "TERFE", "TERFS", "TERGA", "TERMS",
    "TERNE", "TERNS", "TERRA", "TERRY", "TERSE", "TERTS", "TESLA", "TESTA", "TESTE", "TESTS",
    "TESTY", "TETES", "TETHS", "TETRA", "TETRI", "TEUCH", "TEUGH", "TEWED", "TEWEL", "TEWIT",
    "TEXAS", "TEXES", "TEXTS", "THACK", "THAGI", "THAIM", "THALI", "THANA", "THANE", "THANK",
    "THANS", "THARM", "THARS", "THAWS", "THAWY", "THEBE", "THECA", "THEED", "THEEK", "THEES",
    "THEFT", "THEGN", "THEIC", "THEIN", "THEIR", "THELF", "THEMA", "THEME", "THENS", "THEOW",
    "THERE", "THERM", "THESE", "THESP", "THETA", "THETE", "THEWS", "THEWY", "THICK", "THIEF",
    "THIGH", "THIGS", "THILK", "THILL", "THINE", "THING", "THINK", "THINS", "THIOL", "THIRD",
    "THIRL", "THOFT", "THOLE", "THOLI", "THONG", "THORN", "THORO", "THORP", "THOSE", "THOUS",
    "THOWL", "THRAE", "THRAW", "THREE", "THREW", "THRID", "THRIP", "THROB", "THROE", "THROW",
    "THRUM", "THUDS", "THUGS", "THUJA", "THUMB", "THUMP", "THUNK", "THURL", "THUYA", "THYME",
    "THYMI", "THYMY", "TIARA", "TIARS", "TIBIA", "TICAL", "TICCA", "TICED", "TICES", "TICHY",
    "TICKS", "TICKY", "TIDAL", "TIDDY", "TIDED", "TIDES", "TIERS", "TIFFS", "TIFTS", "TIGER",
    "TIGES", "TIGHT", "TIGON", "TIKAS", "TIKES", "TIKIS", "TIKKA", "TILAK", "TILDE", "TILED",
    "TILER", "TILES", "TILLS", "TILLY", "TILTH", "TILTS", "TIMBO", "TIMED", "TIMER", "TIMES",
    "TIMID", "TIMON", "TIMPS", "TINCT", "TINDS", "TINEA", "TINED", "TINES", "TINGE", "TINGS",
    "TINKS", "TINNY", "TINTS", "TINTY", "TIPIS", "TIPPY", "TIPSY", "TIRED", "TIRES", "TIRLS",
    "TIROS", "TIRRS", "TITAN", "TITCH", "TITER", "TITHE", "TITIS", "TITLE", "TITRE", "TITTY",
    "TITUP", "TIZZY", "TOADS", "TOADY", "TOAST", "TOAZE", "TOCKS", "TOCKY", "TOCOS", "TODAY",
    "TODDE", "TODDY", "TOEAS", "TOFFS", "TOFFY", "TOFTS", "TOFUS", "TOGAE", "TOGAS", "TOGED",
    "TOGES", "TOGUE", "TOHOS", "TOILE", "TOILS", "TOING", "TOISE", "TOITS", "TOKAY", "TOKED",
    "TOKEN", "TOKER", "TOKES", "TOKOS", "TOLAN", "TOLAR", "TOLAS", "TOLED", "TOLES", "TOLLS",
    "TOLLY", "TOLTS", "TOLUS", "TOLYL", "TOMAN", "TOMBS", "TOMES", "TOMIA", "TOMMY", "TOMOS",
    "TONAL", "TONDI", "TONDO", "TONED", "TONER", "TONES", "TONEY", "TONGA", "TONGS", "TONIC",
    "TONKA", "TONKS", "TONNE", "TONUS", "TOOLS", "TOOMS", "TOONS", "TOOTH", "TOOTS", "TOPAZ",
    "TOPED", "TOPEE", "TOPEK", "TOPER", "TOPES", "TOPHE", "TOPHI", "TOPHS", "TOPIC", "TOPIS",
    "TOPOI", "TOPOS", "TOQUE", "TORAH", "TORAN", "TORAS", "TORCH", "TORCS", "TORES", "TORIC",
    "TORII", "TOROS", "TOROT", "TORRS", "TORSE", "TORSI", "TORSK", "TORSO", "TORTA", "TORTE",
    "TORTS", "TORUS", "TOSAS", "TOSED", "TOSES", "TOSHY", "TOSSY", "TOTAL", "TOTED", "TOTEM",
    "TOTER", "TOTES", "TOTTY", "TOUCH", "TOUGH", "TOUKS", "TOUNS", "TOURS", "TOUSE", "TOUSY",
    "TOUTS", "TOUZE", "TOUZY", "TOWED", "TOWEL", "TOWER", "TOWIE", "TOWNS", "TOWNY", "TOWSE",
    "TOWSY", "TOWTS", "TOWZE", "TOWZY", "TOXIC", "TOXIN", "TOYED", "TOYER", "TOYON", "TOYOS",
    "TOZED", "TOZES", "TOZIE", "TRABS", "TRACE", "TRACK", "TRACT", "TRADE", "TRADS", "TRAGI",
    "TRAIK", "TRAIL", "TRAIN", "TRAIT", "TRAMP", "TRAMS", "TRANK", "TRANQ", "TRANS", "TRANT",
    "TRAPE", "TRAPS", "TRAPT", "TRASH", "TRASS", "TRATS", "TRATT", "TRAVE", "TRAWL", "TRAYS",
    "TREAD", "TREAT", "TRECK", "TREED", "TREEN", "TREES", "TREFA", "TREIF", "TREKS", "TREMA",
    "TREND", "TRESS", "TREST", "TRETS", "TREWS", "TREYS", "TRIAC", "TRIAD", "TRIAL", "TRIBE",
    "TRICE", "TRICK", "TRIDE", "TRIED", "TRIER", "TRIES", "TRIFF", "TRIGO", "TRIGS", "TRIKE",
    "TRILD", "TRILL", "TRIMS", "TRINE", "TRINS", "TRIOL", "TRIOR", "TRIOS", "TRIPE", "TRIPS",
    "TRIPY", "TRIST", "TRITE", "TROAD", "TROAK", "TROAT", "TROCK", "TRODE", "TRODS", "TROGS",
    "TROIS", "TROKE", "TROLL", "TROMP", "TRONA", "TRONC", "TRONE", "TRONK", "TRONS", "TROOP",
    "TROOZ", "TROPE", "TROTH", "TROTS", "TROUT", "TROVE", "TROWS", "TROYS", "TRUCE", "TRUCK",
    "TRUED", "TRUER", "TRUES", "TRUGO", "TRUGS", "TRULL", "TRULY", "TRUMP", "TRUNK", "TRUSS",
    "TRUST", "TRUTH", "TRYER", "TRYKE", "TRYMA", "TRYPS", "TRYST", "TSADE", "TSADI", "TSARS",
    "TSKED", "TSUBA", "TUANS", "TUART", "TUATH", "TUBAE", "TUBAL", "TUBAR", "TUBAS", "TUBBY",
    "TUBED", "TUBER", "TUBES", "TUCKS", "TUFAS", "TUFFE", "TUFFS", "TUFTS", "TUFTY", "TUGRA",
    "TUINA", "TUISM", "TUKTU", "TULES", "TULIP", "TULLE", "TULPA", "TUMID", "TUMMY", "TUMOR",
    "TUMPS", "TUMPY", "TUNAS", "TUNDS", "TUNED", "TUNER", "TUNES", "TUNGS", "TUNIC", "TUNNY",
    "TUPEK", "TUPIK", "TUPLE", "TUQUE", "TURBO", "TURDS", "TURFS", "TURFY", "TURKS", "TURME",
    "TURMS", "TURNS", "TURPS", "TUSHY", "TUSKS", "TUSKY", "TUTEE", "TUTOR", "TUTTI", "TUTTY",
    "TUTUS", "TUXES", "TUYER", "TWAES", "TWAIN", "TWALS", "TWANG", "TWANK", "TWATS", "TWAYS",
    "TWEAK", "TWEED", "TWEEL", "TWEEN", "TWEER", "TWEET", "TWERP", "TWICE", "TWIER", "TWIGS",
    "TWILL", "TWILT", "TWINE", "TWINK", "TWINS", "TWINY", "TWIRE", "TWIRL", "TWIRP", "TWIST",
    "TWITE", "TWITS", "TWIXT", "TWOER", "TWYER", "TYEES", "TYERS", "TYING", "TYIYN", "TYKES",
    "TYLER", "TYMPS", "TYNDE", "TYNED", "TYNES", "TYPAL", "TYPED", "TYPES", "TYPEY", "TYPIC",
    "TYPOS", "TYPPS", "TYPTO", "TYRAN", "TYRED", "TYRES", "TYROS", "TYTHE", "TZARS", "UDALS",
    "UDDER", "UDONS", "UGALI", "UGGED", "UHLAN", "UHURU", "UKASE", "ULAMA", "ULANS", "ULCER",
    "ULEMA", "ULMIN", "ULNAD", "ULNAE", "ULNAR", "ULNAS", "ULPAN", "ULTRA", "ULVAS", "ULYIE",
    "ULZIE", "UMAMI", "UMBEL", "UMBER", "UMBLE", "UMBOS", "UMBRA", "UMBRE", "UMIAC", "UMIAK",
    "UMIAQ", "UMPED", "UMPIE", "UMPTY", "UNAIS", "UNAPT", "UNARM", "UNARY", "UNAUS", "UNBAG",
    "UNBAN", "UNBAR", "UNBED", "UNBID", "UNBOX", "UNCAP", "UNCES", "UNCIA", "UNCLE", "UNCOS",
    "UNCOY", "UNCUS", "UNCUT", "UNDAM", "UNDEE", "UNDER", "UNDID", "UNDUE", "UNDUG", "UNETH",
    "UNFED", "UNFIT", "UNFIX", "UNGAG", "UNGET", "UNGOD", "UNGOT", "UNGUM", "UNHAT", "UNHIP",
    "UNIFY", "UNION", "UNITE", "UNITS", "UNITY", "UNJAM", "UNKED", "UNKET", "UNKID", "UNLAW",
    "UNLAY", "UNLED", "UNLET", "UNLID", "UNLIT", "UNMAN", "UNMET", "UNMEW", "UNMIX", "UNPAY",
    "UNPEG", "UNPEN", "UNPIN", "UNRED", "UNRID", "UNRIG", "UNRIP", "UNSAY", "UNSET", "UNSEW",
    "UNSEX", "UNSOD", "UNTAX", "UNTIE", "UNTIL", "UNTIN", "UNWED", "UNWET", "UNWIT", "UNWON",
    "UNZIP", "UPBOW", "UPBYE", "UPDOS", "UPDRY", "UPEND", "UPJET", "UPLAY", "UPLED", "UPLIT",
    "UPPED", "UPPER", "UPRAN", "UPRUN", "UPSEE", "UPSET", "UPSEY", "UPTAK", "UPTER", "UPTIE",
    "URAEI", "URALI", "URAOS", "URARE", "URARI", "URASE", "URATE", "URBAN", "URBIA", "URDEE",
    "UREAL", "UREAS", "UREDO", "UREIC", "URENA", "URENT", "URGED", "URGER", "URGES", "URIAL",
    "URINE", "URITE", "URMAN", "URNAL", "URNED", "URPED", "URSAE", "URSID", "URSON", "URUBU",
    "URVAS", "USAGE", "USERS", "USHER", "USING", "USNEA", "USQUE", "USUAL", "USURE", "USURP",
    "USURY", "UTERI", "UTILE", "UTTER", "UVEAL", "UVEAS", "UVULA", "VACUA", "VADED", "VADES",
    "VAGAL", "VAGUE", "VAGUS", "VAILS", "VAIRE", "VAIRS", "VAIRY", "VAKIL", "VALES", "VALET",
    "VALID", "VALIS", "VALOR", "VALSE", "VALUE", "VALVE", "VAMPS", "VAMPY", "VANDA", "VANED",
    "VANES", "VANGS", "VANTS", "VAPID", "VAPOR", "VARAN", "VARAS", "VARDY", "VAREC", "VARES",
    "VARIA", "VARIX", "VARNA", "VARUS", "VARVE", "VASAL", "VASES", "VASTS", "VASTY", "VATIC",
    "VATUS", "VAUCH", "VAULT", "VAUNT", "VAUTE", "VAUTS", "VAWTE", "VEALE", "VEALS", "VEALY",
    "VEENA", "VEEPS", "VEERS", "VEERY", "VEGAN", "VEGAS", "VEGES", "VEGIE", "VEGOS", "VEHME",
    "VEILS", "VEILY", "VEINS", "VEINY", "VELAR", "VELDS", "VELDT", "VELES", "VELLS", "VELUM",
    "VENAE", "VENAL", "VENDS", "VENEY", "VENGE", "VENIN", "VENOM", "VENTS", "VENUE", "VENUS",
    "VERBS", "VERGE", "VERRA", "VERRY", "VERSE", "VERSO", "VERST", "VERTS", "VERTU", "VERVE",
    "VESPA", "VESTA", "VESTS", "VETCH", "VEXED", "VEXER", "VEXES", "VEXIL", "VEZIR", "VIALS",
    "VIAND", "VIBES", "VIBEX", "VIBEY", "VICAR", "VICED", "VICES", "VICHY", "VIDEO", "VIERS",
    "VIEWS", "VIEWY", "VIFDA", "VIGAS", "VIGIA", "VIGIL", "VIGOR", "VILDE", "VILER", "VILLA",
    "VILLI", "VILLS", "VIMEN", "VINAL", "VINAS", "VINCA", "VINED", "VINER", "VINES", "VINEW",
    "VINIC", "VINOS", "VINTS", "VINYL", "VIOLA", "VIOLD", "VIOLS", "VIPER", "VIRAL", "VIRED",
    "VIREO", "VIRES", "VIRGA", "VIRGE", "VIRID", "VIRLS", "VIRTU", "VIRUS", "VISAS", "VISED",
    "VISES", "VISIE", "VISIT", "VISNE", "VISON", "VISOR", "VISTA", "VISTO", "VITAE", "VITAL",
    "VITAS", "VITEX", "VITTA", "VIVAS", "VIVAT", "VIVDA", "VIVER", "VIVES", "VIVID", "VIXEN",
    "VIZIR", "VIZOR", "VLEIS", "VLIES", "VOARS", "VOCAB", "VOCAL", "VOCES", "VODKA", "VODOU",
    "VODUN", "VOEMA", "VOGIE", "VOGUE", "VOICE", "VOIDS", "VOILA", "VOILE", "VOLAE", "VOLAR",
    "VOLED", "VOLES", "VOLET", "VOLKS", "VOLTA", "VOLTE", "VOLTI", "VOLTS", "VOLVA", "VOLVE",
    "VOMER", "VOMIT", "VOTED", "VOTER", "VOTES", "VOUCH", "VOUGE", "VOULU", "VOWED", "VOWEL",
    "VOWER", "VOXEL", "VOZHD", "VRAIC", "VRILS", "VROOM", "VROUS", "VROUW", "VROWS", "VUGGS",
    "VUGGY", "VUGHS", "VUGHY", "VULGO", "VULNS", "VULVA", "VUTTY", "VYING", "WAACS", "WACKE",
    "WACKO", "WACKS", "WACKY", "WADDS", "WADDY", "WADED", "WADER", "WADES", "WADIS", "WADTS",
    "WAFER", "WAFFS", "WAFTS", "WAGED", "WAGER", "WAGES", "WAGGA", "WAGON", "WAHOO", "WAIDE",
    "WAIFS", "WAIFT", "WAILS", "WAINS", "WAIRS", "WAIST", "WAITE", "WAITS", "WAIVE", "WAKAS",
    "WAKED", "WAKEN", "WAKER", "WAKES", "WAKFS", "WALDO", "WALDS", "WALED", "WALER", "WALES",
    "WALIS", "WALKS", "WALLA", "WALLS", "WALLY", "WALTY", "WALTZ", "WAMED", "WAMES", "WAMUS",
    "WANDS", "WANED", "WANES", "WANEY", "WANGS", "WANKS", "WANKY", "WANLE", "WANLY", "WANNA",
    "WANTS", "WANTY", "WANZE", "WAQFS", "WARBS", "WARBY", "WARDS", "WARED", "WARES", "WAREZ",
    "WARKS", "WARMS", "WARNS", "WARPS", "WARRE", "WARST", "WARTS", "WARTY", "WASES", "WASHY",
    "WASMS", "WASPS", "WASPY", "WASTE", "WASTS", "WATAP", "WATCH", "WATER", "WATTS", "WAUFF",
    "WAUGH", "WAUKS", "WAULK", "WAULS", "WAURS", "WAVED", "WAVER", "WAVES", "WAVEY", "WAWAS",
    "WAWES", "WAWLS", "WAXED", "WAXEN", "WAXER", "WAXES", "WAYED", "WAZIR", "WAZOO", "WEALD",
    "WEALS", "WEAMB", "WEANS", "WEARS", "WEARY", "WEAVE", "WEBBY", "WEBER", "WECHT", "WEDEL",
    "WEDGE", "WEDGY", "WEEDS", "WEEDY", "WEEKE", "WEEKS", "WEELS", "WEEMS", "WEENS", "WEENY",
    "WEEPS", "WEEPY", "WEEST", "WEETE", "WEETS", "WEFTE", "WEFTS", "WEIDS", "WEIGH", "WEILS",
    "WEIRD", "WEIRS", "WEISE", "WEIZE", "WEKAS", "WELCH", "WELDS", "WELKE", "WELKS", "WELKT",
    "WELLS", "WELLY", "WELSH", "WELTS", "WEMBS", "WENCH", "WENDS", "WENGE", "WENNY", "WENTS",
    "WEROS", "WERSH", "WESTS", "WETAS", "WETLY", "WEXED", "WEXES", "WHACK", "WHALE", "WHAMO",
    "WHAMS", "WHANG", "WHAPS", "WHARE", "WHARF", "WHATA", "WHATS", "WHAUP", "WHAUR", "WHEAL",
    "WHEAR", "WHEAT", "WHEEL", "WHEEN", "WHEEP", "WHEFT", "WHELK", "WHELM", "WHELP", "WHENS",
    "WHERE", "WHETS", "WHEWS", "WHEYS", "WHICH", "WHIDS", "WHIFF", "WHIFT", "WHIGS", "WHILE",
    "WHILK", "WHIMS", "WHINE", "WHINS", "WHINY", "WHIPS", "WHIPT", "WHIRL", "WHIRR", "WHIRS",
    "WHISH", "WHISK", "WHISS", "WHIST", "WHITE", "WHITS", "WHITY", "WHIZZ", "WHOLE", "WHOMP",
    "WHOOF", "WHOOP", "WHOOT", "WHOPS", "WHORE", "WHORL", "WHORT", "WHOSE", "WHOSO", "WHUMP",
    "WHUPS", "WICCA", "WICKS", "WICKY", "WIDDY", "WIDEN", "WIDER", "WIDES", "WIDOW", "WIDTH",
    "WIELD", "WIELS", "WIFED", "WIFES", "WIFEY", "WIFIE", "WIFTY", "WIGAN", "WIGGA", "WIGGY",
    "WIGHT", "WILCO", "WILDS", "WILED", "WILES", "WILGA", "WILIS", "WILJA", "WILLS", "WILLY",
    "WILTS", "WIMPS", "WIMPY", "WINCE", "WINCH", "WINDS", "WINDY", "WINED", "WINES", "WINEY",
    "WINGE", "WINGS", "WINGY", "WINKS", "WINNA", "WINNS", "WINOS", "WINZE", "WIPED", "WIPER",
    "WIPES", "WIRED", "WIRER", "WIRES", "WIRRA", "WISED", "WISER", "WISES", "WISHA", "WISHT",
    "WISPS", "WISPY", "WISTS", "WITAN", "WITCH", "WITED", "WITES", "WITHE", "WITHS", "WITHY",
    "WITTY", "WIVED", "WIVER", "WIVES", "WIZEN", "WIZES", "WOADS", "WOALD", "WOCKS", "WODGE",
    "WOFUL", "WOKEN", "WOKKA", "WOLDS", "WOLFS", "WOLLY", "WOLVE", "WOMAN", "WOMBS", "WOMBY",
    "WOMEN", "WOMYN", "WONGA", "WONGI", "WONKS", "WONKY", "WONTS", "WOODS", "WOODY", "WOOED",
    "WOOER", "WOOFS", "WOOFY", "WOOLD", "WOOLS", "WOOLY", "WOONS", "WOOPS", "WOOSE", "WOOSH",
    "WOOTZ", "WOOZY", "WORDS", "WORDY", "WORKS", "WORLD", "WORMS", "WORMY", "WORRY", "WORSE",
    "WORST", "WORTH", "WORTS", "WOULD", "WOUND", "WOVEN", "WOWED", "WOWEE", "WOXEN", "WRACK",
    "WRANG", "WRAPS", "WRAPT", "WRAST", "WRATE", "WRATH", "WRAWL", "WREAK", "WRECK", "WRENS",
    "WREST", "WRICK", "WRIED", "WRIER", "WRIES", "WRING", "WRIST", "WRITE", "WRITS", "WROKE",
    "WRONG", "WROOT", "WROTE", "WROTH", "WRUNG", "WRYER", "WRYLY", "WUDUS", "WULLS", "WURST",
    "WUSES", "WUSHU", "WUSSY", "WUXIA", "WYLED", "WYLES", "WYNDS", "WYNNS", "WYTED", "WYTES",
    "XEBEC", "XENIA", "XENIC", "XENON", "XERIC", "XEROX", "XERUS", "XOANA", "XYLAN", "XYLEM",
    "XYLIC", "XYLOL", "XYLYL", "XYSTI", "XYSTS", "YAARS", "YABBA", "YABBY", "YACCA", "YACHT",
    "YACKA", "YACKS", "YAFFS", "YAGER", "YAGIS", "YAHOO", "YAIRD", "YAKKA", "YAKOW", "YALES",
    "YAMEN", "YAMPY", "YAMUN", "YANGS", "YANKS", "YAPOK", "YAPON", "YAPPS", "YAPPY", "YARCO",
    "YARDS", "YARER", "YARFA", "YARKS", "YARNS", "YARRS", "YARTA", "YARTO", "YATES", "YAUDS",
    "YAULD", "YAUPS", "YAWED", "YAWEY", "YAWLS", "YAWNS", "YAWNY", "YAWPS", "YBORE", "YCLAD",
    "YCLED", "YCOND", "YDRAD", "YDRED", "YEADS", "YEAHS", "YEALM", "YEANS", "YEARD", "YEARN",
    "YEARS", "YEAST", "YECCH", "YECHS", "YECHY", "YEDES", "YEEDS", "YEGGS", "YELKS", "YELLS",
    "YELMS", "YELPS", "YELTS", "YENTA", "YENTE", "YERBA", "YERDS", "YERKS", "YESES", "YESKS",
    "YESTS", "YESTY", "YETIS", "YETTS", "YEUKS", "YEUKY", "YEVEN", "YEVES", "YEWEN", "YEXED",
    "YEXES", "YFERE", "YIELD", "YIKED", "YIKES", "YILLS", "YINCE", "YIPES", "YIPPY", "YIRDS",
    "YIRKS", "YIRRS", "YIRTH", "YITES", "YITIE", "YLEMS", "YLIKE", "YLKES", "YMOLT", "YMPES",
    "YOBBO", "YOCKS", "YODEL", "YODHS", "YODLE", "YOGAS", "YOGEE", "YOGHS", "YOGIC", "YOGIN",
    "YOGIS", "YOICK", "YOJAN", "YOKED", "YOKEL", "YOKER", "YOKES", "YOKUL", "YOLKS", "YOLKY",
    "YOMIM", "YOMPS", "YONIC", "YONIS", "YONKS", "YOOFS", "YOOPS", "YORES", "YORKS", "YORPS",
    "YOUKS", "YOUNG", "YOURN", "YOURS", "YOURT", "YOUSE", "YOUTH", "YOWED", "YOWES", "YOWIE",
    "YOWLS", "YRAPT", "YRENT", "YRIVD", "YRNEH", "YSAME", "YTOST", "YUANS", "YUCAS", "YUCCA",
    "YUCCH", "YUCKO", "YUCKS", "YUCKY", "YUFTS", "YUGAS", "YUKED", "YUKES", "YUKKY", "YUKOS",
    "YULAN", "YULES", "YUMMO", "YUMMY", "YUMPS", "YUPON", "YUPPY", "YURTA", "YURTS", "YUZUS",
    "ZABRA", "ZACKS", "ZAIRE", "ZAKAT", "ZAMAN", "ZAMBO", "ZAMIA", "ZANJA", "ZANTE", "ZANZA",
    "ZANZE", "ZAPPY", "ZARFS", "ZATIS", "ZAXES", "ZAYIN", "ZAZEN", "ZEALS", "ZEBEC", "ZEBRA",
    "ZEBUB", "ZEBUS", "ZEINS", "ZERDA", "ZERKS", "ZEROS", "ZESTS", "ZESTY", "ZETAS", "ZEXES",
    "ZEZES", "ZHOMO", "ZIBET", "ZIFFS", "ZIGAN", "ZILAS", "ZILCH", "ZILLA", "ZILLS", "ZIMBI",
    "ZIMBS", "ZINCO", "ZINCS", "ZINCY", "ZINEB", "ZINES", "ZINGS", "ZINGY", "ZINKE", "ZINKY",
    "ZIPPO", "ZIPPY", "ZIRAM", "ZITIS", "ZIZEL", "ZIZIT", "ZLOTE", "ZLOTY", "ZOAEA", "ZOBOS",
    "ZOBUS", "ZOCCO", "ZOEAE", "ZOEAL", "ZOEAS", "ZOISM", "ZOIST", "ZOMBI", "ZONAE", "ZONAL",
    "ZONDA", "ZONED", "ZONER", "ZONES", "ZONKS", "ZOOEA", "ZOOEY", "ZOOID", "ZOOKS", "ZOOMS",
    "ZOONS", "ZOOTY", "ZOPPA", "ZOPPO", "ZORIL", "ZORIS", "ZORRO", "ZOUKS", "ZOWIE", "ZULUS",
    "ZUPAN", "ZUPAS", "ZURFS", "ZUZIM", "ZYGAL", "ZYGON", "ZYMES", "ZYMIC",
];
