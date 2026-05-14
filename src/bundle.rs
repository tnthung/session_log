use super::components::{Component, ComponentEx};
use std::io::Write;
use log::Record;


pub(crate) trait Bundle: Send + Sync {
  fn write<'a>(f: &mut dyn Write, record: &Record<'a>);
  fn print<'a>(f: &mut dyn Write, record: &Record<'a>);
}


macro_rules! impl_bundle {
  () => {};

  ($j:ident $($i:ident)*) => {
    impl_bundle! { @ $j $($i)* }
    impl_bundle! { $($i)* }
  };

  (@ $($i:ident)+) => {
    impl<$($i: ComponentEx),+> Bundle for ($($i,)+) {
      #[allow(non_snake_case)]
      fn write<'a>(f: &mut dyn Write, record: &Record<'a>) {
        $($i::write(f, &record);)+
      }

      #[allow(non_snake_case)]
      fn print<'a>(f: &mut dyn Write, record: &Record<'a>) {
        $($i::print(f, &record);)+
      }
    }
  };
}


impl_bundle! { A B C D E F G H I J K L }


impl<B: Bundle> Component for B {
  fn write_plain<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::write(f, record);
  }

  fn write_color<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::print(f, record);
  }
}
