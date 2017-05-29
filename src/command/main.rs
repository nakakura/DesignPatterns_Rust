use std::collections::VecDeque;

trait Command<T> {
    fn execute(&self, &mut T);
    fn undo(&self, &mut T);
}

struct Invoker<'a, Cmd, T: 'a> {
    commands: VecDeque<Cmd>,
    old_commands: Vec<Cmd>,
    target: &'a mut T,
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T> {
    fn new(t: &'a mut T) -> Self {
        Invoker {
            commands: VecDeque::new(),
            old_commands: Vec::new(),
            target: t,
        }
    }

    fn target(&self) -> &T {
        self.target
    }

    fn append_command(&mut self, c: Cmd) {
        self.commands.push_back(c);
    }
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T> where Cmd: Command<T> {
    fn execute_command(&mut self) {
        let command_opt = self.commands.pop_front();
        let _ = command_opt.map(|command| {
            let t = &mut *self.target;
            command.execute(t);
            self.old_commands.push(command);
        });
    }

    fn execute_all_commands(&mut self) {
        while self.commands.front().is_some() {
            self.execute_command();
        }
    }

    fn undo(&mut self) {
        let command_opt = self.old_commands.pop();
        command_opt.map(|command| {
            let t = &mut *self.target;
            command.undo(t);
            self.commands.push_front(command);
        });
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}

enum RoboCommand {
    MoveForward,
    TurnRight,
    TurnLeft,
}


impl Command<Robot> for RoboCommand {
    fn execute(&self, r: &mut Robot) {
        use RoboCommand::*;
        match self {
            &MoveForward => {
                r.move_forward();
            },
            &TurnRight => {
                let (dx, dy) = r.get_direction();
                r.set_direction((dy, -dx));
            }
            &TurnLeft => {
                let (dx, dy) = r.get_direction();
                r.set_direction((-dy, dx));
            }
        }
    }

    fn undo(&self, r: &mut Robot) {
        use RoboCommand::*;
        match self {
            &MoveForward => {
                let c1 = TurnRight;
                c1.execute(r);
                c1.execute(r);
                self.execute(r);
                c1.execute(r);
                c1.execute(r);
            },
            &TurnRight => {
                let c = TurnLeft;
                c.execute(r);
            },
            &TurnLeft => {
                let c = TurnRight;
                c.execute(r);

            }
        }
    }
}

fn main() {
    let mut r = Robot::new();

    let mut invoker = Invoker::new(&mut r);
    assert_eq!(*invoker.target(),
    Robot {
        x: 0,
        y: 0,
        dx: 0,
        dy: 1,
    });

    {
        use RoboCommand::*;
        invoker.append_command(TurnRight);
        invoker.append_command(TurnLeft);
        invoker.append_command(MoveForward);
    }

    invoker.execute_all_commands();
    assert_eq!(*invoker.target(),
    Robot {
        x: 0,
        y: 1,
        dx: 0,
        dy: 1,
    });

    invoker.undo();
    assert_eq!(*invoker.target(),
    Robot {
        x: 0,
        y: 0,
        dx: 0,
        dy: 1,
    });

    invoker.undo();
    assert_eq!(*invoker.target(),
    Robot {
        x: 0,
        y: 0,
        dx: 1,
        dy: 0,
    });
}
