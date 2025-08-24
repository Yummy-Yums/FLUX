use std::{fs, io::{self, Write}};


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub completed: bool,
    pub created_at: String,
}

impl Task {
    pub fn new(content: String) -> Self {
        Self {
            id: chrono::Utc::now().timestamp().to_string(),
            content,
            completed: false,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    fn to_file_line(&self) -> String {
        format!(
            "task_{}: {} | status: {} | created: {}",
            self.id,
            self.content,
            if self.completed { "done" } else { "pending" },
            self.created_at
        )
    }
}

pub fn task_management_menu(username: String) {
    loop {
        println!("\n=== Task Manager - {} ===", username);
        println!("1. View tasks");
        println!("2. Add task");
        println!("3. Delete task");
        println!("4. Mark task as done");
        println!("5. Mark task as pending");
        println!("6. Search tasks");
        println!("7. Task statistics");
        println!("8. Export to JSON file");
        println!("9. Logout");
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "1" => view_task(&username),
            "2" => add_task(&username),
            "3" => delete_task(&username),
            "4" => mark_task_done(&username),
            "5" => mark_task_pending(&username),
            "6" => search_tasks(&username),
            "7" => show_task_statistics(&username),
            "8" => export_to_json(&username),
            "9" => {
                println!("Logged out successfully!");
                break;
            }
            _ => println!("❌ Invalid command! Please enter 1-8."),
        }
    }
}


fn view_task(username: &str) {
    println!("\n=== Your Tasks ===");

    let tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("📝 No tasks found. Add some tasks first!");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        let status_icon = if task.completed { "✅" } else { "⏳" };
        let status_text = if task.completed { "DONE" } else { "PENDING" };

        println!(
            "{}. {} [{}] {} (ID: {}) - Created: {}",
            index + 1,
            status_icon,
            status_text,
            task.content,
            task.id,
            task.created_at
        );
    }

    println!("\n📊 Total tasks: {}", tasks.len());
}

fn add_task(username: &str) {
    println!("\n=== Add New Task ===");

    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut task_content = String::new();
    io::stdin()
        .read_line(&mut task_content)
        .expect("Failed to read line");
    let task_content = task_content.trim();

    if task_content.is_empty() {
        println!("❌ Task description cannot be empty!");
        return;
    }

    let new_task = Task::new(task_content.to_string());
    let mut tasks = get_all_tasks(username);
    tasks.push(new_task);

    match save_tasks(username, &tasks) {
        Ok(()) => println!("✅ Task added successfully!"),
        Err(e) => eprintln!("❌ Failed to save task: {}", e),
    }
}

fn delete_task(username: &str) {
    println!("\n=== Delete Task ===");

    let mut tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("📝 No tasks to delete!");
        return;
    }

    // Show tasks with numbers
    println!("Current tasks:");
    for (index, task) in tasks.iter().enumerate() {
        let status_icon = if task.completed { "✅" } else { "⏳" };
        println!(
            "{}. {} {} (ID: {})",
            index + 1,
            status_icon,
            task.content,
            task.id
        );
    }

    print!("\nEnter task number to delete (1-{}): ", tasks.len());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(task_num) if task_num > 0 && task_num <= tasks.len() => {
            let removed_task = tasks.remove(task_num - 1);

            match save_tasks(username, &tasks) {
                Ok(()) => println!("✅ Task '{}' deleted successfully!", removed_task.content),
                Err(e) => eprintln!("❌ Failed to delete task: {}", e),
            }
        }
        _ => println!("❌ Invalid task number!"),
    }
}

fn mark_task_done(username: &str) {
    mark_task_status(username, true, "done");
}

fn mark_task_pending(username: &str) {
    mark_task_status(username, false, "pending");
}

fn mark_task_status(username: &str, completed: bool, status_name: &str) {
    println!("\n=== Mark Task as {} ===", status_name.to_uppercase());

    let mut tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("📝 No tasks found!");
        return;
    }

    // Show only tasks with opposite status
    let relevant_tasks: Vec<(usize, &Task)> = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| task.completed != completed)
        .collect();

    if relevant_tasks.is_empty() {
        println!("📝 No tasks available to mark as {}!", status_name);
        return;
    }

    println!("Tasks available to mark as {}:", status_name);
    for (display_index, (_, task)) in relevant_tasks.iter().enumerate() {
        let status_icon = if task.completed { "✅" } else { "⏳" };
        println!(
            "{}. {} {} (ID: {})",
            display_index + 1,
            status_icon,
            task.content,
            task.id
        );
    }

    print!("\nEnter task number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(choice) if choice > 0 && choice <= relevant_tasks.len() => {
            let (original_index, _) = relevant_tasks[choice - 1];
            tasks[original_index].completed = completed;

            match save_tasks(username, &tasks) {
                Ok(()) => println!("✅ Task marked as {} successfully!", status_name),
                Err(e) => eprintln!("❌ Failed to update task: {}", e),
            }
        }
        _ => println!("❌ Invalid choice!"),
    }
}
fn search_tasks(username: &str) {
    println!("\n=== Search Tasks ===");

    print!("Enter search term: ");
    io::stdout().flush().unwrap();
    let mut search_term = String::new();
    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");
    let search_term = search_term.trim().to_lowercase();

    if search_term.is_empty() {
        println!("❌ Search term cannot be empty!");
        return;
    }

    let tasks = get_all_tasks(username);
    let matching_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.content.to_lowercase().contains(&search_term))
        .collect();

    if matching_tasks.is_empty() {
        println!("🔍 No tasks found containing '{}'", search_term);
    } else {
        println!(
            "🔍 Found {} task(s) containing '{}':",
            matching_tasks.len(),
            search_term
        );
        for (index, task) in matching_tasks.iter().enumerate() {
            let status_icon = if task.completed { "✅" } else { "⏳" };
            let status_text = if task.completed { "DONE" } else { "PENDING" };
            println!(
                "{}. {} [{}] {} (ID: {})",
                index + 1,
                status_icon,
                status_text,
                task.content,
                task.id
            );
        }
    }
}

fn show_task_statistics(username: &str) {
    println!("\n=== Task Statistics ===");

    let tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("📊 No tasks found!");
        return;
    }

    let total = tasks.len();
    let completed = tasks.iter().filter(|task| task.completed).count();
    let pending = total - completed;
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("📊 Total tasks: {}", total);
    println!("✅ Completed: {}", completed);
    println!("⏳ Pending: {}", pending);
    println!("📈 Completion rate: {:.1}%", completion_rate);

    // Show recent tasks
    if !tasks.is_empty() {
        println!("\n📅 Recent tasks:");
        let mut recent_tasks = tasks.clone();
        recent_tasks.sort_by(|a, b| b.id.cmp(&a.id)); // Sort by ID (timestamp) descending

        for task in recent_tasks.iter().take(3) {
            let status_icon = if task.completed { "✅" } else { "⏳" };
            println!("  {} {} - {}", status_icon, task.content, task.created_at);
        }
    }
}
pub fn export_to_json(username: &str) {
    let tasks = get_all_tasks(username);
    let json_str = serde_json::to_string(&tasks).unwrap();
    match fs::write("tasks.json", json_str) {
        Ok(_) => println!("tasks.json created successfully!"),
        Err(_) => eprintln!("tasks.json could not be written")
    }
}

pub fn get_all_tasks(username: &str) -> Vec<Task> {
    let filename = format!("{}.txt", username.trim());

    match fs::read_to_string(&filename) {
        Ok(content) => content.lines().filter_map(parse_task_line).collect(),
        Err(_) => Vec::new(),
    }
}

pub fn save_tasks(username: &str, tasks: &[Task]) -> Result<(), std::io::Error> {
    let filename = format!("{}.txt", username.trim());

    // Read existing file to preserve user info
    let mut lines = Vec::new();

    if let Ok(content) = fs::read_to_string(&filename) {
        for line in content.lines() {
            if !line.starts_with("task_") {
                lines.push(line.to_string());
            }
        }
    }

    // Add all tasks
    for task in tasks {
        lines.push(task.to_file_line());
    }

    fs::write(&filename, lines.join("\n"))
}

fn parse_task_line(line: &str) -> Option<Task> {
    // Try advanced format first, then fall back to simple format
    parse_task_line_advanced(line).or_else(|| parse_task_line_simple(line))
}

// BACKWARD COMPATIBILITY - parse old format tasks
pub fn parse_task_line_simple(line: &str) -> Option<Task> {
    let after_prefix = line.strip_prefix("task_")?;
    let (id_part, content_part) = after_prefix.split_once(": ")?;

    if id_part.trim().is_empty() || content_part.trim().is_empty() {
        return None;
    }

    Some(Task {
        id: id_part.trim().to_string(),
        content: content_part.trim().to_string(),
        completed: false,
        created_at: "Unknown".to_string(),
    })
}
// IMPROVED TASK PARSING - handles new format with status
pub fn parse_task_line_advanced(line: &str) -> Option<Task> {
    let after_prefix = line.strip_prefix("task_")?;

    // Split by " | " to get parts
    let parts: Vec<&str> = after_prefix.split(" | ").collect();
    if parts.len() < 3 {
        return None; // Not enough parts
    }

    // Parse ID and content from first part
    let (id_part, content_part) = parts[0].split_once(": ")?;

    // Parse status
    let status_part = parts[1].strip_prefix("status: ")?;
    let completed = status_part == "done";

    // Parse created date
    let created_part = parts[2].strip_prefix("created: ").unwrap_or("Unknown");

    if id_part.trim().is_empty() || content_part.trim().is_empty() {
        return None;
    }

    Some(Task {
        id: id_part.trim().to_string(),
        content: content_part.trim().to_string(),
        completed,
        created_at: created_part.to_string(),
    })
}