use chrono::{Datelike, Timelike, Utc as time};
use std::{env, mem::size_of};

use super::{fungetypes::FungeInteger, Program};

pub trait SystemInfoReporter<T: FungeInteger> {
    fn get_env_flags() -> Vec<T>;
    fn get_cell_size() -> Vec<T>;
    fn get_handprint() -> Vec<T>;
    fn get_version() -> Vec<T>;
    fn get_operating_paradigm() -> Vec<T>;
    fn get_path_separator() -> Vec<T>;
    fn get_dimensions(&self) -> Vec<T>;
    fn get_ip_id(&self) -> Vec<T>;
    fn get_team_number(&self) -> Vec<T>;
    fn get_position(&self) -> Vec<T>;
    fn get_delta(&self) -> Vec<T>;
    fn get_storage_offset(&self) -> Vec<T>;
    fn get_least_grid_point(&self) -> Vec<T>;
    fn get_greatest_grid_point(&self) -> Vec<T>;
    fn get_day() -> Vec<T>;
    fn get_second() -> Vec<T>;
    fn get_sstack_size(&self) -> Vec<T>;
    fn get_stack_sizes(&self) -> Vec<T>;
    fn get_cli_args() -> Vec<T>;
    fn get_env_vars() -> Vec<T>;

    fn get_full_report(&self) -> Vec<T>;
}

impl<T: FungeInteger> SystemInfoReporter<T> for Program<T> {
    // 1
    fn get_env_flags() -> Vec<T> {
        let t = 0;
        let i = 0;
        let o = 0;
        let e = 0;
        let unbuffered = 1;
        let flags: u8 = t + (i << 1) + (o << 2) + (e << 3) + (unbuffered << 4);
        vec![T::from(flags).unwrap_or_default()]
    }

    // 2
    fn get_cell_size() -> Vec<T> {
        vec![T::from(size_of::<T>()).unwrap_or_default()]
    }

    // 3
    fn get_handprint() -> Vec<T> {
        vec![T::from(11111996).unwrap_or_default()]
    }

    // 4
    fn get_version() -> Vec<T> {
        let version = env!("CARGO_PKG_VERSION").replace(".", "");
        vec![T::from(version.parse::<i32>().unwrap()).unwrap_or_default()]
    }
    // 5
    fn get_operating_paradigm() -> Vec<T> {
        // TODO: fix this after implementing the '=' instruction
        vec![T::zero()]
    }

    // 6
    fn get_path_separator() -> Vec<T> {
        vec![T::from(std::path::MAIN_SEPARATOR as u8).unwrap_or_default()]
    }

    // 7
    fn get_dimensions(&self) -> Vec<T> {
        // TODO: change that when Unefunge restrictions are implemented for the REPL
        vec![T::from(self.get_position().len()).unwrap_or_default()]
    }

    // 8
    fn get_ip_id(&self) -> Vec<T> {
        // TODO: implement that in concurrency
        vec![T::zero()]
    }

    // 9
    fn get_team_number(&self) -> Vec<T> {
        // TODO: implement something for this?
        vec![T::zero()]
    }

    // 10
    fn get_position(&self) -> Vec<T> {
        let position = self.cursor.position();
        vec![position.0, position.1]
    }

    // 11
    fn get_delta(&self) -> Vec<T> {
        let delta = self.cursor.delta();
        vec![delta.x, delta.y]
    }

    // 12
    fn get_storage_offset(&self) -> Vec<T> {
        let so = self.cursor.storage_offset();
        vec![so.0, so.1]
    }

    // 13
    fn get_least_grid_point(&self) -> Vec<T> {
        self.grid.get_least_point()
    }

    // 14
    fn get_greatest_grid_point(&self) -> Vec<T> {
        self.grid.get_greatest_point()
    }

    // 15
    fn get_day() -> Vec<T> {
        let current_time = time::now();
        let year = current_time.year() - 1900;
        let month = current_time.month() as i32;
        let day = current_time.day() as i32;
        vec![T::from(year * 256 * 256 + month * 256 + day).unwrap_or_default()]
    }

    // 16
    fn get_second() -> Vec<T> {
        let current_time = time::now();
        let hour = current_time.hour();
        let minute = current_time.minute();
        let second = current_time.second();
        vec![T::from(hour * 256 * 256 + minute * 256 + second).unwrap_or_default()]
    }

    // 17
    fn get_sstack_size(&self) -> Vec<T> {
        vec![T::from(self.sstack.get_stacks().len()).unwrap_or_default()]
    }

    // 18
    fn get_stack_sizes(&self) -> Vec<T> {
        self.sstack
            .get_stacks()
            .iter()
            .map(|v| T::from(v.len()).unwrap_or_default())
            .collect()
    }

    // 19
    fn get_cli_args() -> Vec<T> {
        let mut res = vec![];
        // TODO: replace for loop with map
        for arg in env::args().skip(1) {
            let mut arg_str: Vec<T> = arg
                .bytes()
                .map(|b| T::from(b).unwrap_or_default())
                .collect();
            arg_str.push(T::zero());
            res.append(&mut arg_str);
        }
        res.push(T::zero());
        res.push(T::zero());
        res.into_iter().rev().collect()
    }

    // 20
    fn get_env_vars() -> Vec<T> {
        let mut res = vec![];
        for (key, value) in env::vars() {
            res.append(
                &mut key
                    .bytes()
                    .map(|b| T::from(b).unwrap_or_default())
                    .collect(),
            );
            res.push(T::from(61).unwrap_or_default()); // 61 is ASCII '='
            res.append(
                &mut value
                    .bytes()
                    .map(|b| T::from(b).unwrap_or_default())
                    .collect(),
            );
            res.push(T::zero());
        }
        res.push(T::zero());
        res.into_iter().rev().collect()
    }

    fn get_full_report(&self) -> Vec<T> {
        vec![
            Self::get_env_vars(),
            Self::get_cli_args(),
            self.get_stack_sizes(),
            self.get_sstack_size(),
            Self::get_second(),
            Self::get_day(),
            self.get_greatest_grid_point(),
            self.get_least_grid_point(),
            self.get_storage_offset(),
            self.get_delta(),
            self.get_position(),
            self.get_team_number(),
            self.get_ip_id(),
            self.get_dimensions(),
            Self::get_path_separator(),
            Self::get_operating_paradigm(),
            Self::get_version(),
            Self::get_handprint(),
            Self::get_cell_size(),
            Self::get_env_flags(),
        ]
        .concat()
    }
}
