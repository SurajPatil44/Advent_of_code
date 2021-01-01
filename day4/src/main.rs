use std::fs::File;
use std::io::Read;
use core::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Year(u32);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Length {
    Cm(u32),
    In(u32),
    //Unspec(u32)
}

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    byr: Year,
    iyr: Year,
    eyr: Year,
    hgt: Length,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: Option<&'a str>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<Length>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("could not parse {0} : {1}")]
    ParseError(String,String)
}
impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        Ok(Passport {
            byr: self.byr.ok_or(Error::MissingField("byr"))?,
            iyr: self.iyr.ok_or(Error::MissingField("iyr"))?,
            eyr: self.eyr.ok_or(Error::MissingField("eyr"))?,
            hgt: self.hgt.ok_or(Error::MissingField("hgt"))?,
            hcl: self.hcl.ok_or(Error::MissingField("hcl"))?,
            ecl: self.ecl.ok_or(Error::MissingField("ecl"))?,
            pid: self.pid.ok_or(Error::MissingField("pid"))?,
            cid: self.cid,
        })
    }
    fn parse(input: &'a str) -> Result<Self,Error> {
        let mut b: Self = Default::default();

        peg::parser! {
            grammar parser() for str {
                pub(crate) rule root(b: &mut PassportBuilder<'input>)
                    =(field(b) separator()*)* ![_]
                rule separator() = ['\n' | ' ' | '\r' ]
                rule field(b: &mut PassportBuilder<'input>) = byr(b) / iyr(b)
                /eyr(b) /hgt(b) /hcl(b) /ecl(b) /pid(b) /cid(b)
                rule byr(b: &mut PassportBuilder<'input>)
                    ="byr:" year:year((1920..=2002)) { b.byr = Some(year)}

                rule iyr(b: &mut PassportBuilder<'input>)
                    ="iyr:" year:year((2010..=2020)) { b.iyr = Some(year)}

                rule eyr(b: &mut PassportBuilder<'input>)
                    ="eyr:" year:year((2020..=2030)) { b.eyr = Some(year)}

                rule hgt(b: &mut PassportBuilder<'input>)
                    ="hgt:" height:length() {?
                        match &height {
                            Length::Cm(v) if !(150..=193).contains(v) => {
                                Err("bad ht (cm)")
                            },
                            Length::In(v) if !(59..=76).contains(v) => {
                                Err("bad ht in")
                            },
                            _ => {
                                b.hgt = Some(height);
                                Ok(())
                            },
                        }
                    }

                rule pid(b: &mut PassportBuilder<'input>)
                    ="pid:" num:$(['0'..='9']*<9,9>) { b.pid = Some(num)}

                rule cid(b: &mut PassportBuilder<'input>)
                    ="cid:" num:$((!separator()[_])+) { b.cid = Some(num)}

                rule ecl(b: &mut PassportBuilder<'input>)
                    ="ecl:" s:$(("amb"/"blu"/"brn"/"gry"/"grn"/"hzl"/"oth")) { b.ecl = Some(s)}

                rule hcl(b: &mut PassportBuilder<'input>)
                    ="hcl:" s:$("#" ['0'..='9' | 'a'..='f']*<6,6>) { b.hcl = Some(s)}

                rule year(range: RangeInclusive<u32>) -> Year
                    = num:num() { ? 
                        if range.contains(&num) {
                            Ok(Year(num))
                        } else {
                            Err("year oor")
                        }
                    }

                rule num() -> u32
                    = s:$(['0'..='9']+) {s.parse().unwrap()}

                rule length() -> Length
                    = num:num() "cm" { Length::Cm(num) }
                    / num:num() "in" { Length::In(num) }
                   // / num:num() {Length::Unspec(num)}
            }
        }
        parser::root(input, &mut b).map_err(|e| Error::ParseError(input.into(),e.to_string()))?;
        Ok(b)
    }
}
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let results = contents
        .split("\r\n\r\n")
        .map(|input| PassportBuilder::parse(input).and_then(|b| b.build()));
    let num_valid = results.filter(Result::is_ok).count();
    dbg!(num_valid);
}
