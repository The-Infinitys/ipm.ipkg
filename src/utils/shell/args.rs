//! このモジュールはコマンドライン引数をパースし、構造化された形式で表現するための機能を提供します。
//! シンプルな引数、短いオプション、長いオプション、および連結された短いオプション（例: `-iv`）を扱います。

use colored::Colorize;
use std::env;
use std::fmt;

/// コマンドライン引数の種類を表す列挙型。
#[derive(Debug, Clone)]
pub enum ArgumentType {
    /// プレーンな引数（例: `"file.txt"`）。オプションやフラグではない文字列。
    Simple,
    /// 短いオプション（例: `"-v"`）。単一のハイフンとそれに続く文字で構成されます。
    ShortOpt,
    /// 長いオプション（例: `"--verbose"`）。二重ハイフンとそれに続く文字列で構成されます。
    LongOpt,
}

/// パースされたコマンドライン引数の情報を保持する構造体。
#[derive(Debug, Clone)]
pub struct Argument {
    /// この引数の種類。
    pub arg_type: ArgumentType,
    /// 引数の生の文字列形式。オプションの場合はフラグ自身（例: `"--data"`, `"-v"`, `"-i"`）。
    pub arg_str: String,
    /// この引数に関連付けられた値のリスト。
    /// 例: `--data=data1,data2,data3` の場合、`arg_values` は `["data1", "data2", "data3"]` になります。
    /// Simple引数やShortOptの場合は通常空です。
    pub arg_values: Vec<String>,
}

/// パースされたコマンドライン全体（コマンド名と引数のリスト）を表す構造体。
#[derive(Debug)]
pub struct Command {
    /// 実行されたコマンドの名前（通常はプログラム名）。
    pub cmd_name: String,
    /// パースされた引数のリスト。
    pub args: Vec<Argument>,
}

// DisplayトレイトをCommandに実装（カラー化）
impl fmt::Display for Command {
    /// `Command`構造体の内容を、色付けして人間が読める形式でフォーマットします。
    /// コマンド名、引数の種類、文字列、および関連付けられた値を表示します。
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
    /// 新しい`Command`インスタンスを作成します。
    ///
    /// 初期状態では引数リストは空です。
    ///
    /// # 引数
    ///
    /// * `cmd_name`: コマンドの名前。
    ///
    /// # 戻り値
    ///
    /// 新しい`Command`インスタンス。
    pub fn new(cmd_name: String) -> Self {
        Command {
            cmd_name,
            args: Vec::new(),
        }
    }

    /// 引数リストに`Argument`を追加します。
    ///
    /// # 引数
    ///
    /// * `arg`: 追加する`Argument`構造体。
    pub fn add_arg(&mut self, arg: Argument) {
        self.args.push(arg);
    }
}

/// 環境からコマンドライン引数のベクタを取得します。
///
/// 最初の要素は通常、実行可能ファイル自身のパスです。
///
/// # 戻り値
///
/// コマンドライン引数を文字列のベクタとして返します。
fn get_args() -> Vec<String> {
    env::args().collect()
}

/// 引数文字列の種類（Simple, ShortOpt, LongOpt）を判定します。
///
/// `--` で始まる場合は LongOpt、`-` で始まり長さが2以上の場合は ShortOpt、
/// それ以外は Simple と判定します。`-` 単体は ShortOpt としません。
///
/// # 引数
///
/// * `arg`: 判定する引数文字列スライス。
///
/// # 戻り値
///
/// 引数の種類を示す`ArgumentType`。
fn determine_arg_type(arg: &str) -> ArgumentType {
    if arg.starts_with("--") {
        ArgumentType::LongOpt
    } else if arg.starts_with("-") && arg.len() > 1 {
        // '-' だけは ShortOpt としない
        ArgumentType::ShortOpt
    } else {
        ArgumentType::Simple
    }
}

/// カンマ区切りの文字列を分割し、トリムして空でない要素のベクタを返します。
///
/// # 引数
///
/// * `value`: 分割するカンマ区切りの文字列スライス。
///
/// # 戻り値
///
/// 分割された値の文字列ベクタ。
fn parse_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 環境からコマンドライン引数を取得し、パースして`Command`構造体として返します。
///
/// コマンド名、シンプルな引数、長いオプション（値付き/なし）、および
/// 連結された短いオプション（例: `-iv` を `-i` と `-v` に分解）を処理します。
///
/// # 戻り値
///
/// パースされたコマンドを含む`Command`構造体。
pub fn init() -> Command {
    let args = get_args();
    let mut command = if let Some(cmd_name) = args.first() {
        // 最初の引数はコマンド名（プログラム名）
        Command::new(cmd_name.clone())
    } else {
        // コマンド名がない場合は空文字列を使用
        Command::new(String::new())
    };

    let mut i = 1; // コマンド名を除く最初の引数から開始
    while i < args.len() {
        let arg_str = &args[i];
        let arg_type = determine_arg_type(arg_str);

        // 長いオプションで値が付いている場合（例: "--data=data1,data2,data3"）
        if let ArgumentType::LongOpt = arg_type {
            if let Some((key, value)) = arg_str.split_once('=') {
                let arg_values = parse_values(value);
                command.add_arg(Argument {
                    arg_type,
                    arg_str: key.to_string(), // キー部分だけをarg_strとして保存
                    arg_values,
                });
                i += 1; // この引数は処理済み
                continue; // 次の引数へ
            }
        }

        // 短いオプションが複数連なっている場合（例: "-iv"）
        if let ArgumentType::ShortOpt = arg_type {
            if arg_str.len() > 2 {
                // 長さが2より大きい場合（例: "-iv" は長さ3）
                // '-' に続く各文字を個別の短いオプションとして追加
                for c in arg_str.chars().skip(1) {
                    // 最初の '-' をスキップ
                    let short_opt_str = format!("-{}", c);
                    command.add_arg(Argument {
                        arg_type: ArgumentType::ShortOpt,
                        arg_str: short_opt_str,
                        arg_values: Vec::new(), // 連結された短いオプションは通常値を持たない
                    });
                }
                i += 1; // この引数は処理済み
                continue; // 次の引数へ
            }
        }

        // 上記のどのパターンにも当てはまらない場合（シンプルな引数、単一の短いオプション "-v", 値のない長いオプション "--help"）
        // これらは単一のArgumentとして追加
        let arg_values = Vec::new(); // これらのケースでは値のパースは行わない
        command.add_arg(Argument {
            arg_type,
            arg_str: arg_str.to_string(),
            arg_values,
        });
        i += 1; // この引数は処理済み
    }

    command
}

/// 実行された際のコマンドを文字列として取得します。
pub fn cmd_str() -> String {
    get_args().join(" ")
}

/*
// main.rs などで以下のように使用できます

mod args;

fn main() {
    let command = args::init();
    println!("{}", command);
}

// 実行例:
// cargo run -- -iv file.txt --data=a,b --verbose
//
// 出力例:
// Command: target/debug/your_program_name
// Arguments:
//   1. -i (Type: Short Option): Values: None
//   2. -v (Type: Short Option): Values: None
//   3. file.txt (Type: Simple): Values: None
//   4. --data (Type: Long Option): Values: [a, b]
//   5. --verbose (Type: Long Option): Values: None
*/
