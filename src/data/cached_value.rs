use super::AppData;

pub struct CachedValue<T> {
    cache: Option<(u32, T)>,
}

impl<T> Default for CachedValue<T> {
    fn default() -> Self {
        Self { cache: None }
    }
}

impl<T> CachedValue<T> {
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        self.cache = None;
    }

    pub fn get<F>(&mut self, app_data: &AppData, compute: F) -> &T
    where
        F: Fn(&AppData) -> T,
    {
        let cache_is_good = match &self.cache {
            Some((modification_count, _t)) => *modification_count == app_data.modification_count(),
            None => false,
        };
        if !cache_is_good {
            let t = compute(app_data);
            self.cache = Some((app_data.modification_count(), t));
        }
        &self.cache.as_ref().unwrap().1
    }
}
