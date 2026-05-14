use super::Component;


macro_rules! format_component {
  ($name:ident, $doc:literal, $code:literal) => {
    #[doc = $doc]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct $name;

    impl Component for $name {
      fn write_plain<'a>(&self, _: &mut dyn std::io::Write, _: &log::Record<'a>) {}
      fn write_color<'a>(&self, f: &mut dyn std::io::Write, _: &log::Record<'a>) {
        write!(f, concat!("\x1b[", $code, "m")).unwrap();
      }
    }
  };
}


format_component!(Reset, "Reset all colors and styles.", "0");

format_component!(Bold, "Enable bold text.", "1");
format_component!(Dim, "Enable dim text.", "2");
format_component!(Italic, "Enable italic text.", "3");
format_component!(Underline, "Enable underlined text.", "4");
format_component!(Blink, "Enable blinking text.", "5");
format_component!(Reverse, "Swap foreground and background colors.", "7");
format_component!(Hidden, "Hide text.", "8");
format_component!(Strikethrough, "Enable struck-through text.", "9");

format_component!(NoBold, "Disable bold and dim text.", "22");
format_component!(NoItalic, "Disable italic text.", "23");
format_component!(NoUnderline, "Disable underlined text.", "24");
format_component!(NoBlink, "Disable blinking text.", "25");
format_component!(NoReverse, "Disable reversed foreground and background colors.", "27");
format_component!(NoHidden, "Disable hidden text.", "28");
format_component!(NoStrikethrough, "Disable struck-through text.", "29");