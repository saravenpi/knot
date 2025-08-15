use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Template {
    pub name: String,
    pub description: String,
    pub files: HashMap<String, String>,
    pub package_json: Option<String>,
}

pub struct TemplateManager;

impl TemplateManager {
    pub fn get_package_templates() -> HashMap<String, Template> {
        let mut templates = HashMap::new();
        
        // TypeScript package template
        templates.insert("typescript".to_string(), Template {
            name: "typescript".to_string(),
            description: "TypeScript package with build configuration".to_string(),
            files: [
                ("src/index.ts".to_string(), r#"/**
 * Main entry point for the package
 */
export function hello(name: string): string {
    return `Hello, ${name}!`;
}

export default hello;
"#.to_string()),
                ("tsconfig.json".to_string(), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["dist", "node_modules"]
}
"#.to_string()),
                (".gitignore".to_string(), r#"node_modules/
dist/
*.log
.DS_Store
*.tsbuildinfo
"#.to_string()),
            ].into(),
            package_json: Some(r#"{
  "name": "{{name}}",
  "version": "{{version}}",
  "description": "{{description}}",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "dev": "tsc --watch",
    "clean": "rm -rf dist",
    "test": "echo \"No tests specified\" && exit 1"
  },
  "files": [
    "dist"
  ],
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}
"#.to_string()),
        });
        
        // React component library template
        templates.insert("react".to_string(), Template {
            name: "react".to_string(),
            description: "React component library with TypeScript".to_string(),
            files: [
                ("src/index.ts".to_string(), r#"export { default as Button } from './Button';
export * from './Button';
"#.to_string()),
                ("src/Button.tsx".to_string(), r#"import React from 'react';

export interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary';
  disabled?: boolean;
}

const Button: React.FC<ButtonProps> = ({ 
  children, 
  onClick, 
  variant = 'primary',
  disabled = false 
}) => {
  const baseClasses = 'px-4 py-2 rounded font-medium transition-colors';
  const variantClasses = variant === 'primary' 
    ? 'bg-blue-500 text-white hover:bg-blue-600' 
    : 'bg-gray-200 text-gray-800 hover:bg-gray-300';
  const disabledClasses = disabled ? 'opacity-50 cursor-not-allowed' : '';

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`${baseClasses} ${variantClasses} ${disabledClasses}`}
    >
      {children}
    </button>
  );
};

export default Button;
"#.to_string()),
                ("tsconfig.json".to_string(), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["DOM", "DOM.Iterable", "ES6"],
    "allowJs": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "declaration": true,
    "outDir": "./dist"
  },
  "include": ["src"],
  "exclude": ["dist", "node_modules"]
}
"#.to_string()),
                (".gitignore".to_string(), r#"node_modules/
dist/
*.log
.DS_Store
*.tsbuildinfo
"#.to_string()),
            ].into(),
            package_json: Some(r#"{
  "name": "{{name}}",
  "version": "{{version}}",
  "description": "{{description}}",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "dev": "tsc --watch",
    "clean": "rm -rf dist"
  },
  "peerDependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "typescript": "^5.0.0"
  }
}
"#.to_string()),
        });
        
        templates
    }
    
    pub fn get_app_templates() -> HashMap<String, Template> {
        let mut templates = HashMap::new();
        
        // React with Vite template
        templates.insert("react".to_string(), Template {
            name: "react".to_string(),
            description: "React application with Vite".to_string(),
            files: [
                ("src/main.tsx".to_string(), r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
"#.to_string()),
                ("src/App.tsx".to_string(), r#"import React from 'react'
import './App.css'

function App() {
  return (
    <div className="App">
      <h1>Hello {{name}}!</h1>
      <p>Welcome to your new Knot app built with React and Vite.</p>
    </div>
  )
}

export default App
"#.to_string()),
                ("src/App.css".to_string(), r#".App {
  text-align: center;
  padding: 2rem;
}

h1 {
  color: #333;
  margin-bottom: 1rem;
}
"#.to_string()),
                ("src/index.css".to_string(), r#"body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#root {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
"#.to_string()),
                ("index.html".to_string(), r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{name}}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#.to_string()),
                ("vite.config.ts".to_string(), r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
})
"#.to_string()),
                ("tsconfig.json".to_string(), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
"#.to_string()),
                ("tsconfig.node.json".to_string(), r#"{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
"#.to_string()),
                (".gitignore".to_string(), r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
"#.to_string()),
            ].into(),
            package_json: Some(r#"{
  "name": "{{name}}",
  "private": true,
  "version": "{{version}}",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.15",
    "@types/react-dom": "^18.2.7",
    "@vitejs/plugin-react": "^4.0.3",
    "typescript": "^5.0.2",
    "vite": "^4.4.5"
  }
}
"#.to_string()),
        });
        
        // SvelteKit template
        templates.insert("svelte".to_string(), Template {
            name: "svelte".to_string(),
            description: "SvelteKit application".to_string(),
            files: [
                ("src/app.html".to_string(), r#"<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<link rel="icon" href="%sveltekit.assets%/favicon.png" />
		<meta name="viewport" content="width=device-width" />
		<title>{{name}}</title>
		%sveltekit.head%
	</head>
	<body data-sveltekit-preload-data="hover">
		<div style="display: contents">%sveltekit.body%</div>
	</body>
</html>
"#.to_string()),
                ("src/routes/+page.svelte".to_string(), r#"<script>
	const name = '{{name}}';
</script>

<h1>Welcome to {name}!</h1>
<p>This is your new SvelteKit app created with Knot CLI.</p>

<style>
	h1 {
		color: #333;
		text-align: center;
		margin: 2rem 0;
	}
	
	p {
		text-align: center;
		color: #666;
	}
</style>
"#.to_string()),
                ("src/routes/+layout.svelte".to_string(), r#"<main>
	<slot />
</main>

<style>
	main {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}
</style>
"#.to_string()),
                ("svelte.config.js".to_string(), r#"import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter()
	}
};

export default config;
"#.to_string()),
                ("vite.config.ts".to_string(), r#"import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()]
});
"#.to_string()),
                ("tsconfig.json".to_string(), r#"{
	"extends": "./.svelte-kit/tsconfig.json",
	"compilerOptions": {
		"allowJs": true,
		"checkJs": true,
		"esModuleInterop": true,
		"forceConsistentCasingInFileNames": true,
		"resolveJsonModule": true,
		"skipLibCheck": true,
		"sourceMap": true,
		"strict": true
	}
}
"#.to_string()),
            ].into(),
            package_json: Some(r#"{
	"name": "{{name}}",
	"version": "{{version}}",
	"private": true,
	"scripts": {
		"dev": "vite dev",
		"build": "vite build",
		"preview": "vite preview"
	},
	"devDependencies": {
		"@sveltejs/adapter-auto": "^2.0.0",
		"@sveltejs/kit": "^1.20.4",
		"svelte": "^4.0.5",
		"typescript": "^5.0.0",
		"vite": "^4.4.2"
	},
	"type": "module"
}
"#.to_string()),
        });
        
        templates
    }
    
    pub fn create_from_template(
        template: &Template,
        target_dir: &Path,
        name: &str,
        version: &str,
        description: &str,
    ) -> Result<()> {
        // Create all template files
        for (file_path, content) in &template.files {
            let full_path = target_dir.join(file_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Replace template variables
            let processed_content = content
                .replace("{{name}}", name)
                .replace("{{version}}", version)
                .replace("{{description}}", description);
            
            fs::write(full_path, processed_content)
                .with_context(|| format!("Failed to create file: {}", file_path))?;
        }
        
        // Create package.json if template provides it
        if let Some(package_json_template) = &template.package_json {
            let package_json_path = target_dir.join("package.json");
            let processed_content = package_json_template
                .replace("{{name}}", name)
                .replace("{{version}}", version)
                .replace("{{description}}", description);
            
            fs::write(package_json_path, processed_content)
                .context("Failed to create package.json")?;
        }
        
        Ok(())
    }
    
    pub fn list_package_templates() -> Vec<String> {
        Self::get_package_templates().keys().cloned().collect()
    }
    
    pub fn list_app_templates() -> Vec<String> {
        Self::get_app_templates().keys().cloned().collect()
    }
    
    pub fn get_template_info(template_name: &str, is_app: bool) -> Option<String> {
        let templates = if is_app {
            Self::get_app_templates()
        } else {
            Self::get_package_templates()
        };
        
        templates.get(template_name).map(|t| t.description.clone())
    }
}