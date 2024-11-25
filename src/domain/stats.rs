use std::collections::HashMap;

pub struct Stats {
    name: String,
    stargazers: i32,
    forks: i64,
    total_contributions: i64,
    languages: HashMap<String, Language>,
    repos: Vec<String>,
    lines_changed: (i64, i64),
    views: i64,
}

impl Stats {
    pub fn new(
        name: String,
        stargazers: i32,
        forks: i64,
        total_contributions: i64,
        languages: HashMap<String, Language>,
        repos: Vec<String>,
        lines_changed: (i64, i64),
        views: i64,
    ) -> Self {
        Self {
            name,
            stargazers,
            forks,
            total_contributions,
            languages,
            repos,
            lines_changed,
            views,
        }
    }

    pub fn builder() -> StatsBuilder {
        StatsBuilder::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stargazers(&self) -> i32 {
        self.stargazers
    }

    pub fn forks(&self) -> i64 {
        self.forks
    }

    pub fn total_contributions(&self) -> i64 {
        self.total_contributions
    }

    pub fn languages(&self) -> &HashMap<String, Language> {
        &self.languages
    }

    pub fn repos(&self) -> &Vec<String> {
        self.repos.as_ref()
    }

    pub fn lines_changed(&self) -> (i64, i64) {
        self.lines_changed
    }

    pub fn views(&self) -> i64 {
        self.views
    }
}

#[derive(Default)]
pub struct StatsBuilder {
    name: Option<String>,
    stargazers: Option<i32>,
    forks: Option<i64>,
    total_contributions: Option<i64>,
    languages: Option<HashMap<String, Language>>,
    repos: Option<Vec<String>>,
    lines_changed: Option<(i64, i64)>,
    views: Option<i64>,
}

impl StatsBuilder {
    pub fn build(self) -> Stats {
        Stats {
            name: self.name.unwrap_or_default(),
            stargazers: self.stargazers.unwrap_or_default(),
            forks: self.forks.unwrap_or_default(),
            total_contributions: self.total_contributions.unwrap_or_default(),
            languages: self.languages.unwrap_or_default(),
            repos: self.repos.unwrap_or_default(),
            lines_changed: self.lines_changed.unwrap_or((0, 0)),
            views: self.views.unwrap_or_default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn stargazers(mut self, stargazers: i32) -> Self {
        self.stargazers = Some(stargazers);
        self
    }

    pub fn forks(mut self, forks: i64) -> Self {
        self.forks = Some(forks);
        self
    }

    pub fn total_contributions(mut self, total_contributions: i64) -> Self {
        self.total_contributions = Some(total_contributions);
        self
    }

    pub fn languages(mut self, languages: HashMap<String, Language>) -> Self {
        self.languages = Some(languages);
        self
    }

    pub fn repos(mut self, repos: Vec<String>) -> Self {
        self.repos = Some(repos);
        self
    }

    pub fn lines_changed(mut self, lines_changed: (i64, i64)) -> Self {
        self.lines_changed = Some(lines_changed);
        self
    }

    pub fn views(mut self, views: i64) -> Self {
        self.views = Some(views);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Language {
    name: String,
    size: i64,
    occurrences: i64,
    color: String,
    proportion: f64,
}

impl Language {
    pub fn new(name: String, size: i64, occurrences: i64, color: String, proportion: f64) -> Self {
        Self {
            name,
            size,
            occurrences,
            color,
            proportion,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn proportion(&self) -> f64 {
        self.proportion
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
