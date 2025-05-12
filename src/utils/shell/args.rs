// args.rs
use std::env;
use std::fmt;
use colored::Colorize;

// 引数の種類を表す列挙型
#[derive(Debug, Clone)]
pub enum ArgumentType {
    Simple,      // プレーンな引数（例: "file.txt"）
    ShortOpt,    // 短いオプション（例: "-v"）
    LongOpt,     // 長いオプション（例: "--verbose"）
}

// 引数の情報を保持する構造体
#[derive(Debug, Clone)]
pub struct Argument {
    pub arg_type: ArgumentType,
    pub arg_str: String,           // 引数の生の文字列（例: "--data"）
    pub arg_values: Vec<String>,   // 引数の値（例: ["data1", "data2", "data3"]）
}

// コマンド全体を表す構造体
#[derive(Debug)]
pub struct Command {
    pub cmd_name: String,          // コマンド名（例: "my_program"）
    pub args: Vec<Argument>,       // 引数のリスト
}

// DisplayトレイトをCommandに実装（カラー化）
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // コマンド名を青色で表示
        writeln!(f, "{}: {}", "Command".cyan().bold(), self.cmd_name.blue())?;
        
        // 引数がなければその旨を赤色で表示
        if self.args.is_empty() {
            return writeln!(f, "{}", "No arguments provided.".red());
        }

        // 引数リストのヘッダーを緑色で表示
        writeln!(f, "{}:", "Arguments".green().bold())?;
        for (i, arg) in self.args.iter().enumerate() {
            let arg_type = match arg.arg_type {
                ArgumentType::Simple => "Simple".purple(),
                ArgumentType::ShortOpt => "Short Option".yellow(),
                ArgumentType::LongOpt => "Long Option".cyan(),
            };
            let values = if arg.arg_values.is_empty() {
                "None".red().to_string()
            } else {
                format!("[{}]", arg.arg_values.join(", ").green())
            };
            writeln!(
                f,
                "  {}. {} ({}: {}): {}: {}",
                (i + 1).to_string().bold(),
                arg.arg_str.magenta(),
                "Type".cyan(),
                arg_type,
                "Values".cyan(),
                values
            )?;
        }
        Ok(())
    }
}

impl Command {
    // 新しいCommandインスタンスを作成
    pub fn new(cmd_name: String) -> Self {
        Command {
            cmd_name,
            args: Vec::new(),
        }
    }

    // 引数を追加
    pub fn add_arg(&mut self, arg: Argument) {
        self.args.push(arg);
    }
}

// コマンドライン引数を取得
fn get_args() -> Vec<String> {
    env::args().collect()
}

// 引数の種類を判定
fn determine_arg_type(arg: &str) -> ArgumentType {
    if arg.starts_with("--") {
        ArgumentType::LongOpt
    } else if arg.starts_with("-") {
        ArgumentType::ShortOpt
    } else {
        ArgumentType::Simple
    }
}

// カンマ区切りの値を分割（例: "data1,data2,data3" -> ["data1", "data2", "data3"]）
fn parse_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// 引数を初期化してCommandを返す
pub fn init() -> Command {
    let args = get_args();
    let mut command = if let Some(cmd_name) = args.first() {
        // 最初の引数はコマンド名（プログラム名）
        Command::new(cmd_name.clone())
    } else {
        Command::new(String::new())
    };

    // コマンド名を除く引数を処理
    for arg in args.iter().skip(1) {
        let arg_type = determine_arg_type(arg);
        let mut arg_values = Vec::new();

        // 長いオプションで値が付いている場合（例: "--data=data1,data2,data3"）
        if let ArgumentType::LongOpt = arg_type {
            if let Some((key, value)) = arg.split_once('=') {
                // カンマ区切りの値をパース
                arg_values = parse_values(value);
                // キー部分だけをarg_strとして保存
                command.add_arg(Argument {
                    arg_type,
                    arg_str: key.to_string(),
                    arg_values,
                });
                continue;
            }
        }

        // 値がない場合やシンプルな引数、短いオプション
        command.add_arg(Argument {
            arg_type,
            arg_str: arg.to_string(),
            arg_values,
        });
    }

    command
}