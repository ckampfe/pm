use lazy_static::*;
use std::collections::HashMap;
use yew::prelude::*;

pub struct App {
    players: Vec<String>,
    scoreboard: HashMap<String, usize>,
    last_turn_count: usize,
    game_state: GameState,
    link: ComponentLink<Self>,
    adding_player: String,
    turn_points: usize,
    mixed_combo: bool,
    pig_one: Option<Pig>,
}

pub enum GameState {
    Over,
    Playing,
    LastTurn,
    Pregame,
}

pub enum Msg {
    AddPlayer,
    UpdatePlayerName(String),
    StartGame,
    NewGame,

    PigOut,
    MakinBacon,
    MixedCombo,

    McHoofer,
    McRazorback,
    McSnouter,
    McJowler,

    Sider,
    Hoofer,
    Razorback,
    Snouter,
    Jowler,

    DoubleHoofer,
    DoubleRazorback,
    DoubleSnouter,
    DoubleLeaningJowler,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Pig {
    Sider,
    Hoofer,
    Razorback,
    Snouter,
    Jowler,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            players: vec![],
            scoreboard: HashMap::new(),
            last_turn_count: 0,
            game_state: GameState::Pregame,
            link,
            adding_player: "".to_string(),
            turn_points: 0,
            mixed_combo: false,
            pig_one: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePlayerName(player_name) => {
                self.adding_player = player_name;
                true
            }
            Msg::AddPlayer => {
                let player_name = std::mem::take(&mut self.adding_player);
                self.adding_player = "".to_string();
                self.players.push(player_name.clone());
                self.scoreboard.insert(player_name, 0);
                true
            }
            Msg::StartGame => {
                self.game_state = GameState::Playing;
                true
            }
            Msg::NewGame => {
                self.game_state = GameState::Pregame;
                true
            }

            Msg::PigOut => {
                let current_player = self.players[0].clone();

                *self.scoreboard.get_mut(&current_player).unwrap() += self.turn_points;

                let is_last_turn = self
                    .scoreboard
                    .iter()
                    .any(|(_player, &points)| points >= 100);

                if is_last_turn {
                    self.last_turn_count += 1;
                }

                if self.last_turn_count >= self.players.len() {
                    self.game_state = GameState::Over
                }

                self.turn_points = 0;

                self.players.rotate_left(1);

                true
            }
            Msg::MakinBacon => {
                let is_last_turn = self
                    .scoreboard
                    .iter()
                    .any(|(_player, &points)| points >= 100);

                if is_last_turn {
                    self.last_turn_count += 1;
                }

                if self.last_turn_count >= self.players.len() {
                    self.game_state = GameState::Over
                }

                self.turn_points = 0;

                self.players.rotate_left(1);

                true
            }
            Msg::MixedCombo => {
                self.mixed_combo = true;
                true
            }
            Msg::McHoofer => {
                if let Some(pig_one) = self.pig_one {
                    let points = SCORES[&[pig_one, Pig::Hoofer]];
                    self.turn_points += points;
                    self.pig_one = None;
                    self.mixed_combo = false;
                } else {
                    self.pig_one = Some(Pig::Hoofer)
                };

                true
            }
            Msg::McRazorback => {
                if let Some(pig_one) = self.pig_one {
                    let points = SCORES[&[pig_one, Pig::Razorback]];
                    self.turn_points += points;
                    self.pig_one = None;
                    self.mixed_combo = false;
                } else {
                    self.pig_one = Some(Pig::Razorback)
                };

                true
            }
            Msg::McSnouter => {
                if let Some(pig_one) = self.pig_one {
                    let points = SCORES[&[pig_one, Pig::Snouter]];
                    self.turn_points += points;
                    self.pig_one = None;
                    self.mixed_combo = false;
                } else {
                    self.pig_one = Some(Pig::Snouter)
                };

                true
            }
            Msg::McJowler => {
                if let Some(pig_one) = self.pig_one {
                    let points = SCORES[&[pig_one, Pig::Jowler]];
                    self.turn_points += points;
                    self.pig_one = None;
                    self.mixed_combo = false;
                } else {
                    self.pig_one = Some(Pig::Jowler)
                };

                true
            }

            Msg::Sider => {
                let points = SCORES[&[Pig::Sider, Pig::Sider]];
                self.turn_points += points;
                true
            }
            Msg::Hoofer => {
                let points = SCORES[&[Pig::Hoofer, Pig::Sider]];
                self.turn_points += points;
                true
            }
            Msg::Razorback => {
                let points = SCORES[&[Pig::Razorback, Pig::Sider]];
                self.turn_points += points;
                true
            }
            Msg::Snouter => {
                let points = SCORES[&[Pig::Snouter, Pig::Sider]];
                self.turn_points += points;
                true
            }
            Msg::Jowler => {
                let points = SCORES[&[Pig::Jowler, Pig::Sider]];
                self.turn_points += points;
                true
            }
            Msg::DoubleHoofer => {
                let points = SCORES[&[Pig::Hoofer, Pig::Hoofer]];
                self.turn_points += points;
                true
            }
            Msg::DoubleRazorback => {
                let points = SCORES[&[Pig::Razorback, Pig::Razorback]];
                self.turn_points += points;
                true
            }
            Msg::DoubleSnouter => {
                let points = SCORES[&[Pig::Snouter, Pig::Snouter]];
                self.turn_points += points;
                true
            }
            Msg::DoubleLeaningJowler => {
                let points = SCORES[&[Pig::Jowler, Pig::Jowler]];
                self.turn_points += points;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
                <div class="container">
                    {
            match self.game_state {
                GameState::Pregame => {
                    let add_user = self.link.callback(|_| Msg::AddPlayer);
                    let start_game = self.link.callback(|_| Msg::StartGame);

                    let add_two_players = html! {
                        <>
                        <span>{"Add at least two players to start" }</span>
                        </>
                    };

                    let start_game_button = html! {
                        <>
                        <button onclick=start_game>{ "Start game" }</button>
                        </>
                    };
                    html! {
                        <div>
                            <div>
                                { "Players: " }
                                {
                                    self.players.join(", ")
                                }
                            </div>
                          <input
                              id="player_name"
                              name="player_name"
                              value=self.adding_player
                              type="text"
                              oninput=self.link.callback(|e: InputData| Msg::UpdatePlayerName(e.value)) >
                          </input>
                          <button onclick=add_user>{ "Add User" }</button>

                            <div>
                                 {
                                     if self.players.len() < 2 {
                                         add_two_players
                                     } else {
                                         start_game_button
                                     }
                                 }
                            </div>
                        </div>
                    }
                }
                GameState::Playing | GameState::LastTurn => {

                    let last_turn = html! {
                        <>
                        <div>
                            <h2>{ "Last turn!" }</h2>
                       </div>
                        </>
                    };

                    if self.mixed_combo {
                        let mc = if self.pig_one.is_none() {
                            html! {
                                <>
                                <h3>{ "Pig one?" }</h3>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                <h3>{ "Pig two?" }</h3>
                                </>
                            }
                        };

                        html! {
                            <div>
                                { mc }

                                <button onclick=self.link.callback(|_| Msg::McHoofer)>{ "Hoofer" }</button>
                                <button onclick=self.link.callback(|_| Msg::McRazorback)>{ "Razorback" }</button>
                                <button onclick=self.link.callback(|_| Msg::McSnouter)>{ "Snouter" }</button>
                                <button onclick=self.link.callback(|_| Msg::McJowler)>{ "Leaning Jowler" }</button>
                            </div>
                        }
                    } else {
                        html! {
                            <div>
                                <div>
                                    { format!("Current turn: {}", self.players[0]) }
                                </div>
                                <div>
                                    { format!("Turn points: {}", self.turn_points) }
                                </div>
                                <div>
                            {
                                for self.scoreboard.iter().map(|(player_name, score)| {
                                    html! {
                                        <div>
                                            { format!("{}: {}\n", player_name, score) }
                                        </div>
                                    }
                                })
                            }
                                </div>

                            {
                                if self.last_turn_count >= self.players.len() - 1 {
                                    last_turn
                                } else {
                                    html! {}
                                }
                            }

                                <div>
                                    <button onclick=self.link.callback(|_| Msg::PigOut)>{ "Pig Out" }</button>
                                    <button onclick=self.link.callback(|_| Msg::MakinBacon)>{ "Making Bacon" }</button>
                                    <button onclick=self.link.callback(|_| Msg::MixedCombo)>{ "Mixed Combo" }</button>
                                </div>

                                <hr />

                                <div>
                                    <button onclick=self.link.callback(|_| Msg::Sider)>{ "Sider" }</button>
                                    <button onclick=self.link.callback(|_| Msg::Hoofer)>{ "Hoofer" }</button>
                                    <button onclick=self.link.callback(|_| Msg::Razorback)>{ "Razorback" }</button>
                                    <button onclick=self.link.callback(|_| Msg::Snouter)>{ "Snouter" }</button>
                                    <button onclick=self.link.callback(|_| Msg::Jowler)>{ "Jowler" }</button>
                                </div>

                                <hr />

                                <div>
                                    <button onclick=self.link.callback(|_| Msg::DoubleHoofer)>{ "2x hoofer" }</button>
                                    <button onclick=self.link.callback(|_| Msg::DoubleRazorback)>{ "2x Razorback" }</button>
                                    <button onclick=self.link.callback(|_| Msg::DoubleSnouter)>{ "2x Snouter" }</button>
                                    <button onclick=self.link.callback(|_| Msg::DoubleLeaningJowler)>{ "2x Jowler" }</button>
                                </div>

                                <hr />
                            </div>
                        }
                    }
                }
                GameState::Over => {
                    let mut scoreboard_tuples =
                        self.scoreboard.iter().collect::<Vec<(&String, &usize)>>();
                    scoreboard_tuples.sort_by(|(_, score1), (_, score2)| score1.cmp(&score2));
                    scoreboard_tuples.reverse();

                    html! {
                        <div>
                            <h2>{ format!("{} wins!", scoreboard_tuples[0].0.clone()) }</h2>
                        {
                            for scoreboard_tuples.iter().map(|(player_name, score)| {
                                html! {
                                    <div>
                                { format!("{}: {}\n", player_name, score) }
                                    </div>
                                }
                            })
                        }
                        </div>
                    }
                }
            }
        }
            </div>
            }
    }
}

lazy_static! {
    static ref SCORES: HashMap<[Pig; 2], usize> = {
        let mut scores = HashMap::new();
        scores.insert([Pig::Hoofer, Pig::Hoofer], 20);
        scores.insert([Pig::Hoofer, Pig::Jowler], 15);
        scores.insert([Pig::Hoofer, Pig::Razorback], 5);
        scores.insert([Pig::Hoofer, Pig::Sider], 5);
        scores.insert([Pig::Hoofer, Pig::Snouter], 10);
        scores.insert([Pig::Jowler, Pig::Hoofer], 15);
        scores.insert([Pig::Jowler, Pig::Jowler], 60);
        scores.insert([Pig::Jowler, Pig::Razorback], 15);
        scores.insert([Pig::Jowler, Pig::Sider], 15);
        scores.insert([Pig::Jowler, Pig::Snouter], 15);
        scores.insert([Pig::Razorback, Pig::Hoofer], 5);
        scores.insert([Pig::Razorback, Pig::Jowler], 15);
        scores.insert([Pig::Razorback, Pig::Razorback], 20);
        scores.insert([Pig::Razorback, Pig::Sider], 5);
        scores.insert([Pig::Razorback, Pig::Snouter], 10);
        scores.insert([Pig::Sider, Pig::Hoofer], 5);
        scores.insert([Pig::Sider, Pig::Jowler], 15);
        scores.insert([Pig::Sider, Pig::Razorback], 5);
        scores.insert([Pig::Sider, Pig::Sider], 1);
        scores.insert([Pig::Sider, Pig::Snouter], 10);
        scores.insert([Pig::Snouter, Pig::Hoofer], 10);
        scores.insert([Pig::Snouter, Pig::Jowler], 15);
        scores.insert([Pig::Snouter, Pig::Razorback], 10);
        scores.insert([Pig::Snouter, Pig::Sider], 10);
        scores.insert([Pig::Snouter, Pig::Snouter], 40);
        scores
    };
}
