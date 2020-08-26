
/// vocabulary Address generated from address.cql
mod address {
  use std::convert::TryFrom;
  use std::convert::From;

  pub struct Constellation {
    pub all_city: Vec<City>,
    pub all_company_name: Vec<CompanyName>,
    pub all_company: Vec<Company>,
  }

  /// Error value for conversions.
  // - I'd probably write a macro for this whole thing, but codegen is alright.
  pub struct StringTooLong {
    pub original: String,
    pub maximum: usize,
    pub actual: usize,
  }

  /// City is written as String(64);
  pub struct City(String);

  impl TryFrom<String> for City  {
    type Error = StringTooLong;
    fn try_from(value: String) -> Result<Self, StringTooLong> {
      let len = value.chars().count();
      let max = 64;
        if len > max {
            Err(StringTooLong {original: value, maximum: max, actual: len })
        } else {
            Ok(City(value))
        }
    }
  }

  /// CompanyName is written as String;
  pub struct CompanyName(String);

  impl From<String> for CompanyName {
    fn from(value: String) -> Self {
      CompanyName(value)
    }
  }

  pub struct Company {
    pub company_name: CompanyName,
  }

  use rbtree::RBTree;

  #[test]
  fn address() {
    let mut book_reviews = RBTree::new();
    book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.");

    let mstr = String::from("Melbourne");
    match City::try_from(mstr) {
      _ => "",
      StringTooLong => panic!("it is not too long"),
    };
    let lstr = String::from("MelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourneMelbourne");
    match City::try_from(lstr) {
      StringTooLong => "",
      _ => panic!("string should be too long")
    };
  }
  #[test]
  fn constellation() {
    let all_city = Vec::new();
    let all_company_name = Vec::new();
    let all_company = Vec::new();
    let mut c = Constellation{all_city, all_company_name, all_company};

    let mel = City::try_from(String::from("Melbourne")).ok().expect("Melbourne is not too long");
    c.all_city.push(mel)
  }
}
