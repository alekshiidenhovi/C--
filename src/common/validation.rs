use std::path::Path;

/// Checks if a given path has a specific file extension.
///
/// # Arguments
///
/// * `path`: The path to check.
/// * `extension`: The desired file extension (e.g., "c", "i").
///
/// # Returns
///
/// `true` if the path has the specified extension, `false` otherwise.
pub fn is_valid_path_extension(path: &Path, extension: &str) -> bool {
    path.extension().map_or(false, |ext| ext == extension)
}
