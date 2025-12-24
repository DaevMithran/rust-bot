#!/usr/bin/env python3
import os
import json
from datetime import datetime
from pathlib import Path
import anthropic

def load_config():
    with open('config.json') as f:
        return json.load(f)

def load_topics(config):
    topic_file = config['active_topic_file']
    with open(topic_file) as f:
        return json.load(f)

def get_active_topic(config, topics):
    topic_key = config['active_topic']
    return topics[topic_key]

def generate_mcq(client, config, topic):
    prompt = f"""Generate {config['settings']['mcq_count']} multiple-choice questions about {topic['name']} in Rust.

Difficulty: {config['difficulty']}
Current focus: {topic['subtopic']}
Key concepts: {', '.join(topic['focus_areas'])}

Format each question exactly as:

## Question N
[Question text]

- [ ] A) Option 1
- [ ] B) Option 2
- [ ] C) Option 3
- [ ] D) Option 4

**Concept tested:** [concept]

Use markdown checkboxes. No answers."""

    message = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=2000,
        messages=[{"role": "user", "content": prompt}]
    )
    return message.content[0].text

def generate_learning(client, config, topic):
    prompt = f"""Create a learning guide about {topic['name']} - {topic['subtopic']} in Rust.

Difficulty: {config['difficulty']}
Focus: {', '.join(topic['focus_areas'])}

Structure:
1. Brief intro (2-3 sentences)
2. Core concepts
3. 2-3 code examples with comments
4. Common pitfalls

Concise. Markdown format."""

    message = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=3000,
        messages=[{"role": "user", "content": prompt}]
    )
    return message.content[0].text

def generate_problem(client, config, topic):
    prompt = f"""Create a Rust coding problem.

Topic: {topic['name']} - {topic['subtopic']}
Difficulty: {config['difficulty']}
Focus: {', '.join(topic['focus_areas'])}
Scaffold: {config['settings']['problem_scaffold']}

Output in three sections clearly marked:

## DESCRIPTION
[Problem statement and requirements]

## STARTER_CODE
[Rust code with TODOs - {config['settings']['problem_scaffold']} scaffolding]

## TESTS
[Rust unit tests]

30-45 min problem."""

    message = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=3000,
        messages=[{"role": "user", "content": prompt}]
    )
    return message.content[0].text

def parse_problem(content):
    """Split problem response into sections"""
    sections = {}
    current_section = None
    current_content = []
    
    for line in content.split('\n'):
        if line.startswith('## DESCRIPTION'):
            if current_section:
                sections[current_section] = '\n'.join(current_content).strip()
            current_section = 'description'
            current_content = []
        elif line.startswith('## STARTER_CODE'):
            if current_section:
                sections[current_section] = '\n'.join(current_content).strip()
            current_section = 'starter'
            current_content = []
        elif line.startswith('## TESTS'):
            if current_section:
                sections[current_section] = '\n'.join(current_content).strip()
            current_section = 'tests'
            current_content = []
        else:
            current_content.append(line)
    
    if current_section:
        sections[current_section] = '\n'.join(current_content).strip()
    
    return sections

def main():
    # Load config and topics
    config = load_config()
    topics = load_topics(config)
    topic = get_active_topic(config, topics)

    # Setup
    client = anthropic.Anthropic(api_key=os.getenv('ANTHROPIC_API_KEY'))
    today = datetime.now().strftime('%Y-%m-%d')
    topic_dir = Path(config['active_topic']) / today
    topic_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"Generating content for: {topic['name']}")
    print(f"Difficulty: {config['difficulty']}")
    
    # Generate content
    mcq = generate_mcq(client, config, topic)
    learning = generate_learning(client, config, topic)
    problem_raw = generate_problem(client, config, topic)
    problem = parse_problem(problem_raw)
    
    # Save MCQ
    with open(topic_dir / '01-mcq.md', 'w') as f:
        f.write(f"# MCQs: {topic['name']}\n")
        f.write(f"**Topic:** {config['active_topic']}\n")
        f.write(f"**Difficulty:** {config['difficulty']}\n")
        f.write(f"**Date:** {today}\n\n")
        f.write("---\n\n")
        f.write(mcq)
    
    # Save learning
    with open(topic_dir / '02-learn.md', 'w') as f:
        f.write(f"# Learning: {topic['name']}\n\n")
        f.write(f"**Topic:** {config['active_topic']}\n")
        f.write(f"**Subtopic:** {topic['subtopic']}\n")
        f.write(f"**Difficulty:** {config['difficulty']}\n\n")
        f.write("---\n\n")
        f.write(learning)
    
    # Save problem
    problem_dir = topic_dir / '03-problem'
    problem_dir.mkdir(exist_ok=True)
    
    # Create src directory
    src_dir = problem_dir / 'src'
    src_dir.mkdir(exist_ok=True)
    
    with open(problem_dir / 'README.md', 'w') as f:
        f.write(f"# Problem: {topic['name']}\n\n")
        f.write(f"**Difficulty:** {config['difficulty']}\n")
        f.write(f"**Time:** 15-30 minutes\n\n")
        f.write("---\n\n")
        f.write(problem.get('description', ''))
        f.write("\n\n## Files\n")
        f.write("- `src/lib.rs` - Implement your solution here\n")
        f.write("- `src/tests.rs` - Run with `cargo test`\n\n")
        f.write("## Running Tests\n")
        f.write("```bash\n")
        f.write("cd " + str(problem_dir.relative_to(Path.cwd())) + "\n")
        f.write("cargo test\n")
        f.write("```\n")
    
    # Create Cargo.toml
    cargo_toml = f"""[package]
        name = "problem"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
    """
    
    with open(problem_dir / 'Cargo.toml', 'w') as f:
        f.write(cargo_toml)
    
    # Extract code blocks from starter and tests
    starter_code = problem.get('starter', '')
    test_code = problem.get('tests', '')
    
    # Remove markdown code fences if present
    if '```rust' in starter_code:
        starter_code = starter_code.split('```rust')[1].split('```')[0].strip()
    if '```rust' in test_code:
        test_code = test_code.split('```rust')[1].split('```')[0].strip()
    
    # Save lib.rs
    with open(src_dir / 'lib.rs', 'w') as f:
        f.write(starter_code)
    
    # Save tests.rs with proper module structure
    with open(src_dir / 'tests.rs', 'w') as f:
        f.write(test_code)
    
    print(f"✓ Content generated in {topic_dir}")

if __name__ == '__main__':
    main()