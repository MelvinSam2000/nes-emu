use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

use crate::cpu;
use crate::Nes;

#[test]
fn nes_test() -> Result<()> {
    const NES_TEST_FILE: &str = "test-files/nestest.nes";
    const NES_TEST_LOG: &str = "test-files/nestest.log";

    let mut nes = Nes::default();

    let nestest_rom = fs::read(NES_TEST_FILE)?;
    let mut nestest_log = BufReader::new(File::open(NES_TEST_LOG)?)
        .lines()
        .map(|line| line.map_err(|_| anyhow!("Cannot read lines of nes test log")))
        .collect::<Result<Vec<String>>>()?;
    let re = Regex::new(r"\s+")?;
    nestest_log = nestest_log
        .iter()
        .map(|line| {
            let line = line.replace("\t", " ");
            re.replace_all(&line, " ").to_string()
        })
        .collect::<Vec<String>>();

    nes.load(&nestest_rom)?;
    nes.reset()?;
    nes.cpu.pc = 0xc000;
    cpu::step(&mut nes)?;

    for i in 0..6350 {
        let inst = cpu::step(&mut nes)?;
        let inst = inst.replace("\t", " ");
        let inst = re.replace_all(&inst, " ").to_string();
        assert_eq!(inst, nestest_log[i], "Unequal at line {i}");
    }

    Ok(())
}
