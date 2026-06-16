use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result as SqlResult};
use chrono::{DateTime, Utc};
use colored::*;
use humantime;

#[derive(Parser)]
#[command(author, version, about = "Rustle — A fast, reliable CLI task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task description
        description: String,
        /// Due date (e.g. "2 weeks", "tomorrow", "2026-12-25T12:00:00Z")
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List tasks (shows all by default)
    List {
        /// Show only pending tasks
        #[arg(long)]
        pending: bool,
        /// Show only completed tasks
        #[arg(long, conflicts_with_all = ["pending", "overdue"])]
        completed: bool,
        /// Show only overdue tasks
        #[arg(long, conflicts_with_all = ["pending", "completed"])]
        overdue: bool,
    },
    /// Mark a task as complete
    Complete {
        /// ID of the task to complete
        id: u32,
    },
    /// Delete a task
    Delete {
        /// ID of the task to delete
        id: u32,
    },
}

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>,
}

fn main() -> SqlResult<()> {
    let cli = Cli::parse();
    let conn = Connection::open("tasks.db")?;
    init_db(&conn)?;

    match cli.command {
        Commands::Add { description, due } => add_task(&conn, &description, due)?,
        Commands::List { pending, completed, overdue } => {
            list_tasks(&conn, pending, completed, overdue)?
        }
        Commands::Complete { id } => complete_task(&conn, id)?,
        Commands::Delete { id } => delete_task(&conn, id)?,
    }

    Ok(())
}

fn init_db(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            due_date TEXT
        )",
        [],
    )?;
    Ok(())
}

fn add_task(conn: &Connection, description: &str, due: Option<String>) -> SqlResult<()> {
    let now = Utc::now();
    let due_date = due.and_then(|input| {
        if let Ok(dur) = humantime::parse_duration(&input) {
            Some(now + dur)
        } else if let Ok(dt) = DateTime::parse_from_rfc3339(&input) {
            Some(dt.with_timezone(&Utc))
        } else {
            None
        }
    });

    conn.execute(
        "INSERT INTO tasks (description, completed, created_at, due_date) VALUES (?1, 0, ?2, ?3)",
        (description, now.to_rfc3339(), due_date.map(|d| d.to_rfc3339())),
    )?;

    println!("✅ Task added: \"{}\"", description);
    if let Some(due) = due_date {
        println!("   Due: {}", due.format("%b %d, %Y at %H:%M"));
    }
    Ok(())
}

fn list_tasks(
    conn: &Connection,
    pending: bool,
    completed: bool,
    overdue: bool,
) -> SqlResult<()> {
    let query = if overdue {
        "SELECT id, description, completed, created_at, due_date FROM tasks 
         WHERE completed = 0 AND due_date IS NOT NULL AND due_date < ?1 
         ORDER BY due_date ASC"
    } else if completed {
        "SELECT id, description, completed, created_at, due_date FROM tasks 
         WHERE completed = 1 ORDER BY created_at DESC"
    } else if pending {
        "SELECT id, description, completed, created_at, due_date FROM tasks 
         WHERE completed = 0 ORDER BY due_date ASC NULLS LAST, created_at DESC"
    } else {
        "SELECT id, description, completed, created_at, due_date FROM tasks 
         ORDER BY completed ASC, due_date ASC NULLS LAST, created_at DESC"
    };

    let mut stmt = conn.prepare(query)?;

    let now = Utc::now().to_rfc3339();
    let task_iter = if overdue {
        stmt.query_map([now], map_task)?
    } else {
        stmt.query_map([], map_task)?
    };

    println!("📋 Your tasks:\n");
    for task_result in task_iter {
        let t = task_result?;
        let status = if t.completed { "✓".green() } else { "○".yellow() };
        let text = if t.completed { t.description.dimmed() } else { t.description.normal() };

        let due_str = if let Some(d) = t.due_date {
            let due_fmt = format!(" (due {})", d.format("%b %d"));
            if !t.completed && d < Utc::now() {
                due_fmt.red().to_string()
            } else {
                due_fmt
            }
        } else {
            String::new()
        };

        let created = format!(" [created {}]", t.created_at.format("%b %d")).dimmed();

        println!("  {} {} {}{}{}", t.id, status, text, due_str, created);
    }
    Ok(())
}

fn map_task(row: &rusqlite::Row) -> SqlResult<Task> {
    let due_str: Option<String> = row.get(4)?;
    let due_date = due_str.and_then(|s| {
        DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    Ok(Task {
        id: row.get(0)?,
        description: row.get(1)?,
        completed: row.get(2)?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
            .unwrap()
            .with_timezone(&Utc),
        due_date,
    })
}

fn complete_task(conn: &Connection, id: u32) -> SqlResult<()> {
    let rows = conn.execute("UPDATE tasks SET completed = 1 WHERE id = ?1", (id,))?;
    if rows > 0 {
        println!("✅ Task {} marked as complete", id);
    } else {
        println!("⚠️  No task found with ID {}", id);
    }
    Ok(())
}

fn delete_task(conn: &Connection, id: u32) -> SqlResult<()> {
    let rows = conn.execute("DELETE FROM tasks WHERE id = ?1", (id,))?;
    if rows > 0 {
        println!("🗑️  Task {} deleted", id);
    } else {
        println!("⚠️  No task found with ID {}", id);
    }
    Ok(())
}