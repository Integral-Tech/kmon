use tui::style::{Color, Modifier, Style as TuiStyle};
use tui::widgets::Text;

/* Style properties */
pub struct Style {
	pub title_style: TuiStyle,
	pub selected_style: TuiStyle,
	pub unselected_style: TuiStyle,
	pub highlight_style: TuiStyle,
}

impl Default for Style {
	/**
	 * Create a default style instance.
	 *
	 * @return Style
	 */
	fn default() -> Self {
		Self {
			title_style: TuiStyle::default().modifier(Modifier::BOLD),
			selected_style: TuiStyle::default().fg(Color::White),
			unselected_style: TuiStyle::default().fg(Color::DarkGray),
			highlight_style: TuiStyle::default()
				.fg(Color::Red)
				.modifier(Modifier::BOLD),
		}
	}
}

/* Styled text that has raw and style parts */
pub struct StyledText<'a> {
	raw_text: String,
	styled_text: Vec<Text<'a>>,
}

impl Default for StyledText<'_> {
	/**
	 * Create a default styled text instance.
	 *
	 * @return StyledText
	 */
	fn default() -> Self {
		Self {
			raw_text: String::new(),
			styled_text: Vec::new(),
		}
	}
}

impl<'a> StyledText<'a> {
	/**
	 * Get a vector of Text widget from styled text.
	 *
	 * @return vector
	 */
	pub fn get(&'a self) -> Vec<Text<'a>> {
		if self.styled_text.is_empty() {
			vec![Text::raw(&self.raw_text)]
		} else {
			self.styled_text.to_vec()
		}
	}
	/**
	 * Set a raw text.
	 *
	 * @param text
	 */
	pub fn set_raw_text(&mut self, text: String) {
		self.raw_text = text;
		self.styled_text = Vec::new();
	}
	/**
	 * Set a styled text.
	 *
	 * @param text
	 * @param newline_count
	 */
	pub fn set_styled_text(
		&mut self,
		text: Vec<Text<'static>>,
		newline_count: usize,
	) {
		self.styled_text = text;
		/* Append empty strings as much as newlines. */
		for _i in 0..newline_count * 2 {
			self.styled_text.push(Text::raw(""));
		}
		self.raw_text = String::new();
	}
	/**
	 * Return the line count of styled text.
	 *
	 * @return usize
	 */
	pub fn lines(&self) -> usize {
		if !self.raw_text.is_empty() {
			self.raw_text.lines().count()
		} else {
			self.styled_text.len()
		}
	}
}