use std::collections::HashMap;
use std::fmt;
use std::string::String;

use crate::solver::Solver;
mod input;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Ingredient {
    qty: isize,
    name: String,
}
impl Ingredient {
    fn new(ing_str: &str) -> Ingredient {
        let mut ing_iter = ing_str.trim().split(" ");
        let qty: isize = ing_iter.next().unwrap().parse::<isize>().unwrap();
        let name = ing_iter.next().unwrap();
        Ingredient {
            qty: qty,
            name: name.to_string(),
        }
    }
}
type Ingredients = Vec<Ingredient>;
type Reactions = HashMap<Ingredient, Ingredients>;
type Requirements = HashMap<String, isize>;

pub struct Problem;
impl Solver for Problem {
    type Ans1 = isize;
    type Ans2 = isize;

    fn solution1(&self) -> Self::Ans1 {
        let nf = Nanofactory::new(input::INPUT);
        nf.est_ore_for_fuel(1)
    }

    fn solution2(&self) -> Self::Ans2 {
        let nf = Nanofactory::new(input::INPUT);
        nf.max_fuel_for_trillion_ore()
    }
}

struct Nanofactory {
    reacts: Reactions,
}
impl Nanofactory {
    fn new(reactions: &str) -> Nanofactory {
        Nanofactory {
            reacts: parse_reactions(reactions),
        }
    }

    fn est_ore_for_fuel(&self, n: isize) -> isize {
        let mut reqs: Requirements = Requirements::new();
        reqs.insert("FUEL".to_string(), n);
        while let Some(&_cnt) = reqs
            .iter()
            .find(|(&ref name, &cnt)| name != "ORE" && cnt > 0)
            .map(|(_, cnt)| cnt)
        {
            let ing = reqs
                .iter()
                .find(|(&ref name, &cnt)| name != "ORE" && cnt > 0)
                .map(|(name, _)| name.clone())
                .unwrap();
            let prod = self
                .reacts
                .iter()
                .find(|(prod, _)| prod.name == ing)
                .map(|(prod, _)| prod.clone())
                .unwrap();

            let needed = reqs[&ing];
            let qty = (needed as f64 / prod.qty as f64).ceil() as isize;

            *reqs.get_mut(&ing).unwrap() -= qty * prod.qty;

            for ing in self.reacts.get(&prod).unwrap() {
                *reqs.entry(ing.name.clone()).or_insert(0) += qty * ing.qty;
            }
        }
        *reqs.get("ORE").unwrap_or(&0) as isize
    }

    fn max_fuel_for_trillion_ore(&self) -> isize {
        const TRILLION: isize = 1_000_000_000_000;
        let mut low = 0;
        let mut high = TRILLION;

        let mut exp_max_fuel = 1;
        while self.est_ore_for_fuel(exp_max_fuel) <= TRILLION {
            low = exp_max_fuel;
            exp_max_fuel *= 2;
            high = exp_max_fuel;
        }

        while low + 1 < high {
            let pivot = (low + high) / 2;
            let ore_req = self.est_ore_for_fuel(pivot);

            if ore_req <= TRILLION {
                low = pivot;
            } else {
                high = pivot;
            }
        }

        low
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.qty, self.name)
    }
}

impl fmt::Display for Nanofactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for (prod, ings) in self.reacts.iter() {
            let ings_str = ings.iter().map(|i| i.to_string() + ",").collect::<String>();
            writeln!(f, "{} => {}", ings_str.trim_end_matches(","), prod)?;
        }
        Ok(())
    }
}

fn parse_reactions(reactions: &str) -> Reactions {
    let mut reacts: Reactions = Reactions::new();
    let _: Vec<_> = reactions
        .lines()
        .skip(1)
        .map(|reaction| {
            let mut iter = reaction.split("=>");
            let ingredients_str = iter.next().unwrap();
            let product = iter.next().unwrap();
            let ingredients: Ingredients = ingredients_str
                .split(",")
                .into_iter()
                .map(|ing| Ingredient::new(ing.trim()))
                .collect();
            reacts
                .entry(Ingredient::new(product))
                .or_insert(ingredients);
        })
        .collect();
    reacts
}

#[cfg(test)]
mod tests {
    use crate::solutions::day14::*;

    #[test]
    fn test_short_input() {
        let nf = Nanofactory::new(INPUT1);
        assert_eq!(31, nf.est_ore_for_fuel(1));
    }

    #[test]
    fn test_longer_input() {
        let nf = Nanofactory::new(INPUT2);
        assert_eq!(13312, nf.est_ore_for_fuel(1));
    }

    #[test]
    fn test_medium_input() {
        let nf = Nanofactory::new(INPUT3);
        assert_eq!(180697, nf.est_ore_for_fuel(1));
    }

    #[test]
    fn test_longest_input() {
        let nf = Nanofactory::new(INPUT4);
        assert_eq!(2210736, nf.est_ore_for_fuel(1));
    }

    #[test]
    fn test_max_fuel_for_13312_ore() {
        let nf = Nanofactory::new(INPUT2);
        assert_eq!(82892753, nf.max_fuel_for_trillion_ore());
    }

    #[test]
    fn test_max_fuel_for_180697_ore() {
        let nf = Nanofactory::new(INPUT3);
        assert_eq!(5586022, nf.max_fuel_for_trillion_ore());
    }

    #[test]
    fn test_max_fuel_for_2210736_ore() {
        let nf = Nanofactory::new(INPUT4);
        assert_eq!(460664, nf.max_fuel_for_trillion_ore());
    }

    // 31
    const INPUT1: &str = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    // 13312 ORE for 1 FUEL
    const INPUT2: &str = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    // 180697 ORE for 1 FUEL
    const INPUT3: &str = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    // 2210736 ORE for 1 FUEL
    const INPUT4: &str = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
}
