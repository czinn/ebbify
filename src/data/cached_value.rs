use super::AppData;

pub struct CachedValue<F, T>
where
    F: Fn(&AppData) -> T,
{
    compute: F,
    cache: Option<(u32, T)>,
}

impl<F, T> CachedValue<F, T>
where
    F: Fn(&AppData) -> T,
{
    pub fn create(compute: F) -> Self {
        Self {
            compute,
            cache: None,
        }
    }

    pub fn update(&mut self, compute: F) {
        self.compute = compute;
        self.cache = None;
    }

    pub fn get(&mut self, app_data: &AppData) -> &T {
        let cache_is_good = match &self.cache {
            Some((modification_count, _t)) => *modification_count == app_data.modification_count(),
            None => false,
        };
        if !cache_is_good {
            let t = (self.compute)(app_data);
            self.cache = Some((app_data.modification_count(), t));
        }
        &self.cache.as_ref().unwrap().1
    }
}
