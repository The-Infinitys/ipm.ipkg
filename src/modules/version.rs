use std::fmt::{self};

#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    string: String,
    nums: Vec<u32>,
    separators: Vec<String>,
}
fn serialize_version_str(version_str: &str) -> (Vec<u32>, Vec<String>) {
    let mut numbers = Vec::new();
    let mut separators = Vec::new();
    let mut current_num = String::new();
    let mut current_sep = String::new();

    for c in version_str.chars() {
        if c.is_ascii_digit() {
            // 区切り文字のシーケンスが終わった場合、追加
            if !current_sep.is_empty() {
                separators.push(current_sep.clone());
                current_sep.clear();
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
        separators.push(current_sep);
    }

    (numbers, separators)
}

impl Default for Version {
    fn default() -> Self {
        return Version {
            string: "".to_string(),
            nums: Vec::new(),
            separators: Vec::new(),
        };
    }
}

impl Version {
    pub fn from_str(version_str: &str) -> Version {
        let (nums, separators) = serialize_version_str(version_str);
        Version {
            string: version_str.to_string(),
            nums,
            separators,
        }
    }
    fn insert_to_range_data(
        &self,
        range_data: Option<RangeData>,
        insert_type: VersionRangeInsertType,
    ) -> Option<RangeData> {
        range_data.map(|mut range_data| match insert_type {
            VersionRangeInsertType::StrictlyEarlier => {
                if range_data
                    .exactly_equal
                    .as_ref()
                    .map_or(false, |v| v >= self)
                    || range_data
                        .later_or_equal
                        .as_ref()
                        .map_or(false, |v| v >= self)
                    || range_data
                        .strictly_later
                        .as_ref()
                        .map_or(false, |v| v >= self)
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
                if range_data
                    .exactly_equal
                    .as_ref()
                    .map_or(false, |v| v > self)
                    || range_data
                        .later_or_equal
                        .as_ref()
                        .map_or(false, |v| v > self)
                    || range_data
                        .strictly_later
                        .as_ref()
                        .map_or(false, |v| v > self)
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
                if range_data
                    .exactly_equal
                    .as_ref()
                    .map_or(false, |v| v != self)
                {
                    return None;
                }
                range_data.exactly_equal = Some(self.clone());
                Some(range_data)
            }
            VersionRangeInsertType::LaterOrEqual => {
                if range_data
                    .exactly_equal
                    .as_ref()
                    .map_or(false, |v| v < self)
                    || range_data
                        .strictly_earlier
                        .as_ref()
                        .map_or(false, |v| v < self)
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
                if range_data
                    .exactly_equal
                    .as_ref()
                    .map_or(false, |v| v <= self)
                    || range_data
                        .earlier_or_equal
                        .as_ref()
                        .map_or(false, |v| v <= self)
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[derive(Clone, Copy, Debug)]
enum VersionRangeInsertType {
    StrictlyEarlier,
    EarlierOrEqual,
    ExactlyEqual,
    LaterOrEqual,
    StrictlyLater,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let min_len = self.nums.len().min(other.nums.len());
        for i in 0..min_len {
            match self.nums[i].cmp(&other.nums[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return Some(ord),
            }
        }
        Some(self.nums.len().cmp(&other.nums.len()))
    }
}

#[derive(Clone, Debug)]
pub struct VersionRange {
    _range_data: Option<RangeData>,
    string: String,
}

#[derive(Clone, Debug)]
struct RangeData {
    strictly_earlier: Option<Version>,
    earlier_or_equal: Option<Version>,
    exactly_equal: Option<Version>,
    later_or_equal: Option<Version>,
    strictly_later: Option<Version>,
}

impl VersionRange {
    pub fn to_string(&self) -> String {
        self.string.clone()
    }

    pub fn from_str(range_str: &str) -> VersionRange {
        let mut range_data = Some(RangeData {
            strictly_earlier: None,
            earlier_or_equal: None,
            exactly_equal: None,
            later_or_equal: None,
            strictly_later: None,
        });

        for part in range_str.split(',').map(str::trim) {
            let parts: Vec<&str> = part.split_whitespace().collect();
            if parts.len() == 1 {
                let version_str = parts[0];
                if version_str == "*" {
                    continue;
                } else {
                    let version = Version::from_str(version_str);
                    range_data = version
                        .insert_to_range_data(range_data, VersionRangeInsertType::ExactlyEqual);
                }
            } else if parts.len() == 2 {
                let symbol = parts[0];
                let version_str = parts[1];
                let version = Version::from_str(version_str);
                let insert_type = match symbol {
                    ">>" | ">" => VersionRangeInsertType::StrictlyLater,
                    ">=" => VersionRangeInsertType::LaterOrEqual,
                    "=" | "==" => VersionRangeInsertType::ExactlyEqual,
                    "<=" => VersionRangeInsertType::EarlierOrEqual,
                    "<<" | "<" => VersionRangeInsertType::StrictlyEarlier,
                    _ => {
                        eprintln!("Invalid relation: {}", symbol);
                        continue;
                    }
                };
                range_data = version.insert_to_range_data(range_data, insert_type);
            } else {
                eprintln!("Invalid range format: {}", part);
            }
        }

        VersionRange {
            _range_data: range_data.clone(),
            string: range_data
                .as_ref()
                .map_or_else(|| "".to_string(), |rd| rd.to_string()),
        }
    }

    pub fn compare(&self, version: &Version) -> bool {
        self._range_data.as_ref().map_or(false, |range_data| {
            if let Some(v) = &range_data.strictly_earlier {
                if version >= v {
                    return false;
                }
            }
            if let Some(v) = &range_data.earlier_or_equal {
                if version > v {
                    return false;
                }
            }
            if let Some(v) = &range_data.exactly_equal {
                if version != v {
                    return false;
                }
            }
            if let Some(v) = &range_data.later_or_equal {
                if version < v {
                    return false;
                }
            }
            if let Some(v) = &range_data.strictly_later {
                if version <= v {
                    return false;
                }
            }
            true
        })
    }
}

impl Default for VersionRange {
    fn default() -> Self {
        return VersionRange {
            _range_data: None,
            string: "*".to_string(),
        };
    }
}
impl RangeData {
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some(v) = &self.strictly_earlier {
            parts.push(format!("< {}", v.string));
        }
        if let Some(v) = &self.earlier_or_equal {
            parts.push(format!("<= {}", v.string));
        }
        if let Some(v) = &self.exactly_equal {
            parts.push(format!("== {}", v.string));
        }
        if let Some(v) = &self.later_or_equal {
            parts.push(format!(">= {}", v.string));
        }
        if let Some(v) = &self.strictly_later {
            parts.push(format!("> {}", v.string));
        }
        if parts.is_empty() {
            "*".to_string()
        } else {
            parts.join(", ")
        }
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
pub fn test() {
    let version1 = Version::from_str("1.2.3");
    let version2 = Version::from_str("1.2.2-build-4");
    let version3 = Version::from_str("2.123.12");
    println!("version2 == version1: {}", version1 == version2);
    println!("version2 >= version1: {}", version1 >= version2);
    println!("version3 < version1: {}", version3 < version1);
    let range1 = VersionRange::from_str("< 2.0, > 1.1.3-build-1");
    println!("Range1: {:?}", &range1);
    println!("In Range1, version1: {}", range1.compare(&version1));
}
