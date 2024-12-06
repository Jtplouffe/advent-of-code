use std::collections::{HashMap, VecDeque};

use crate::update::Update;

pub struct SafetyProtocol {
    rules: HashMap<usize, Vec<usize>>,
}

impl SafetyProtocol {
    pub fn new(raw_rules: &str) -> Self {
        let mut rules = HashMap::<usize, Vec<usize>>::new();

        for raw_rule in raw_rules.lines() {
            let (before, after) = raw_rule.split_once('|').unwrap();

            let (before, after) = (before.parse().unwrap(), after.parse().unwrap());

            rules.entry(before).or_default().push(after);
        }

        Self { rules }
    }

    pub fn is_update_safe(&self, update: &Update) -> bool {
        self.is_page_order_safe(&update.pages)
    }

    // There is probably a way more efficient way of doing this, but it works
    pub fn fix_update(&self, update: &Update) -> Update {
        let mut queue = VecDeque::new();
        queue.push_back(Vec::new());

        while let Some(pages) = queue.pop_front() {
            if pages.len() == update.pages.len() {
                return Update::new(pages);
            }

            for &next_page in &update.pages {
                if pages.contains(&next_page) {
                    continue;
                }

                if !self.is_page_safe_to_push(next_page, &pages) {
                    continue;
                }

                let mut new_pages = pages.clone();
                new_pages.push(next_page);

                queue.push_back(new_pages);
            }
        }

        unreachable!("Update is unfixable")
    }

    fn is_page_order_safe(&self, pages: &[usize]) -> bool {
        let mut page_processed = Vec::with_capacity(pages.len());

        for &page in pages {
            let before_pages = match self.rules.get(&page) {
                Some(pages) => pages,
                _ => {
                    page_processed.push(page);
                    continue;
                }
            };

            for before_page in before_pages {
                if page_processed.contains(before_page) {
                    return false;
                }
            }

            page_processed.push(page);
        }

        true
    }

    /// Same as `is_page_order_safe`, but only check the last page
    fn is_page_safe_to_push(&self, page: usize, pages: &[usize]) -> bool {
        if pages.is_empty() {
            return true;
        }

        if let Some(before_pages) = self.rules.get(&page) {
            for prior_page in pages {
                if before_pages.contains(prior_page) {
                    return false;
                }
            }
        }

        true
    }
}
