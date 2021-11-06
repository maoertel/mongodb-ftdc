use indicatif::{ProgressBar, ProgressStyle};

pub struct SpinnerHelper;

impl SpinnerHelper {
  pub fn create(message: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(120);
    spinner.set_style(
      ProgressStyle::default_spinner()
        // For more spinners check out the cli-spinners project:
        // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
        .tick_strings(&[
          "□ □ □ □ □",
          "■ □ □ □ □",
          "□ ■ □ □ □",
          "□ □ ■ □ □",
          "□ □ □ ■ □",
          "□ □ □ □ ■",
          "■ ■ ■ ■ ■",
        ])
        .template("{spinner:.blue} {msg}"),
    );
    spinner.set_message(message);
    spinner
  }
}
