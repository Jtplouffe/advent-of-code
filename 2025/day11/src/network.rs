use std::collections::HashMap;

pub struct Network<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Network<'a> {
    pub fn different_path_count(
        &self,
        start: &str,
        end: &str,
        required_device_visitation: Option<(&str, &str)>,
    ) -> usize {
        let mut cache = HashMap::new();

        match required_device_visitation {
            None => self.count_paths(start, end, &mut cache),
            Some((first_required_device, second_required_device)) => {
                let first = self.count_paths(start, first_required_device, &mut cache)
                    * self.count_paths(first_required_device, second_required_device, &mut cache)
                    * self.count_paths(second_required_device, end, &mut cache);
                let second = self.count_paths(start, second_required_device, &mut cache)
                    * self.count_paths(second_required_device, first_required_device, &mut cache)
                    * self.count_paths(first_required_device, end, &mut cache);

                first + second
            }
        }
    }

    fn count_paths(
        &self,
        current: &'a str,
        destination: &'a str,
        cache: &mut HashMap<(&'a str, &'a str), usize>,
    ) -> usize {
        if current == destination {
            return 1;
        }

        if let Some(&cached_result) = cache.get(&(current, destination)) {
            return cached_result;
        }

        let mut count = 0;
        if let Some(connections) = self.connections.get(current) {
            for next in connections {
                count += self.count_paths(next, destination, cache);
            }
        }

        cache.insert((current, destination), count);
        count
    }
}

impl<'a> From<&'a str> for Network<'a> {
    fn from(value: &'a str) -> Self {
        let connections = value
            .lines()
            .map(|line| {
                let (source, destinations) = line.split_once(": ").expect("Invalid line");

                let destinations: Vec<_> = destinations.split(" ").collect();

                (source, destinations)
            })
            .collect();

        Self { connections }
    }
}
