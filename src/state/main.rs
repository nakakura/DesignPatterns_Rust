// 制御するボタンが一つだけついた電子サイコロ
// ボタンを押す、電源ON & サイコロ回転開始
// ボタンを押す、サイコロ停止(出目が決まる)
// ボタンを押す、電源OFF、最初に戻る.
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug)]
enum StateContextEnum {
    StatePowerOn(StatePowerOn),
    StatePowerOff(StatePowerOff),
    StateStop(StateStop),
}

trait State {
    fn on_press_button(&self) -> StateContextEnum;
}

impl State for StateContextEnum {
    fn on_press_button(&self) -> StateContextEnum {
        match self {
            &StateContextEnum::StatePowerOn(ref s) => {
                s.on_press_button()
            },
            &StateContextEnum::StatePowerOff(ref s) => {
                s.on_press_button()
            },
            &StateContextEnum::StateStop(ref s) => {
                s.on_press_button()
            },
        }
    }
}

#[derive(Debug)]
struct StatePowerOff;

impl StatePowerOff {
    pub fn new() -> Self {
        StatePowerOff{}
    }
}

impl State for StatePowerOff {
    fn on_press_button(&self) -> StateContextEnum {
        // Something to do for turning on the dice.
        println!("Power on.");
        StateContextEnum::StatePowerOn(StatePowerOn::new())
    }
}

#[derive(Debug)]
struct StatePowerOn;

impl StatePowerOn {
    pub fn new() -> Self {
        StatePowerOn{}
    }
}

impl State for StatePowerOn {
    fn on_press_button(&self) -> StateContextEnum {
        // Something to do for turning on the dice.
        println!("shaking dice.");
        StateContextEnum::StateStop(StateStop::new())
    }
}

#[derive(Debug)]
struct StateStop;

impl StateStop {
    pub fn new() -> Self {
        StateStop{}
    }
}

impl State for StateStop {
    fn on_press_button(&self) -> StateContextEnum {
        // Something to do for turning on the dice.
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(1, 7);
        println!("Power off and output value is {}.", n);
        StateContextEnum::StatePowerOff(StatePowerOff::new())
    }
}

#[derive(Debug)]
struct Context {
    state: StateContextEnum,
}

impl Context {
    fn new() -> Self {
        Context {
            state: StateContextEnum::StatePowerOff(StatePowerOff::new())
        }
    }

    #[allow(dead_code)]
    fn state(&self) -> &StateContextEnum {
        &self.state
    }

    fn set_state(&mut self, state: StateContextEnum) {
        self.state = state;
    }

    fn press_button(&mut self) {
        let state = self.state.on_press_button();
        self.set_state(state);
    }
}


fn main() {
    let mut context = Context::new();
    for _ in 0..12 {
        context.press_button();
    }
}