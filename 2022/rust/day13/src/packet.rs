use std::cmp::Ordering;

#[derive(Clone, PartialEq, Debug, Ord, Eq)]
pub enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl Packet {
    pub fn is_before(&self, other: &Self) -> bool {
        match self.inner_is_before(other) {
            PacketComparisonResult::Ordered => true,
            PacketComparisonResult::Unordered => false,
            PacketComparisonResult::Next => unreachable!(),
        }
    }

    fn inner_is_before(&self, other: &Self) -> PacketComparisonResult {
        match (self, other) {
            (Self::Value(left_value), Self::Value(right_value)) => {
                left_value.cmp(right_value).into()
            }
            (Self::Value(_), Self::List(_)) => {
                Self::List(vec![self.clone()]).inner_is_before(other)
            }
            (Self::List(_), Self::Value(_)) => {
                self.inner_is_before(&Self::List(vec![other.clone()]))
            }
            (Self::List(left_list), Self::List(right_list)) => {
                for i in 0..left_list.len().max(right_list.len()) {
                    if left_list.len() < i + 1 {
                        return PacketComparisonResult::Ordered;
                    } else if right_list.len() < i + 1 {
                        return PacketComparisonResult::Unordered;
                    }

                    match left_list[i].inner_is_before(&right_list[i]) {
                        PacketComparisonResult::Next => {}
                        result => return result,
                    }
                }

                PacketComparisonResult::Next
            }
        }
    }
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        match s.chars().nth(0).unwrap() {
            '[' => {
                let mut values = Vec::new();
                let mut depth = 0;
                let mut current_value = String::new();

                for c in s[1..s.len() - 1].chars() {
                    match c {
                        '[' => {
                            depth += 1;
                            current_value.push(c);
                        }
                        ']' => {
                            depth -= 1;
                            current_value.push(c);
                        }
                        ',' => {
                            if depth == 0 {
                                values.push(current_value);
                                current_value = String::new();
                            } else {
                                current_value.push(c);
                            }
                        }
                        c => current_value.push(c),
                    }
                }

                if !current_value.is_empty() {
                    values.push(current_value);
                }

                Self::List(
                    values
                        .iter()
                        .map(|value| Packet::from(value.as_str()))
                        .collect(),
                )
            }
            _ => Self::Value(s.parse().unwrap()),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.inner_is_before(other) {
            PacketComparisonResult::Ordered => Some(Ordering::Less),
            PacketComparisonResult::Unordered => Some(Ordering::Greater),
            PacketComparisonResult::Next => unreachable!(),
        }
    }
}

enum PacketComparisonResult {
    Ordered,
    Unordered,
    Next,
}

impl From<Ordering> for PacketComparisonResult {
    fn from(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => Self::Ordered,
            Ordering::Greater => Self::Unordered,
            Ordering::Equal => Self::Next,
        }
    }
}
