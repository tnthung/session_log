use super::components::ComponentEx;
use std::fmt::Arguments;
use std::io::Write;
use log::Record;


pub trait Bundle: Send + Sync {
  fn create() -> Self where Self: Sized;
  fn write<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>);
  fn print<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>);
}


macro_rules! impl_bundle {
  () => {};

  ($j:ident $($i:ident)*) => {
    impl_bundle! { @ $j $($i)* }
    impl_bundle! { $($i)* }
  };

  (@ $($i:ident)+) => {
    impl<$($i: ComponentEx),+> Bundle for ($($i,)+) {
      fn create() -> Self {
        ($($i::default(),)+)
      }

      #[allow(non_snake_case)]
      fn write<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
        let ($($i,)+) = self;
        $($i.write(f, &message, &record);)+
      }

      #[allow(non_snake_case)]
      fn print<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
        let ($($i,)+) = self;
        $($i.print(f, &message, &record);)+
      }
    }
  };
}


impl_bundle! { A B C D E F G H I J K L M N O P }
