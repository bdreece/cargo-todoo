use std::io::Result;
use std::path::PathBuf;

use clap::Parser;
use prettytable::*;

use cargo_todoo::{
    cli,
    source::{Comment, File, Tree},
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    // TODO: Walk source tree, collecting todos
    let path = PathBuf::from(".");
    let mut tree = Tree::new();
    tree.traverse(&path, &args.ignore_regex).await?;

    let mut todos: Vec<Comment> = vec![];
    let mut fixmes: Vec<Comment> = vec![];

    for entry in tree.entries {
        let mut file = File::new();
        file.parse_todos(&entry).await?;
        file.parse_fixmes(&entry).await?;
        todos.append(&mut file.todos);
        fixmes.append(&mut file.fixmes);
    }

    if !todos.is_empty() {
        let mut todo_table = Table::new();
        todo_table.set_titles(row![bd => "File", "Priority", "Message"]);

        todos.sort_by_key(|todo| 0 - todo.priority as isize);

        todos.iter().for_each(|todo| {
            match todo.priority {
                1 => todo_table.add_row(row![Fm => todo.file_name, todo.priority, todo.message]),
                2 => todo_table.add_row(row![Fb => todo.file_name, todo.priority, todo.message]),
                3 => todo_table.add_row(row![Fg => todo.file_name, todo.priority, todo.message]),
                4 => todo_table.add_row(row![Fy => todo.file_name, todo.priority, todo.message]),
                5 => todo_table.add_row(row![Fr => todo.file_name, todo.priority, todo.message]),
                _ => todo_table.add_row(row![Fw => todo.file_name, todo.priority, todo.message]),
            };
            ()
        });
        println!("TODO:");
        todo_table.printstd();
        println!("");
    }

    if !fixmes.is_empty() {
        let mut fixme_table = Table::new();
        fixme_table.set_titles(row![bd => "File", "Priority", "Message"]);

        fixmes.sort_by_key(|fixme| 0 - fixme.priority as isize);

        // TODOOOO: Rename fixme_table
        fixmes.iter().for_each(|fixme| {
            match fixme.priority {
                1 => {
                    fixme_table.add_row(row![Fm => fixme.file_name, fixme.priority, fixme.message])
                }
                2 => {
                    fixme_table.add_row(row![Fb => fixme.file_name, fixme.priority, fixme.message])
                }
                3 => {
                    fixme_table.add_row(row![Fg => fixme.file_name, fixme.priority, fixme.message])
                }
                4 => {
                    fixme_table.add_row(row![Fy => fixme.file_name, fixme.priority, fixme.message])
                }
                5 => {
                    fixme_table.add_row(row![Fr => fixme.file_name, fixme.priority, fixme.message])
                }
                _ => {
                    fixme_table.add_row(row![Fw => fixme.file_name, fixme.priority, fixme.message])
                }
            };
            ()
        });
        println!("FIXME:");
        fixme_table.printstd();
        println!("");
    }

    Ok(())
}
