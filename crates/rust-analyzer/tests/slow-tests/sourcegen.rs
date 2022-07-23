//! Generates `assists.md` documentation.

#[cfg(not(feature = "in-rust-tree"))]
use std::{fmt, fs, io, path::PathBuf};

#[cfg(not(feature = "in-rust-tree"))]
#[test]
fn sourcegen_feature_docs() {
    let features = Feature::collect().unwrap();
    let contents = features.into_iter().map(|it| it.to_string()).collect::<Vec<_>>().join("\n\n");
    let contents = format!(
        "
// Generated file, do not edit by hand, see `sourcegen_feature_docs`.
{}
",
        contents.trim()
    );
    let dst = sourcegen::project_root().join("docs/user/generated_features.adoc");
    fs::write(&dst, &contents).unwrap();
}

#[cfg(not(feature = "in-rust-tree"))]
#[derive(Debug)]
struct Feature {
    id: String,
    location: sourcegen::Location,
    doc: String,
}

#[cfg(not(feature = "in-rust-tree"))]
impl Feature {
    fn collect() -> io::Result<Vec<Feature>> {
        let crates_dir = sourcegen::project_root().join("crates");

        let mut res = Vec::new();
        for path in sourcegen::list_rust_files(&crates_dir) {
            collect_file(&mut res, path)?;
        }
        res.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));
        return Ok(res);

        fn collect_file(acc: &mut Vec<Feature>, path: PathBuf) -> io::Result<()> {
            let text = std::fs::read_to_string(&path)?;
            let comment_blocks = sourcegen::CommentBlock::extract("Feature", &text);

            for block in comment_blocks {
                let id = block.id;
                if let Err(msg) = is_valid_feature_name(&id) {
                    panic!("invalid feature name: {:?}:\n  {}", id, msg)
                }
                let doc = block.contents.join("\n");
                let location = sourcegen::Location { file: path.clone(), line: block.line };
                acc.push(Feature { id, location, doc })
            }

            Ok(())
        }
    }
}

#[cfg(not(feature = "in-rust-tree"))]
fn is_valid_feature_name(feature: &str) -> Result<(), String> {
    'word: for word in feature.split_whitespace() {
        for short in ["to", "and"] {
            if word == short {
                continue 'word;
            }
        }
        for short in ["To", "And"] {
            if word == short {
                return Err(format!("Don't capitalize {:?}", word));
            }
        }
        if !word.starts_with(char::is_uppercase) {
            return Err(format!("Capitalize {:?}", word));
        }
    }
    Ok(())
}

#[cfg(not(feature = "in-rust-tree"))]
impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== {}\n**Source:** {}\n{}", self.id, self.location, self.doc)
    }
}
