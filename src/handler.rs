use crate::{
    model::{QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
    WebResult, MongoDB,
};
use chrono::prelude::*;
use mongodb::bson::{doc, oid::ObjectId};
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};
use futures::stream::TryStreamExt; // For async MongoDB streaming

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn todos_list_handler(opts: QueryOptions, db: MongoDB) -> WebResult<impl Reply> {
    let collection = db.collection::<Todo>("todos");
    let limit = opts.limit.unwrap_or(10) as i64;
    let offset = (opts.page.unwrap_or(1) as usize - 1) * limit as usize;

    let mut cursor = collection
        .find(None, None)
        .await
        .expect("Failed to fetch todos");
    
    let mut todos: Vec<Todo> = Vec::new();
    while let Some(todo) = cursor.try_next().await.unwrap() {
        todos.push(todo);
    }
    let todos: Vec<Todo> = todos.into_iter().skip(offset as usize).take(limit as usize).collect();
    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    Ok(json(&json_response))
}

pub async fn create_todo_handler(mut body: Todo, db: MongoDB) -> WebResult<impl Reply> {
    let collection = db.collection::<Todo>("todos");

    let existing_todo = collection.find_one(doc! {"title": &body.title}, None).await;
    if let Ok(Some(_)) = existing_todo {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title: '{}' already exists", body.title),
        };
        return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
    }

    let datetime = Utc::now();
    body.id = Some(ObjectId::new().to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    collection.insert_one(&body, None).await.expect("Failed to insert todo");

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo: body },
    };

    Ok(with_status(json(&json_response), StatusCode::CREATED))
}

pub async fn get_todo_handler(id: String, db: MongoDB) -> WebResult<impl Reply> {
    let collection = db.collection::<Todo>("todos");
    let obj_id = ObjectId::parse_str(&id).expect("Invalid ObjectId format");

    let todo = collection.find_one(doc! {"_id": obj_id}, None).await;
    if let Ok(Some(todo)) = todo {
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo },
        };
        return Ok(with_status(json(&json_response), StatusCode::OK));
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
}

pub async fn edit_todo_handler(
    id: String,
    body: UpdateTodoSchema,
    db: MongoDB,
) -> WebResult<impl Reply> {
    let collection = db.collection::<Todo>("todos");
    let obj_id = ObjectId::parse_str(&id).expect("Invalid ObjectId format");
    let datetime = Utc::now();

    let update_doc = doc! {
        "$set": {
            "title": body.title.unwrap_or_else(|| "".to_string()),
            "content": body.content.unwrap_or_else(|| "".to_string()),
            "completed": body.completed.unwrap_or(false),
            "updatedAt": datetime.to_string(), // Convert to string
        }
    };

    let update_result = collection.update_one(doc! {"_id": obj_id}, update_doc, None).await;
    if update_result.is_ok() && update_result.unwrap().matched_count == 1 {
        let updated_todo = collection.find_one(doc! {"_id": obj_id}, None).await.unwrap();
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: updated_todo.unwrap() },
        };
        return Ok(with_status(json(&json_response), StatusCode::OK));
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
}

pub async fn delete_todo_handler(id: String, db: MongoDB) -> WebResult<impl Reply> {
    let collection = db.collection::<Todo>("todos");
    let obj_id = ObjectId::parse_str(&id).expect("Invalid ObjectId format");

    let delete_result = collection.delete_one(doc! {"_id": obj_id}, None).await;
    if let Ok(result) = delete_result {
        if result.deleted_count == 1 {
            return Ok(with_status(json(&""), StatusCode::NO_CONTENT));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
}
