// use anyhow::Context;
use crate::db::repositories::todo_repository::Todo;

type TodoDataMap = HashMap<i32, Todo>;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDataMap>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }

    /// return writable HashMap with thread-safe
    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDataMap> {
        self.store.write().unwrap()
    }

    /// return readable HashMap with thread-safe
    fn read_store_ref(&self) -> RwLockReadGuard<TodoDataMap> {
        self.store.read().unwrap()
    }
}

// use axum::http::{ Response };
// use hyper::http;
// use futures::future::Ready;
use crate::db::repositories::todo_repository::{CreateTodo, RepositoryError, TodoRepository};
use anyhow::Context;
impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Box<Todo>> {
        let store = self.read_store_ref();
        // store.get(&id).map(|todo| todo.clone())
        let todo = store.get(&id)?;
        let todo = Box::new(todo.clone());
        Some(todo)
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    fn update(&self, id: i32, payload: super::todo_repository::UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::repositories::todo_repository::{Todo, UpdateTodo};

    /// test scenario:
    /// 1.create todo
    /// 2.find todo by id
    /// 3.get all todo
    /// 4.update todo that is created todo
    /// 5.delete todo
    #[test]
    fn todo_crud_scenario() {
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo { text });
        assert_eq!(expected, todo);

        let todo = *(repository.find(todo.id).unwrap());
        assert_eq!(expected, todo);

        let todo = repository.all();
        assert_eq!(vec![expected], todo);

        let text = "update todo text".to_string();
        let todo = repository
            .update(
                1,
                UpdateTodo {
                    text: Some(text.clone()),
                    completed: Some(true),
                },
            )
            .expect("failed update todo");
        assert_eq!(
            Todo {
                id,
                text,
                completed: true
            },
            todo
        );

        let res = repository.delete(id);
        assert!(res.is_ok());
    }
}
