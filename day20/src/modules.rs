use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    pub fn invert(self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Module<'a> {
    Broadcaster(&'a str, Vec<&'a str>),
    FlipFlop(&'a str, Pulse, Vec<&'a str>),
    Conjunction(&'a str, Vec<&'a str>, BTreeMap<&'a str, Pulse>),
}

impl<'a> Module<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            Self::Broadcaster(name, _)
            | Self::FlipFlop(name, _, _)
            | Self::Conjunction(name, _, _) => name,
        }
    }

    pub fn connects_to_module(&self, module_name: &str) -> bool {
        match self {
            Self::Broadcaster(_, connected) => {
                connected.iter().any(|module| *module == module_name)
            }
            Self::FlipFlop(_, _, connected) => {
                connected.iter().any(|module| *module == module_name)
            }
            Self::Conjunction(_, connected, _) => {
                connected.iter().any(|module| *module == module_name)
            }
        }
    }

    pub fn pulse(&mut self, origin: &'a str, pulse: Pulse) -> Vec<(&'a str, &'a str, Pulse)> {
        match self {
            Self::Broadcaster(name, connected) => Self::broadcaster_pulse(name, connected, pulse),
            Self::FlipFlop(name, state, connected) => {
                Self::flip_flop_pulse(name, state, connected, pulse)
            }
            Self::Conjunction(name, connected, state) => {
                Self::conjunction_pulse(name, state, connected, origin, pulse)
            }
        }
    }

    fn broadcaster_pulse(
        name: &'a str,
        connected: &[&'a str],
        pulse: Pulse,
    ) -> Vec<(&'a str, &'a str, Pulse)> {
        connected.iter().map(|conn| (name, *conn, pulse)).collect()
    }

    fn flip_flop_pulse(
        name: &'a str,
        state: &mut Pulse,
        connected: &[&'a str],
        pulse: Pulse,
    ) -> Vec<(&'a str, &'a str, Pulse)> {
        match pulse {
            Pulse::High => vec![],
            Pulse::Low => {
                *state = state.invert();
                connected.iter().map(|conn| (name, *conn, *state)).collect()
            }
        }
    }

    fn conjunction_pulse(
        name: &'a str,
        connections: &mut BTreeMap<&'a str, Pulse>,
        connected: &[&'a str],
        origin: &'a str,
        pulse: Pulse,
    ) -> Vec<(&'a str, &'a str, Pulse)> {
        connections
            .entry(origin)
            .and_modify(|e| *e = pulse)
            .or_insert(pulse);

        let response = if connections.iter().all(|(_, v)| *v == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        connected
            .iter()
            .map(|conn| (name, *conn, response))
            .collect()
    }
}

#[derive(Debug)]
pub struct Network<'a> {
    modules: Vec<Module<'a>>,
}

impl<'a> Network<'a> {
    pub fn pulse(&mut self) -> (u64, u64, bool) {
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut rx_activated = false;

        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((transmitter_name, receiver_name, pulse)) = queue.pop_front() {
            if let Some(receiver) = self
                .modules
                .iter_mut()
                .find(|module| module.name() == receiver_name)
            {
                receiver
                    .pulse(transmitter_name, pulse)
                    .into_iter()
                    .for_each(|new_pulse| queue.push_back(new_pulse));
            };

            if matches!((receiver_name, pulse), ("rx", Pulse::Low)) {
                rx_activated = true;
                break;
            }

            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }
        }

        (low_pulses, high_pulses, rx_activated)
    }
}

impl<'a> TryFrom<&'a str> for Network<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut modules = value
            .lines()
            .map(|line| -> Result<Module, String> {
                if let Some((name, connected_to)) = line.split_once("->") {
                    match name.trim() {
                        module_name @ "broadcaster" => {
                            let connected = connected_to.split(',').map(str::trim).collect();
                            Ok(Module::Broadcaster(module_name, connected))
                        }
                        module_name if module_name.starts_with('%') => {
                            let connected = connected_to.split(',').map(str::trim).collect();
                            Ok(Module::FlipFlop(
                                module_name.trim_start_matches('%'),
                                Pulse::Low,
                                connected,
                            ))
                        }
                        module_name if module_name.starts_with('&') => {
                            let connected = connected_to.split(',').map(str::trim).collect();
                            Ok(Module::Conjunction(
                                module_name.trim_start_matches('&'),
                                connected,
                                BTreeMap::new(),
                            ))
                        }
                        _ => Err(format!("Failed to parse module. '{line}'")),
                    }
                } else {
                    Err(format!("Malformed module. '{line}'"))
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        let conjuctions = modules
            .iter()
            .filter_map(|module| match module {
                Module::Conjunction(name, _, _) => Some(*name),
                _ => None,
            })
            .collect::<Vec<_>>();
        conjuctions.iter().try_for_each(|conjuction_name| {
            let connects_to_conjuction = modules
                .iter()
                .filter(|module| module.connects_to_module(conjuction_name))
                .map(Module::name)
                .collect::<Vec<_>>();
            if let Some(Module::Conjunction(_, _, conjuction_connections)) = modules
                .iter_mut()
                .find(|module| module.name() == *conjuction_name)
            {
                for connected_to_conjuction in connects_to_conjuction {
                    conjuction_connections
                        .entry(connected_to_conjuction)
                        .or_insert(Pulse::Low);
                }
                Ok(())
            } else {
                Err(format!("Could not find conjuction '{conjuction_name}'"))
            }
        })?;
        Ok(Self { modules })
    }
}
