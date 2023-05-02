#![allow(unused)]

const MAIN_JS: &str = r#"
export function execute(context) {
    return "Hello Plugin"
}
"#;

const PACKAGE_JSON: &str = r#"
{
  "name": "dashboard",
  "type": "module",
  "version": "0.0.1",
  "private": true,
  "packageManager": "yarn",
  "scripts": {
    "build": "vite-ssg build",
    "test": "vite --port 3000 --open",
    "deploy": "vite --port 3000 --open",
    "lint": "npm run lint:js"
  },
  "dependencies": {
  },
  "devDependencies": {
  }
}
"#;
