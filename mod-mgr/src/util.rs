/// Add an argument to the list of arguments.  
/// If the arguments are empty, then it just replaces it.  
/// If the arguments are nonempty, then we add it after a space.
pub(crate) fn space_replace(params: &mut String, arg: &str) {
    if params.is_empty() {
        params.push_str(arg);
    } else {
        params.push(' ');
        params.push_str(arg);
    }
}

/// Add an argument to the start of the list of arguments
/// If the arguments are empty, then it just replaces it.
/// If the arguments are nonempty, then we add it before a space.
pub(crate) fn space_replace_front(params: &mut String, arg: &str) {
    if params.is_empty() {
        params.push_str(arg);
    } else {
        let mut new_params = String::new();
        new_params.push_str(arg);
        new_params.push(' ');
        new_params.push_str(params);
        *params = new_params;
    }
}
