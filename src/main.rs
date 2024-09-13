use std::io;

enum IOResult<T> {
    Read((), Box<dyn FnOnce(String) -> IOResult<T>>),
    Write(String, Box<dyn FnOnce(()) -> IOResult<T>>),
    Done(T),
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.trim_end().len());
    line
}

fn nonfunc_play_a_game() -> i32 {
    println!("What is your name?");
    let name = read();
    println!("Hi {}! What is your age?", name);
    let line = read();
    let age = line.parse::<i32>().unwrap();
    println!("Nice to meet you {}, {} is a nice age!", name, age);
    age
}

#[allow(dead_code)]
fn nonfunc_play_games() {
    let mut sum_ages = 0;
    loop {
        let age = nonfunc_play_a_game();
        sum_ages += age;
        println!("Do you want to play again? (y/n)");
        let line = read();
        if line.to_lowercase() != "y" {
            break;
        }
    }
    println!("The sum of ages is {}", sum_ages);
}

fn play_a_game_0() -> IOResult<i32> {
    IOResult::<i32>::Write("What is your name?".into(), Box::new(play_a_game_1))
}

fn play_a_game_1(_: ()) -> IOResult<i32> {
    IOResult::<i32>::Read((), Box::new(play_a_game_2))
}

struct State2 {
    name: String,
}

fn play_a_game_2(name: String) -> IOResult<i32> {
    IOResult::<i32>::Write(
        format!("Hi {}! What is your age?", name),
        Box::new(move |_arg| {
            let state = State2 { name };
            play_a_game_3(state)
        }),
    )
}

fn play_a_game_3(state: State2) -> IOResult<i32> {
    IOResult::<i32>::Read((), Box::new(move |arg: String| play_a_game_4(state, arg)))
}

struct State4 {
    #[allow(dead_code)]
    name: String,
    age: i32,
}

fn play_a_game_4(state: State2, arg: String) -> IOResult<i32> {
    let age = arg.parse::<i32>().unwrap();
    IOResult::<i32>::Write(
        format!("Nice to meet you {}, {} is a nice age!", state.name, age),
        Box::new(move |_arg: ()| {
            play_a_game_5(State4 {
                name: state.name,
                age,
            })
        }),
    )
}

fn play_a_game_5(state: State4) -> IOResult<i32> {
    IOResult::<i32>::Done(state.age)
}

fn play_games() -> IOResult<()> {
    play_games_1(0)
}

fn play_games_1(sum_age: i32) -> IOResult<()> {
    let fut = play_a_game_0();
    match fut {
        IOResult::Read(arg, f) => {
            IOResult::<()>::Read(arg, Box::new(move |arg| play_games_2(sum_age, f, arg)))
        }
        IOResult::Write(arg, f) => {
            IOResult::<()>::Write(arg, Box::new(move |arg| play_games_3(sum_age, f, arg)))
        }
        IOResult::Done(age) => play_games_4(sum_age, age),
    }
}

fn play_games_2(
    sum_age: i32,
    f: Box<dyn FnOnce(String) -> IOResult<i32>>,
    arg: String,
) -> IOResult<()> {
    let fut = f(arg);
    match fut {
        IOResult::Read(arg, f) => {
            IOResult::<()>::Read(arg, Box::new(move |arg| play_games_2(sum_age, f, arg)))
        }
        IOResult::Write(arg, f) => {
            IOResult::<()>::Write(arg, Box::new(move |arg| play_games_3(sum_age, f, arg)))
        }
        IOResult::Done(age) => play_games_4(sum_age, age),
    }
}

fn play_games_3(sum_age: i32, f: Box<dyn FnOnce(()) -> IOResult<i32>>, arg: ()) -> IOResult<()> {
    let fut = f(());
    match fut {
        IOResult::Read(arg, f) => {
            IOResult::<()>::Read(arg, Box::new(move |arg| play_games_2(sum_age, f, arg)))
        }
        IOResult::Write(arg, f) => {
            IOResult::<()>::Write(arg, Box::new(move |arg| play_games_3(sum_age, f, arg)))
        }
        IOResult::Done(age) => play_games_4(sum_age, age),
    }
}

fn play_games_4(mut sum_age: i32, age: i32) -> IOResult<()> {
    sum_age += age;
    IOResult::Write(
        "Do you want to play again? (y/n)".into(),
        Box::new(move |_| play_games_5(sum_age)),
    )
}

fn play_games_5(sum_age: i32) -> IOResult<()> {
    IOResult::Read((), Box::new(move |line| play_games_6(sum_age, line)))
}

fn play_games_6(sum_age: i32, line: String) -> IOResult<()> {
    if line.to_lowercase() == "y" {
        play_games_1(sum_age)
    } else {
        let out = format!("The sum of ages is {}", sum_age);
        IOResult::<()>::Write(out, Box::new(move |_| IOResult::<()>::Done(())))
    }
}

fn run_async<T>(future: IOResult<T>) -> T {
    let mut future = future;
    loop {
        match future {
            IOResult::Read(_, f) => {
                let line = read();
                future = f(line);
            }
            IOResult::Write(line, f) => {
                println!("{}", line);
                future = f(());
            }
            IOResult::Done(result) => {
                return result;
            }
        }
    }
}

fn main() {
    // nonfunc_play_a_game();
    // run_async(play_a_game_0());
    // nonfunc_play_games();
    run_async(play_games());
}
