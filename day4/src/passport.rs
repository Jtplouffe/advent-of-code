use std::str::FromStr;
use std::option::Option::Some;

#[derive(PartialEq)]
enum HeightUnit {
    Centimeters,
    Inches,
}

impl FromStr for HeightUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Self::Centimeters),
            "in" => Ok(Self::Inches),
            _ => Err(())
        }
    }
}

enum EyeColors {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Grey),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

pub struct Passport {
    // Birth year
    byr: Option<Result<u16, ()>>,
    // Issue year
    iyr: Option<Result<u16, ()>>,
    // Expiration year
    eyr: Option<Result<u16, ()>>,
    // Height
    hgt: Option<Result<(u16, HeightUnit), ()>>,
    // Hair color
    hcl: Option<Result<String, ()>>,
    // Eye color
    ecl: Option<Result<EyeColors, ()>>,
    // Passport id
    pid: Option<Result<String, ()>>,
    // Country id (optional)
    cid: Option<Result<String, ()>>,
}

impl Passport {
    pub fn has_all_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.has_all_required_fields()
            && self.byr.unwrap().is_ok()
            && self.iyr.unwrap().is_ok()
            && self.eyr.unwrap().is_ok()
            && self.hgt.as_ref().unwrap().is_ok()
            && self.hcl.as_ref().unwrap().is_ok()
            && self.ecl.as_ref().unwrap().is_ok()
            && self.pid.as_ref().unwrap().is_ok()
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_values: Vec<(&str, &str)> = s.split(|c| c == ' ' || c == '\n').map(|l| {
            let kv: Vec<&str> = l.split(":").collect();
            (*kv.get(0).unwrap(), *kv.get(1).unwrap())
        }).collect();

        let byr = if let Some(v) = key_values.iter().find(|i| i.0 == "byr") {
            if let Ok(year) = v.1.parse::<u16>() {
                if year >= 1920 && year <= 2002 {
                    Some(Ok(year))
                } else {
                    Some(Err(()))
                }
            } else {
                Some(Err(()))
            }
        } else {
            None
        };

        let iyr = if let Some(v) = key_values.iter().find(|i| i.0 == "iyr") {
            if let Ok(year) = v.1.parse::<u16>() {
                if year >= 2010 && year <= 2020 {
                    Some(Ok(year))
                } else {
                    Some(Err(()))
                }
            } else {
                Some(Err(()))
            }
        } else {
            None
        };

        let eyr = if let Some(v) = key_values.iter().find(|i| i.0 == "eyr") {
            if let Ok(year) = v.1.parse::<u16>() {
                if year >= 2020 && year <= 2030 {
                    Some(Ok(year))
                } else {
                    Some(Err(()))
                }
            } else {
                Some(Err(()))
            }
        } else {
            None
        };

        let hgt = if let Some(v) = key_values.iter().find(|i| i.0 == "hgt") {
            let value = v.1.chars().take_while(|n| n.is_ascii_digit()).collect::<String>();
            if let Ok(e) = value.parse::<u16>() {
                if let Ok(u) = v.1.replace(&value, "").parse::<HeightUnit>() {
                    if (u == HeightUnit::Centimeters && e >= 150 && e <= 193) || (u == HeightUnit::Inches && e >= 59 && e <= 76) {
                        Some(Ok((e, u)))
                    } else {
                        Some(Err(()))
                    }
                } else {
                    Some(Err(()))
                }
            } else {
                Some(Err(()))
            }
        } else {
            None
        };


        let hcl = if let Some(v) = key_values.iter().find(|i| i.0 == "hcl") {
            if v.1.starts_with(&"#")
                && v.1[1..].chars().all(|c| c.is_ascii_hexdigit())
                && v.1.len() == 7 // 6 byte + '#'
            {
                Some(Ok(v.1.to_string()))
            } else {
                Some(Err(()))
            }
        } else {
            None
        };

        let ecl = if let Some(v) = key_values.iter().find(|i| i.0 == "ecl") {
            Some(v.1.parse::<EyeColors>())
        } else {
            None
        };

        let pid = if let Some(v) = key_values.iter().find(|i| i.0 == "pid") {
            if v.1.len() == 9 && v.1.to_string().into_bytes().iter().all(|c| c.is_ascii_digit()) {
                Some(Ok(v.1.to_string()))
            } else {
                Some(Err(()))
            }
        } else {
            None
        };

        let cid = if let Some(v) = key_values.iter().find(|i| i.0 == "cid") {
            Some(Ok(v.1.to_string()))
        } else {
            None
        };

        Ok(Self { byr, iyr, eyr, hgt, hcl, ecl, pid, cid })
    }
}
