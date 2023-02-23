
use std::process::{Command, exit};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todavincivideo")]
#[command(author = "Sherlock Holmes")]
#[command(version = "0.0.1")]
#[command(about = "Tool to convert videos for Davinci Resolve cuz audio codec probems written in Rust.", long_about = "TTool to convert videos for Davinci Resolve cuz audio codec probems written in Rust.\nCopyright (C) 2023  Sherlock\n\nThis program is free software: you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation, either version 3 of the License, or\n(at your option) any later version.\n\nThis program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License\nalong with this program.  If not, see <https://www.gnu.org/licenses/>.")]
#[command(propagate_version = true)]
struct Cli {
    /// The input file to use
    input: String,
    /// The output file to use
    output: String,
}
fn main() {
    let cli = Cli::parse();
    Command::new("ffmpeg")
        .arg("-i")
        .arg(cli.input)
        .arg("-vcodec")
        .arg("h264")
        .arg("-q:v")
        .arg("2")
        .arg("-acodec")
        .arg("pcm_s16be")
        .arg("-q:a")
        .arg("0")
        .arg("-f")
        .arg("mov")
        .arg(cli.output)
        .spawn()
        .expect("ffmpeg command failed to start")
        .wait();

}
