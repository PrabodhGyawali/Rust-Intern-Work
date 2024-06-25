#[cfg(test)]
mod tests {
    use std::process:Command;
    use std::str;

    #[test]
    fn test_no_questions_valid_range() {
        for size in 10..=50 {
            let output = Command::new("cargo")
                .args(&["run", "--", "--no-questions", &size.to_string()])
                .output()
                .expect("Failed to execute command");
            assert!(
                output.status.success(),
                "Expected success for size {}, but it failed.", size
            );
        }
    }
    
    #[test]
    fn test_args_invalid_range() {
        for size in 0..10 {
            let output = Command::new("cargo")
                .args(&["run", "--", "--no-questions", &size.to_string()])
                .output()
                .expect("Failed to execute command");
            assert!(
                !output.status.success(),
                "Expected failure for size {}, but it succeeded.", size
            );
        }
        for size in 51..59 {
            let output = Command::new("cargo")
                .args(&["run", "--", "--no-questions", &size.to_string()])
                .output()
                .expect("Failed to execute command");
            assert!(
                !output.status.success(),
                "Expected failure for size {}, but it succeeded.", size
            );
        }
    }
   
   #[test]
   fn test_category_valid_range() {
        for size in 1..=20 {
            let output = Command::new("cargo")
                .args(&["run", "--", format!("--category={}", &size.to_string())])
                .output()
                .expect("Failed to execute command");
            assert!(
                output.status.success(),
                "Expected success for size {}, but it failed.", size
            );
        }
   }
   #[test]
    fn test_category_invalid_range() {
          for size in 0..1 {
                let output = Command::new("cargo")
                 .args(&["run", "--", format!("--category={}", &size.to_string())])
                 .output()
                 .expect("Failed to execute command");
                assert!(
                 !output.status.success(),
                 "Expected failure for size {}, but it succeeded.", size
                );
          }
          for size in 21..30 {
                let output = Command::new("cargo")
                 .args(&["run", "--", format!("--category={}", &size.to_string())])
                 .output()
                 .expect("Failed to execute command");
                assert!(
                 !output.status.success(),
                 "Expected failure for size {}, but it succeeded.", size
                );
          }
    }
    #[test]
    fn test_difficulty_valid_range() {
        let difficulty = vec!["easy", "medium", "hard"];
        for size in difficulty {
            let output = Command::new("cargo")
                .args(&["run", "--", format!("--difficulty={}", &size.to_string())])
                .output()
                .expect("Failed to execute command");
            assert!(
                output.status.success(),
                "Expected success for size {}, but it failed.", size
            );
        }
    }
    #[test]
    fn test_type_valid_range() {
        let type = vec!["multiple", "boolean"];
        for size in type {
            let output = Command::new("cargo")
                .args(&["run", "--", format!("--type={}", &size.to_string())])
                .output()
                .expect("Failed to execute command");
            assert!(
                output.status.success(),
                "Expected success for size {}, but it failed.", size
            );
        }
    }
}