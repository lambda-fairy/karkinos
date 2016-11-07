use caseless::Caseless;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SearchIndex<K> {
    index: BTreeMap<String, BTreeMap<K, u64>>,
}

impl<K: Clone + Ord> SearchIndex<K> {
    pub fn new() -> Self {
        SearchIndex {
            index: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, key: K, text: &str) {
        for word in text.unicode_words().map(nfkd_case_fold) {
            self.add_word(key.clone(), word);
        }
    }

    fn add_word(&mut self, key: K, word: String) {
        let count = self.index
            .entry(word).or_insert_with(BTreeMap::new)
            .entry(key).or_insert(0);
        *count += 1;
    }

    #[allow(dead_code)]  // ... for now!
    pub fn remove<Q: ?Sized>(&mut self, key: &Q, text: &str) where
        K: Borrow<Q>, Q: Ord
    {
        for word in text.unicode_words().map(nfkd_case_fold) {
            self.remove_word(key, &word);
        }
    }

    fn remove_word<Q: ?Sized>(&mut self, key: &Q, word: &str) where
        K: Borrow<Q>, Q: Ord
    {
        let mut zero = false;
        if let Some(count) = self.index.get_mut(word).and_then(|m| m.get_mut(key)) {
            *count -= 1;
            zero = *count == 0;
        }
        if zero {
            self.index.get_mut(word).unwrap().remove(key);
        }
    }

    pub fn query(&self, text: &str) -> Vec<(K, u64)> {
        // Split text into words
        let mut results = text.unicode_words().map(nfkd_case_fold)
            // Look up each word
            .filter_map(|word| self.index.get(&word))
            // Intersect the results for each word
            .fold(None, |uberresult, result| {
                if let Some(mut uberresult) = uberresult {
                    for entry in &mut uberresult {
                        let (uberkey, ubercount): (&K, &mut u64) = entry;
                        *ubercount *= *result.get(uberkey).unwrap_or(&0);
                    }
                    Some(uberresult)
                } else {
                    Some(result.clone())
                }
            })
            .unwrap_or_else(|| BTreeMap::new())
            .into_iter()
            // Delete the users for which at least one word doesn't appear
            .filter(|&(_, count)| count > 0)
            .collect::<Vec<_>>();
        // Sort by decreasing matchiness
        results.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));
        results
    }
}

fn nfkd_case_fold(text: &str) -> String {
    text.nfd().default_case_fold().nfkd().default_case_fold().nfkd().collect()
}
