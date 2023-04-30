use juniper::GraphQLObject;




#[derive(GraphQLObject)]
#[graphql(description = "asdf")]
pub struct Task {
    id: i32,
    name: String,
    description: String,
    parent_id: i32,
    taskType: String, 
    deadline: NaiveDateTime,
    duration: i32, 
    is_done: bool, 
    create_time: NativeDateTime,, b.update_time, b.root_id

}