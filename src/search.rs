use bk_tree::{metrics, BKTree};
use caseless::Caseless;
use std::collections::BTreeMap;
use radix_trie::{Trie, TrieCommon, TrieKey};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SearchIndex<K> {
    index: Trie<String, BTreeMap<K, u64>>,
    bk_tree: BKTree<String>,
}

impl<K: Clone + Ord> SearchIndex<K> {
    pub fn new() -> Self {
        SearchIndex {
            index: Trie::new(),
            bk_tree: BKTree::new(metrics::Levenshtein),
        }
    }

    pub fn add(&mut self, key: K, text: &str, weight: u64) {
        for word in text.unicode_words().map(nfkd_case_fold) {
            self.add_word(key.clone(), word, weight);
        }
    }

    fn add_word(&mut self, key: K, word: String, weight: u64) {
        let count = self.index
            .get_or_insert_with(word.clone(), BTreeMap::new)
            .entry(key).or_insert(0);
        *count += weight;
        self.bk_tree.add(word);
    }

    /// Performs a search with the given query.
    ///
    /// Returns a list of results, paired with their match weight. The results
    /// are sorted such that the best matches come first.
    ///
    /// This method performs automatic spelling correction. If the search query
    /// was corrected, then this new query is returned with the result.
    pub fn query(&self, text: &str) -> (Vec<(K, u64)>, Option<String>) {
        let words: Vec<String> = text.unicode_words().map(nfkd_case_fold).collect();
        let results = self.query_exact(&words);
        if results.is_empty() {
            let words: Vec<&str> = words.iter().map(|word| {
                if let Some((_, new_word)) = self.bk_tree.find(word, 3).min() {
                    new_word
                } else {
                    word
                }.as_ref()
            }).collect();
            let results = self.query_exact(&words);
            let results_is_empty = results.is_empty();  // Get around borrowck
            (results, if results_is_empty { None } else { Some(words.join(" ")) })
        } else {
            (results, None)
        }
    }

    fn query_exact<S: AsRef<str>>(&self, words: &[S]) -> Vec<(K, u64)> {
        // Split text into words
        let mut results = words.iter()
            // Look up each word
            .map(|word| {
                // Match words by prefix so that e.g. "quie" matches "QuietMisdreavus"
                let mut result = BTreeMap::new();
                if let Some(subtrie) = self.index.get_raw_descendant(word.as_ref()) {
                    for (key, count) in subtrie.values().flat_map(|result| result) {
                        *result.entry(key.clone()).or_insert(0) += *count;
                    }
                }
                result
            })
            // Intersect the results for each word
            .fold(None, |uberresult, result| {
                if let Some(mut uberresult) = uberresult {
                    for entry in &mut uberresult {
                        let (uberkey, ubercount): (&K, &mut u64) = entry;
                        *ubercount *= *result.get(uberkey).unwrap_or(&0);
                    }
                    Some(uberresult)
                } else {
                    Some(result)
                }
            })
            .unwrap_or_else(BTreeMap::new)
            .into_iter()
            // Delete the users for which at least one word doesn't appear
            .filter(|&(_, count)| count > 0)
            .collect::<Vec<_>>();
        // Sort by decreasing matchiness
        results.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));
        results
    }
}

// FIXME: https://github.com/michaelsproul/rust_radix_trie/issues/32
trait TrieExt<K, V> {
    fn get_or_insert_with<F>(&mut self, K, F) -> &mut V where F: FnOnce() -> V;
}

impl<K: Clone + TrieKey, V> TrieExt<K, V> for Trie<K, V> {
    fn get_or_insert_with<F>(&mut self, key: K, new: F) -> &mut V where
        F: FnOnce() -> V
    {
        if self.get(&key).is_none() {
            self.insert(key.clone(), new());
        }
        self.get_mut(&key).unwrap()
    }
}

fn nfkd_case_fold(text: &str) -> String {
    text.nfd().default_case_fold().nfkd().default_case_fold().nfkd().collect()
}
