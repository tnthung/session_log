use super::components::{Component, ComponentEx};
use std::io::Write;
use log::Record;


pub(crate) trait Bundle: Send + Sync {
  fn write<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>);
  #[cfg_attr(not(feature = "style"), allow(dead_code))]
  fn print<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>);
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
      fn write<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>) {
        $($i::write(f, &record);)+
      }

      #[allow(non_snake_case)]
      fn print<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>) {
        $($i::print(f, &record);)+
      }
    }
  };
}


impl_bundle! { A B C D E F G H I J K L }


impl<B: Bundle> Component for B {
  fn write_plain<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>) { Self::write(f, record); }
  #[cfg(feature = "style")]
  fn write_style<'a, W: Write + ?Sized>(f: &mut W, record: &Record<'a>) { Self::print(f, record); }
}
