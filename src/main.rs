use yaml_rust::{YamlLoader, YamlEmitter};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Pipeline {
    name: String,
    steps: Vec<Steps>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Steps {
    name: String,
    commands: Option<Vec<String>>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = std::fs::read_to_string(".drone.yml")?;
    let yaml_docs = YamlLoader::load_from_str(&yaml)?;

    
    for yaml_doc in yaml_docs {
        let pipeline: Pipeline = serde_yaml::from_str({
            let mut value = String::new();
            let mut emitter = YamlEmitter::new(&mut value);
            emitter.dump(&yaml_doc)?;
            &value.clone()
        })?;

        println!("Pipeline: {}", pipeline.name);
        println!("============");

        for step in pipeline.steps {
            println!("Step: {}", step.name);
            println!("--------");

            if let Some(commands) = step.commands {
                for command in commands {
                    println!("{}", command);
                }
            }

            println!("");
        }
    }

    Ok(())
}
