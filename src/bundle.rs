use super::components::{Component, ComponentEx};
use std::io::Write;
use log::Record;


pub(crate) trait Bundle: Default + Send + Sync {
  fn write<'a>(&self, f: &mut dyn Write, record: &Record<'a>);
  fn print<'a>(&self, f: &mut dyn Write, record: &Record<'a>);
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
      fn write<'a>(&self, f: &mut dyn Write, record: &Record<'a>) {
        let ($($i,)+) = self;
        $($i.write(f, &record);)+
      }

      #[allow(non_snake_case)]
      fn print<'a>(&self, f: &mut dyn Write, record: &Record<'a>) {
        let ($($i,)+) = self;
        $($i.print(f, &record);)+
      }
    }
  };
}


impl_bundle! { A B C D E F G H I J K L }


impl<B: Bundle> Component for B {
  fn write_plain<'a>(&self, f: &mut dyn Write, record: &Record<'a>) {
    self.write(f, record);
  }

  fn write_color<'a>(&self, f: &mut dyn Write, record: &Record<'a>) {
    self.print(f, record);
  }
}
