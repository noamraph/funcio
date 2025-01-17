use std::io;

enum IOResult<T> {
    Read((), Box<dyn FnOnce(String) -> IOResult<T>>),
    Write(String, Box<dyn FnOnce(()) -> IOResult<T>>),
    Done(T),
}

fn call_async<T: 'static, S: 'static>(
    fut: IOResult<T>,
    after: Box<dyn FnOnce(T) -> IOResult<S>>,
) -> IOResult<S> {
    match fut {
        IOResult::Read(_, f) => {
            IOResult::<S>::Read((), Box::new(move |line| call_async(f(line), after)))
        }
        IOResult::Write(line, f) => {
            IOResult::<S>::Write(line, Box::new(move |_| call_async(f(()), after)))
        }
        IOResult::Done(result) => after(result),
    }
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

fn play_a_game_2(name: String) -> IOResult<i32> {
    IOResult::<i32>::Write(
        format!("Hi {}! What is your age?", name),
        Box::new(move |_arg| play_a_game_3(name)),
    )
}

fn play_a_game_3(name: String) -> IOResult<i32> {
    IOResult::<i32>::Read((), Box::new(move |line: String| play_a_game_4(name, line)))
}

fn play_a_game_4(name: String, line: String) -> IOResult<i32> {
    let age = line.parse::<i32>().unwrap();
    IOResult::<i32>::Write(
        format!("Nice to meet you {}, {} is a nice age!", name, age),
        Box::new(move |_: ()| play_a_game_5(age)),
    )
}

fn play_a_game_5(age: i32) -> IOResult<i32> {
    IOResult::<i32>::Done(age)
}

//////////////// play_games

fn play_games() -> IOResult<()> {
    play_games_1(0)
}

fn play_games_1(sum_age: i32) -> IOResult<()> {
    call_async(
        play_a_game_0(),
        Box::new(move |age| play_games_2(sum_age, age)),
    )
}

fn play_games_2(mut sum_age: i32, age: i32) -> IOResult<()> {
    sum_age += age;
    IOResult::Write(
        "Do you want to play again? (y/n)".into(),
        Box::new(move |_| play_games_3(sum_age)),
    )
}

fn play_games_3(sum_age: i32) -> IOResult<()> {
    IOResult::Read((), Box::new(move |line| play_games_4(sum_age, line)))
}

fn play_games_4(sum_age: i32, line: String) -> IOResult<()> {
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
