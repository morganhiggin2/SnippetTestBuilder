use serde_json;

/// validate the json schema according to 
/// # Schema Schema
/// types are denoted with a name a value, as an example:
/// {
///     "input_one": {type_object} 
/// }
/// 
/// values can themselfs be a nesting of other sub schemas, and follow any format:
/// {
///     "input_one": {
///         "sub_input_one": {type_object} 
///     } 
///     "input_two": {type_object}
/// }
/// 
/// the string json values are themselfs the name of the primitive type, which includes
/// * str
/// * bytes
/// * bool
/// * int 
/// * float
/// 
/// additional types supported:
/// * tuple[T: other primitive types, U: other primitive types]
/// * set[T: other primitive types]
/// * list[T: other primitive types] 
/// * dict[K: types, V: types]
/// all types are denoted here at {type}
/// all primitve types (the first list above) are denoted {primitive_type}
/// 
/// type_object is a json object which contains specifics about the type, its schema is
/// {
///     "type": {type} 
///     ?"element": {type_object}
///     ?"key": {type_object} with only primitive_type as sub type
///     ?"value": {type_object}
/// }
/// 
/// all elements prefixed with ? are optional depending on the {type}
///
/// key "element" is required if it is a list or set
pub fn validate_schema(schema: serde_json::Value) -> (bool, String) {
    return (false, "".to_string());
}