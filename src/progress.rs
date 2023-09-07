use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::error::Error;

pub struct SpinnerHelper;

const TICK_TIME: u64 = 120;

impl SpinnerHelper {
    pub fn create(message: String) -> Result<ProgressBar, Error> {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_secs(TICK_TIME));
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
                .template("{spinner:.blue} {msg}")?,
        );
        spinner.set_message(message);

        Ok(spinner)
    }
}
