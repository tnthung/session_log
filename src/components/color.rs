use super::Component;


macro_rules! color_component {
  ($name:ident, $doc:literal, $code:literal) => {
    #[doc = $doc]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct $name;

    impl Component for $name {
      fn write_plain<'a>(_: &mut dyn std::io::Write, _: &log::Record<'a>) {}
      fn write_color<'a>(f: &mut dyn std::io::Write, _: &log::Record<'a>) {
        write!(f, concat!("\x1b[", $code, "m")).unwrap();
      }
    }
  };
}


color_component!(Reset, "Reset all colors and styles.", "0");
color_component!(ResetForeground, "Reset the foreground color.", "39");
color_component!(ResetBackground, "Reset the background color.", "49");

color_component!(Black, "Change the foreground color to black.", "30");
color_component!(Red, "Change the foreground color to red.", "31");
color_component!(Green, "Change the foreground color to green.", "32");
color_component!(Yellow, "Change the foreground color to yellow.", "33");
color_component!(Blue, "Change the foreground color to blue.", "34");
color_component!(Magenta, "Change the foreground color to magenta.", "35");
color_component!(Cyan, "Change the foreground color to cyan.", "36");
color_component!(White, "Change the foreground color to white.", "37");

color_component!(BrightBlack, "Change the foreground color to bright black.", "90");
color_component!(BrightRed, "Change the foreground color to bright red.", "91");
color_component!(BrightGreen, "Change the foreground color to bright green.", "92");
color_component!(BrightYellow, "Change the foreground color to bright yellow.", "93");
color_component!(BrightBlue, "Change the foreground color to bright blue.", "94");
color_component!(BrightMagenta, "Change the foreground color to bright magenta.", "95");
color_component!(BrightCyan, "Change the foreground color to bright cyan.", "96");
color_component!(BrightWhite, "Change the foreground color to bright white.", "97");

color_component!(BgBlack, "Change the background color to black.", "40");
color_component!(BgRed, "Change the background color to red.", "41");
color_component!(BgGreen, "Change the background color to green.", "42");
color_component!(BgYellow, "Change the background color to yellow.", "43");
color_component!(BgBlue, "Change the background color to blue.", "44");
color_component!(BgMagenta, "Change the background color to magenta.", "45");
color_component!(BgCyan, "Change the background color to cyan.", "46");
color_component!(BgWhite, "Change the background color to white.", "47");

color_component!(BgBrightBlack, "Change the background color to bright black.", "100");
color_component!(BgBrightRed, "Change the background color to bright red.", "101");
color_component!(BgBrightGreen, "Change the background color to bright green.", "102");
color_component!(BgBrightYellow, "Change the background color to bright yellow.", "103");
color_component!(BgBrightBlue, "Change the background color to bright blue.", "104");
color_component!(BgBrightMagenta, "Change the background color to bright magenta.", "105");
color_component!(BgBrightCyan, "Change the background color to bright cyan.", "106");
color_component!(BgBrightWhite, "Change the background color to bright white.", "107");
