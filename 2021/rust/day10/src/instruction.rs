#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    Parenthesis,
    ClosingParenthesis,
    Bracket,
    ClosingBracket,
    Brace,
    ClosingBrace,
    Chevron,
    ClosingChevron,
}

impl Instruction {
    pub fn is_opening(&self) -> bool {
        match self {
            Self::Parenthesis | Self::Bracket | Self::Brace | Self::Chevron => true,
            _ => false,
        }
    }

    pub fn opposite(&self) -> Instruction {
        match self {
            Self::Parenthesis => Self::ClosingParenthesis,
            Self::ClosingParenthesis => Self::Parenthesis,
            Self::Bracket => Self::ClosingBracket,
            Self::ClosingBracket => Self::Bracket,
            Self::Brace => Self::ClosingBrace,
            Self::ClosingBrace => Self::Brace,
            Self::Chevron => Self::ClosingChevron,
            Self::ClosingChevron => Self::Chevron,
        }
    }

    pub fn syntax_error_score(&self) -> u32 {
        match self {
            Self::ClosingParenthesis => 3,
            Self::ClosingBracket => 57,
            Self::ClosingBrace => 1197,
            Self::ClosingChevron => 25137,
            _ => 0,
        }
    }

    pub fn autocomplete_score(&self) -> u32 {
        match self {
            Self::ClosingParenthesis => 1,
            Self::ClosingBracket => 2,
            Self::ClosingBrace => 3,
            Self::ClosingChevron => 4,
            _ => 0,
        }
    }
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '(' => Self::Parenthesis,
            ')' => Self::ClosingParenthesis,
            '[' => Self::Bracket,
            ']' => Self::ClosingBracket,
            '{' => Self::Brace,
            '}' => Self::ClosingBrace,
            '<' => Self::Chevron,
            '>' => Self::ClosingChevron,
            _ => panic!("Unknown instruction: {}", c),
        }
    }
}
