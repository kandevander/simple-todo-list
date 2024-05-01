use gtk::prelude::*;
use gtk::{Entry, ListBox, ListBoxRow, Button, Window, WindowType, Inhibit};
use glib::clone;
use std::rc::Rc;
use std::cell::RefCell;

struct TodoList {
    tasks: Rc<RefCell<Vec<String>>>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Rc::new(RefCell::new(Vec::new())) }
    }

    fn add_task(&self, task: String) {
        self.tasks.borrow_mut().push(task);
    }

    fn remove_task(&self, index: usize) {
        if index < self.tasks.borrow().len() {
            self.tasks.borrow_mut().remove(index);
        }
    }
}

fn create_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Todo List");
    window.set_default_size(350, 70);
    window
}

fn create_vbox() -> gtk::Box {
    gtk::Box::new(gtk::Orientation::Vertical, 5)
}

fn create_entry() -> Rc<RefCell<Entry>> {
    Rc::new(RefCell::new(Entry::new()))
}

fn create_button(label: &str) -> Button {
    Button::with_label(label)
}

fn create_list_box() -> Rc<RefCell<ListBox>> {
    Rc::new(RefCell::new(ListBox::new()))
}

fn add_task(todo_list: &TodoList, task: &str, list_box: &Rc<RefCell<ListBox>>, new_task_entry: &Rc<RefCell<Entry>>) {
    if !task.trim().is_empty() {
        todo_list.add_task(task.to_string());

        let row = ListBoxRow::new();
        let label = gtk::Label::new(Some(task));
        row.add(&label);
        list_box.borrow_mut().add(&row);

        new_task_entry.borrow_mut().set_text("");
        list_box.borrow().show_all();
    }
}

fn remove_task(todo_list: &TodoList, list_box: &Rc<RefCell<ListBox>>) {
    let selected_row = list_box.borrow().selected_row();
    if let Some(row) = selected_row {
        let index = row.index() as usize;
        todo_list.remove_task(index);
        list_box.borrow_mut().remove(&row);
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let todo_list = Rc::new(RefCell::new(TodoList::new()));

    let window = create_window();

    let vbox = create_vbox();
    window.add(&vbox);

    let new_task_entry = create_entry();
    vbox.pack_start(&*new_task_entry.borrow(), false, false, 0);

    let add_button = create_button("Add Task");
    vbox.pack_start(&add_button, false, false, 0);

    let list_box = create_list_box();
    vbox.pack_start(&list_box.borrow().clone(), false, false, 0);

    let remove_button = create_button("Remove Task");
    vbox.pack_start(&remove_button, false, false, 0);

    add_button.connect_clicked(clone!(@weak todo_list, @weak list_box, @weak new_task_entry => move |_| {
        let task = new_task_entry.borrow().text().as_str().to_string();
        add_task(&todo_list.borrow(), &task, &list_box, &new_task_entry);
    }));

    remove_button.connect_clicked(clone!(@weak list_box, @weak todo_list => move |_| {
        remove_task(&todo_list.borrow(), &list_box);
    }));

    new_task_entry.borrow().connect_activate(clone!(@weak todo_list, @weak list_box, @weak new_task_entry => move |_| {
        let task = new_task_entry.borrow().text().as_str().to_string();
        add_task(&todo_list.borrow(), &task, &list_box, &new_task_entry);
    }));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window.show_all();
    gtk::main();
}