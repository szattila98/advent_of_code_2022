use macros::include_str_arr;

use super::AdventTask;

pub struct ElvenCalories;

impl AdventTask for ElvenCalories {
    type Solution = u32;

    fn get_name(&self) -> &str {
        "Elven Calories"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/elven_calories.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let sum_calories = self.calculate_calories(input);
        let max = sum_calories.iter().max().expect("no calories?");
        *max
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut sum_calories = self.calculate_calories(input);
        sum_calories.sort();
        sum_calories.reverse();
        sum_calories[..3].iter().sum()
    }
}

impl ElvenCalories {
    fn calculate_calories(
        &self,
        input: &[Option<&'static str>],
    ) -> Vec<<ElvenCalories as AdventTask>::Solution> {
        let mut sum_calories = vec![];
        let mut current_sum = 0;
        for line in input {
            current_sum += match line {
                Some(calories) => calories
                    .parse::<<ElvenCalories as AdventTask>::Solution>()
                    .expect("cannot parse calories"),
                None => {
                    sum_calories.push(current_sum);
                    current_sum = 0;
                    continue;
                }
            }
        }
        sum_calories
    }
}
