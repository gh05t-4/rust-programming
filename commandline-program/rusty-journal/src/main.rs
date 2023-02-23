/* 
The program interface will handle these three actions:

   - Add new tasks to a to-do list.
   - Remove completed tasks from that list.
   - Print all the current tasks in the list.
   - The program will persist our to-do items in some kind of storage. A text file should be good enough to store this kind of data, so we can stick to a file format like JSON to encode our information.
*/

mod cli;
mod tasks;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

use std::path::PathBuf;
use anyhow::anyhow;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file.or_else(find_default_journal_file).ok_or(anyhow!("Failed to find journal file"))?;

    // Perform the action.
    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
