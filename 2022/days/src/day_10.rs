use crate::DayChallenge;
use utils::read_input;
use utils::Purpose;

pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl Day10 {
    pub fn init(purp: Purpose) -> Self {
        Day10 {
            instructions: parse_input(purp),
        }
    }
}

enum Instruction {
    Noop,
    Addx { v: i16 },
}

struct Cpu {
    X: i16,
    cycles: usize,
    CRT: Vec<Vec<char>>,
}

impl Cpu {
    fn init() -> Self {
        return Cpu {
            X: 1,
            cycles: 0,
            CRT: vec![vec!['.'; 40]; 6],
        };
    }

    fn addx(&mut self, v: i16) {
        self.draw_pixel(self.cycles);
        self.draw_pixel(self.cycles + 1);
        self.cycles += 2;
        self.X += v;
        //self.draw_pixel(self.cycles);
    }

    fn noop(&mut self) {
        self.draw_pixel(self.cycles);
        self.cycles += 1;
    }

    fn execute(&mut self, i: &Instruction) {
        match i {
            Instruction::Noop => self.noop(),
            Instruction::Addx { v } => self.addx(*v),
        }
    }

    fn draw_pixel(&mut self, cycle: usize) {
        let row: usize = (cycle / 40) % 6;
        let col: usize = cycle % 40;
        let pixel = self.CRT.get_mut(row).unwrap().get_mut(col).unwrap();
        if col as i16 >= self.X - 1 && col as i16 <= self.X + 1 {
            *pixel = '#'
        } else {
            *pixel = '.'
        }
    }

    fn get_CRT(&self) -> String {
        self.CRT
            .iter()
            .map(|line| line.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl DayChallenge for Day10 {
    fn part_1(&mut self) -> String {
        let mut cpu = Cpu::init();
        let mut next_relevant_cycle: usize = 20;
        let mut sum_strengths = 0;
        self.instructions.iter().for_each(|i| {
            let prev_val = cpu.X;
            cpu.execute(i);
            if cpu.cycles >= next_relevant_cycle {
                sum_strengths += prev_val * next_relevant_cycle as i16;
                next_relevant_cycle += 40;
            }
        });
        sum_strengths.to_string()
    }

    fn part_2(&mut self) -> String {
        let mut cpu = Cpu::init();
        self.instructions.iter().for_each(|i| {
            cpu.execute(i);
        });
        format!("\n{}", cpu.get_CRT())
    }
}

fn parse_input(purp: Purpose) -> Vec<Instruction> {
    let input = read_input(10, purp);
    input
        .iter()
        .map(|l| {
            let instr_str = l.split_whitespace().collect::<Vec<&str>>();
            match instr_str[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx {
                    v: instr_str[1].parse::<i16>().unwrap(),
                },
                &_ => todo!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day10 {
        Day10::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_10 = init_test();
        assert_eq!(day_10.part_1(), "13140");
    }

    #[test]
    fn part_2_works() {
        let mut day_10 = init_test();
        assert_eq!(
            day_10.part_2(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
