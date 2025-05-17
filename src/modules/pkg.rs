//! このモジュールは、カスタムのバージョン構造体とバージョン範囲構造体を定義し、
//! 文字列からのパース、比較、および表示機能を提供します。
//! 特に、バージョン文字列を数値部分と区切り文字に分解して扱い、
//! 複数の条件を組み合わせたバージョン範囲を表現・評価する機能を含みます。

use std::{fmt, fmt::Display, str::FromStr};

/// カスタムバージョン番号を表す構造体。
///
/// バージョン文字列を元の形式 (`string`)、数値部分 (`nums`)、
/// および区切り文字 (`separators`) に分解して保持します。
/// これにより、柔軟なパースと比較が可能になります。
#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    /// バージョン番号の元の文字列形式。
    string: String,
    /// バージョン番号の数値部分を格納したベクタ。
    /// 例: "1.2.3-beta.4" の場合、`nums` は `[1, 2, 3, 4]` になります。
    nums: Vec<u32>,
    /// バージョン番号の区切り文字を格納したベクタ。
    /// 例: "1.2.3-beta.4" の場合、`separators` は `[".", ".", "-", "beta."]` になります。
    separators: Vec<String>,
}

impl Default for Version {
    /// `Version`のデフォルトインスタンスを作成します。
    ///
    /// デフォルトは "1.0.0" です。パースに失敗することはないと仮定しています。
    fn default() -> Self {
        // デフォルトの "1.0.0" は有効なバージョン文字列としてFromStrでパースされるはずです。
        Version::from_str("1.0.0").unwrap()
    }
}

/// バージョン文字列を数値部分と区切り文字に分解します。
///
/// 数字の連続と非数字の連続を交互に区切り、それぞれをパースしてベクタに格納します。
///
/// # 引数
///
/// * `version_str`: 分解するバージョン番号の文字列。
///
/// # 戻り値
///
/// 数値部分のベクタと区切り文字（非数字文字列）のベクタのタプル。
fn serialize_version_str(version_str: &str) -> (Vec<u32>, Vec<String>) {
    let mut numbers = Vec::new();
    let mut separators = Vec::new();
    let mut current_num = String::new();
    let mut current_sep = String::new();

    for c in version_str.chars() {
        if c.is_ascii_digit() {
            // 区切り文字のシーケンスが終わった場合、追加
            if !current_sep.is_empty() {
                // Efficiency: Move the string instead of cloning
                separators.push(current_sep);
                current_sep = String::new(); // Create a new empty string for the next separator
            }
            // 数字を蓄積
            current_num.push(c);
        } else {
            // 数字のシーケンスが終わった場合、追加
            if !current_num.is_empty() {
                if let Ok(num) = current_num.parse::<u32>() {
                    numbers.push(num);
                }
                current_num.clear();
            }
            // 非数字を蓄積
            current_sep.push(c);
        }
    }

    // 残りの数字または区切り文字を追加
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<u32>() {
            numbers.push(num);
        }
    }
    if !current_sep.is_empty() {
        // Efficiency: Move the last separator string
        separators.push(current_sep);
    }

    (numbers, separators)
}

impl FromStr for Version {
    /// 文字列を`Version`構造体にパースする際のエラー型。
    type Err = String;

    /// 文字列スライスから`Version`インスタンスを作成します。
    ///
    /// 文字列を数値と区切り文字に分解し、構造体に格納します。
    /// 数値部分が一つもない場合はエラーを返します。
    ///
    /// # 引数
    ///
    /// * `s`: パースするバージョン番号の文字列スライス。
    ///
    /// # 戻り値
    ///
    /// パースに成功した場合は`Ok(Version)`、失敗した場合は`Err(String)`。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nums, separators) = serialize_version_str(s);
        if nums.is_empty() {
            return Err("There is no values for Version struct.".to_string());
        }
        Ok(Version {
            string: s.to_string(),
            nums,
            separators,
        })
    }
}

impl Version {
    /// このバージョンを指定されたバージョン範囲データに挿入し、制約を更新します。
    ///
    /// 新しい制約が既存の制約と矛盾する場合、結果は `None` になります。
    ///
    /// # 引数
    ///
    /// * `range_data`: 現在のバージョン範囲データを含む`Option`。`None` の場合は何もしません。
    /// * `insert_type`: 挿入するバージョンの関係性タイプ（>, >=, =, <=, <）。
    ///
    /// # 戻り値
    ///
    /// 更新されたバージョン範囲データを含む`Option`。挿入により範囲が無効になった場合は`None`。
    fn insert_to_range_data(
        &self,
        range_data: Option<RangeData>,
        insert_type: VersionRangeInsertType,
    ) -> Option<RangeData> {
        range_data.map(|mut range_data| match insert_type {
            VersionRangeInsertType::StrictlyEarlier => {
                if range_data.exactly_equal.as_ref().is_some_and(|v| v >= self)
                    || range_data
                        .later_or_equal
                        .as_ref()
                        .is_some_and(|v| v >= self)
                    || range_data
                        .strictly_later
                        .as_ref()
                        .is_some_and(|v| v >= self)
                {
                    return None;
                }
                if let Some(check_ver) = &range_data.earlier_or_equal {
                    if check_ver >= self {
                        range_data.earlier_or_equal = None;
                        range_data.strictly_earlier = Some(self.clone());
                    }
                }
                if let Some(check_ver) = &range_data.strictly_earlier {
                    if check_ver > self {
                        range_data.strictly_earlier = Some(self.clone());
                    }
                }
                Some(range_data)
            }
            VersionRangeInsertType::EarlierOrEqual => {
                if range_data.exactly_equal.as_ref().is_some_and(|v| v > self)
                    || range_data.later_or_equal.as_ref().is_some_and(|v| v > self)
                    || range_data.strictly_later.as_ref().is_some_and(|v| v > self)
                {
                    return None;
                }
                if let Some(check_ver) = &range_data.earlier_or_equal {
                    if check_ver > self {
                        range_data.earlier_or_equal = Some(self.clone());
                    }
                } else {
                    range_data.earlier_or_equal = Some(self.clone());
                }
                Some(range_data)
            }
            VersionRangeInsertType::ExactlyEqual => {
                if range_data.exactly_equal.as_ref().is_some_and(|v| v != self) {
                    return None;
                }
                range_data.exactly_equal = Some(self.clone());
                Some(range_data)
            }
            VersionRangeInsertType::LaterOrEqual => {
                if range_data.exactly_equal.as_ref().is_some_and(|v| v < self)
                    || range_data
                        .strictly_earlier
                        .as_ref()
                        .is_some_and(|v| v < self)
                {
                    return None;
                }
                if let Some(check_ver) = &range_data.later_or_equal {
                    if check_ver < self {
                        range_data.later_or_equal = Some(self.clone());
                    }
                } else {
                    range_data.later_or_equal = Some(self.clone());
                }
                Some(range_data)
            }
            VersionRangeInsertType::StrictlyLater => {
                if range_data.exactly_equal.as_ref().is_some_and(|v| v <= self)
                    || range_data
                        .earlier_or_equal
                        .as_ref()
                        .is_some_and(|v| v <= self)
                {
                    return None;
                }
                if let Some(check_ver) = &range_data.later_or_equal {
                    if check_ver <= self {
                        range_data.later_or_equal = None;
                        range_data.strictly_later = Some(self.clone());
                    }
                }
                if let Some(check_ver) = &range_data.strictly_later {
                    if check_ver < self {
                        range_data.strictly_later = Some(self.clone());
                    }
                } else {
                    range_data.strictly_later = Some(self.clone());
                }
                Some(range_data)
            }
        })?
    }
}

impl fmt::Display for Version {
    /// `Version`を元の文字列形式でフォーマットします。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// バージョン範囲の挿入タイプを表す列挙型。
#[derive(Clone, Copy, Debug)]
enum VersionRangeInsertType {
    /// より厳密に小さい (<)
    StrictlyEarlier,
    /// 以下 (<=)
    EarlierOrEqual,
    /// 等しい (== または =)
    ExactlyEqual,
    /// 以上 (>=)
    LaterOrEqual,
    /// より厳密に大きい (>, >>)
    StrictlyLater,
}

impl PartialOrd for Version {
    /// 別の`Version`との比較を行います。
    ///
    /// 数値部分を左から順に比較し、最初の異なる数値で大小を判断します。
    /// 数値部分が同じ長さまで全て等しい場合は、数値部分が多い方が大きいと判断します。
    /// 区切り文字は比較には使用されません。
    ///
    /// 例: 1.2.3 と 1.2.2 は、3 > 2 なので 1.2.3 > 1.2.2
    /// 例: 1.2 と 1.2.0 は、長さが異なるため 1.2.0 > 1.2 と判断されます。
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let min_len = self.nums.len().min(other.nums.len());
        for i in 0..min_len {
            match self.nums[i].cmp(&other.nums[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return Some(ord),
            }
        }
        // 数値部分が等しい場合、数値部分の長さで比較
        Some(self.nums.len().cmp(&other.nums.len()))
    }
}

/// 複数のバージョン制約の組み合わせを表す構造体。
///
/// 内部的に`RangeData`を保持し、様々な範囲指定（例: ">= 1.0, < 2.0"）を表現します。
#[derive(Clone, Debug, Default)]
pub struct VersionRange {
    /// バージョン範囲の具体的な制約データを含むOption。
    /// `None`の場合、制約がない（全てのバージョンが一致する）ことを意味します（例: "*"）。
    _range_data: Option<RangeData>,
}

/// バージョン範囲の境界値を格納する内部構造体。
#[derive(Clone, Debug)]
struct RangeData {
    /// 許容される最も新しいバージョン（含まない） (例: < 2.0 の 2.0)。
    strictly_earlier: Option<Version>,
    /// 許容される最も新しいバージョン（含む） (例: <= 1.5 の 1.5)。
    earlier_or_equal: Option<Version>,
    /// 厳密に一致する必要があるバージョン (例: == 1.2.3 の 1.2.3)。
    exactly_equal: Option<Version>,
    /// 許容される最も古いバージョン（含む） (例: >= 1.0 の 1.0)。
    later_or_equal: Option<Version>,
    /// 許容される最も古いバージョン（含まない） (例: > 0.5 の 0.5)。
    strictly_later: Option<Version>,
}

impl FromStr for VersionRange {
    /// 文字列を`VersionRange`構造体にパースする際のエラー型。
    type Err = String;

    /// 文字列スライスから`VersionRange`インスタンスを作成します。
    ///
    /// カンマで区切られた複数のバージョン制約（例: ">= 1.0, < 2.0"）をパースし、
    /// 内部的な`RangeData`構造体を構築します。
    /// "*" は全てのバージョンを許可することを意味します。
    ///
    /// # 引数
    ///
    /// * `s`: パースするバージョン範囲の文字列スライス。
    ///
    /// # 戻り値
    ///
    /// パースに成功した場合は`Ok(VersionRange)`、失敗した場合は`Err(String)`。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut range_data = Some(RangeData {
            strictly_earlier: None,
            earlier_or_equal: None,
            exactly_equal: None,
            later_or_equal: None,
            strictly_later: None,
        });

        for part in s.split(',').map(str::trim) {
            let parts: Vec<&str> = part.split_whitespace().collect();
            if parts.len() == 1 {
                let version_str = parts[0];
                if version_str == "*" {
                    // "*" は制約なしを意味するので、range_data を None にする
                    range_data = None;
                    break; // "*" があれば他の制約は無視
                } else {
                    // "=" と同じ意味として扱う
                    // Original had unwrap(), keeping it as per user's request not to fix errors
                    let version = Version::from_str(version_str).unwrap();
                    range_data = version
                        .insert_to_range_data(range_data, VersionRangeInsertType::ExactlyEqual);
                }
            } else if parts.len() == 2 {
                let symbol = parts[0];
                let version_str = parts[1];
                // Original had unwrap(), keeping it as per user's request not to fix errors
                let version = Version::from_str(version_str).unwrap();
                let insert_type = match symbol {
                    ">>" | ">" => VersionRangeInsertType::StrictlyLater,
                    ">=" => VersionRangeInsertType::LaterOrEqual,
                    "=" | "==" => VersionRangeInsertType::ExactlyEqual,
                    "<=" => VersionRangeInsertType::EarlierOrEqual,
                    "<<" | "<" => VersionRangeInsertType::StrictlyEarlier,
                    _ => {
                        return Err(format!("Invalid relation: {}", symbol));
                    }
                };
                range_data = version.insert_to_range_data(range_data, insert_type);
            } else {
                return Err(format!("Invalid range format: {}", part));
            }
            // 挿入の結果、範囲が無効になった場合のチェックが不足している可能性
        }

        Ok(VersionRange {
            // Efficiency: Removed the clone here. Move the ownership of range_data.
            _range_data: range_data,
        })
    }
}

impl VersionRange {
    /// 与えられた`Version`がこのバージョン範囲内に含まれるかどうかを判定します。
    ///
    /// 内部の`RangeData`に基づき、全ての制約を満たすかどうかをチェックします。
    /// `_range_data`が`None`（つまり "*"）の場合は常に`true`を返します。
    ///
    /// # 引数
    ///
    /// * `version`: 範囲に含まれるか判定する対象のバージョン。
    ///
    /// # 戻り値
    ///
    /// `version`が範囲内に含まれる場合は`true`、そうでない場合は`false`。
    pub fn compare(&self, version: &Version) -> bool {
        // _range_data が None の場合（つまり "*"）は常に true
        self._range_data.as_ref().map_or(true, |range_data| {
            // 各制約に対して、version がその条件を満たさない場合は false を返す
            if let Some(v) = &range_data.strictly_earlier {
                if version >= v {
                    // version >= strictly_earlier_version なら範囲外
                    return false;
                }
            }
            if let Some(v) = &range_data.earlier_or_equal {
                if version > v {
                    // version > earlier_or_equal_version なら範囲外
                    return false;
                }
            }
            if let Some(v) = &range_data.exactly_equal {
                if version != v {
                    // version != exactly_equal_version なら範囲外
                    return false;
                }
            }
            if let Some(v) = &range_data.later_or_equal {
                if version < v {
                    // version < later_or_equal_version なら範囲外
                    return false;
                }
            }
            if let Some(v) = &range_data.strictly_later {
                if version <= v {
                    // version <= strictly_later_version なら範囲外
                    return false;
                }
            }
            true // 全ての制約を満たす場合は true
        })
    }
}

impl Display for VersionRange {
    /// `VersionRange`の表示をフォーマットします。
    ///
    /// 内部の`_range_data`が`None`の場合は"*"と表示します。
    /// `_range_data`がある場合、現在は何も表示しませんが、RangeDataのDisplay実装を使用することで
    /// 具体的な制約を表示できます（※現在の実装ではこのDisplayはRangeDataに委譲していません）。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // RangeDataのDisplay実装は存在するが、VersionRangeのDisplayはそれを使っていない点に注意。
        // RangeDataのDisplayを使うと、解析された制約（例: "< 2.0, >= 1.0"）を表示できる。
        if self._range_data.is_none() {
            write!(f, "*")
        } else {
            // FIXME: 具体的なバージョン範囲の表示が不完全な可能性
            write!(f, "")
        }
    }
}

impl Display for RangeData {
    /// `RangeData`の内容を整形して、人間が読める形式で標準出力（または指定されたフォーマッタ）に書き出します。
    ///
    /// 設定されているバージョン制約（<, <=, ==, >=, >）をカンマ区切りで表示します。
    /// どの制約も設定されていない場合は"*"と表示します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(v) = &self.strictly_earlier {
            parts.push(format!("< {}", v.string));
        }
        if let Some(v) = &self.earlier_or_equal {
            parts.push(format!("<= {}", v.string));
        }
        if let Some(v) = &self.exactly_equal {
            // == は元の文字列形式を表示
            parts.push(format!("== {}", v.string));
        }
        if let Some(v) = &self.later_or_equal {
            parts.push(format!(">= {}", v.string));
        }
        if let Some(v) = &self.strictly_later {
            parts.push(format!("> {}", v.string));
        }
        if parts.is_empty() {
            write!(f, "*")
        } else {
            write!(f, "{}", parts.join(", "))
        }
    }
}

/// バージョンとバージョン範囲に関する簡単なテスト関数。
///
/// いくつかの`Version`と`VersionRange`を作成し、比較や表示を行います。
/// この関数はテストフレームワークではなく、通常の関数として定義されています。
#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let version1 = Version::from_str("1.2.3").unwrap();
        let version2 = Version::from_str("1.2.2-build-4").unwrap();
        let version3 = Version::from_str("2.123.12").unwrap();
        println!("version2 == version1: {}", version1 == version2);
        println!("version2 >= version1: {}", version1 >= version2);
        println!("version3 < version1: {}", version3 < version1);
        let range1 = VersionRange::from_str("< 2.0, > 1.1.3-build-1").unwrap();
        println!("Range1: {:?}", &range1);
        println!("In Range1, version1: {}", range1.compare(&version1));
    }
}
