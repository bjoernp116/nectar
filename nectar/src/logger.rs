
pub enum LogType {
    ERROR,
    WARNING,
    INFO,
    SUCCESS
}
pub fn log(msg: &str, t: LogType){
    let st = match t {
        LogType::ERROR   => "\x1b[1;31mERROR  ",
        LogType::WARNING => "\x1b[1;33mWARNING",
        LogType::INFO    => "\x1b[1;97mINFO   ",
        LogType::SUCCESS => "\x1b[1;32mSUCCESS"
    };
    println!("{}\x1B[0m:   {}", st, msg);
}

