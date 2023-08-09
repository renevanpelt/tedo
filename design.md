## Design


Commands:

### 1. Create a Project

Creates a new project and optionally switches the context to the new project.

```bash
tedo create project <project_name> [--switch]
```

- `<project_name>`: The name of the project.
- `--switch`: An optional flag to switch to the newly created project.

### 2. Change Context to an Existing Project

Switches the current context to an existing project.

```bash
tedo switch <project_name>
```

- `<project_name>`: The name of the existing project.

### 3. Create a Task in Current Context

Creates a new task in the current context (project).

```bash
tedo create task <task_description> [--due <due_date>]
```

- `<task_description>`: The description of the task.
- `--due <due_date>`: An optional due date for the task.

### 4. Create a List in Current Context

Creates a new list in the current context (project).

```bash
tedo create list <list_name>
```

- `<list_name>`: The name of the list.

### 5. Create a Note in Current Context

Creates a new note in the current context (project).

```bash
tedo create note <note_title> [--content <note_content>]
```

- `<note_title>`: The title of the note.
- `--content <note_content>`: An optional content for the note.

### 6. List All Projects

Lists all the existing projects.

```bash
tedo list projects
```

### 7. List All Tasks

Lists all the tasks.

```bash
tedo list tasks [--project <project_name>]
```

- `--project <project_name>`: An optional filter to list tasks within a specific project.

### 8. List All Tasks in Project

Lists all the tasks within a specific project.

```bash
tedo list tasks --project <project_name>
```

- `<project_name>`: The name of the project.

### 9. List All Tasks in Current Context

Lists all the tasks within the current context (project).

```bash
tedo list tasks --current
```

This syntax aims to be intuitive and consistent. It provides a hierarchical structure where actions like creating, listing, and switching are grouped with their corresponding objects like projects, tasks, lists, and notes. Optional parameters and flags allow further customization and filtering.


```toml

[[lists]]
    name = "Links"
    [[lists.fields]]
    name        = "title"
    label       = "Title"

    [[lists.fields]]
    name        = "url"
    label       = "Url"
    

[[extension_actions]]
    name            = 'Add Page to "Links"'
    type            = 'button'
    action          = 'add_to_list'
    [[extensions_actions.mappings]]
    selector    = 'title'
    field       = 'title'
    [[extensions_actions.mappings]]
    selector    = 'title'
    field       = 'title'

    

```