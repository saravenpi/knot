Output assets/features.gif

Set FontSize 16
Set Width 1400
Set Height 800
Set Theme "TokyoNight"
Set TypingSpeed 100ms
Set WindowBar Colorful
Set WindowBarSize 40

Type "# Knot Features Demo - Advanced Package Management"
Enter
Sleep 1s

Type "# Package templates from Knot Space (online registry)"
Enter
Type "knot init:app react-app --template @react-starter"
Enter
Sleep 500ms

Type "📱 Creating app 'react-app' with template '@react-starter'"
Enter
Type "🌐 Downloading template from Knot Space..."
Enter
Type "📦 Template '@react-starter' downloaded successfully"
Enter
Type "📁 Created app directory: apps/react-app/"
Enter
Type "📝 Applied template configuration"
Enter
Type "✅ App 'react-app' created with template!"
Enter
Sleep 1s

Type "# Context-aware building"
Enter
Type "knot build"
Enter
Sleep 500ms

Type "🔨 Building all apps for project 'my-awesome-app'..."
Enter
Type "📋 Found 2 app(s) with build commands"
Enter
Type "🔨 Building app 'frontend'..."
Enter
Type "📝 Running: npm run build"
Enter
Type "✅ Successfully built app 'frontend'"
Enter
Type "🔨 Building app 'backend'..."
Enter
Type "📝 Running: npm run build"
Enter
Type "✅ Successfully built app 'backend'"
Enter
Type "🎉 All apps built successfully!"
Enter
Sleep 1s

Type "# Script runner with context awareness"
Enter
Type "cd apps/frontend"
Enter
Type "knot run dev"
Enter
Sleep 300ms

Type "▶️ Running script 'dev' from app 'frontend'"
Enter
Type "📝 Command: npm run dev"
Enter
Type "🚀 Starting development server..."
Enter
Type "⚡ Server running on http://localhost:3000"
Enter
Sleep 1s

Type "# Stop server"
Enter
Sleep 300ms

Type "cd ../.."
Enter

Type "# Global project scripts"
Enter
Type "knot run setup"
Enter
Sleep 300ms

Type "▶️ Running script 'setup' from project 'my-awesome-app'"
Enter
Type "📝 Command: echo 'Setting up development environment...'"
Enter
Type "Setting up development environment..."
Enter
Type "Complete!"
Enter
Sleep 1s

Type "# Show script options when script not found"
Enter
Type "knot run invalid"
Enter
Sleep 300ms

Type "❌ Script 'invalid' not found"
Enter
Type "💡 Available scripts:"
Enter
Type "  🏗️ Project scripts (my-awesome-app):"
Enter
Type "    • setup - echo 'Setting up...' && echo 'Complete!'"
Enter
Type "    • test-all - knot run test --all"
Enter
Type "    • clean - rm -rf */dist */build"
Enter
Sleep 1s

Type "# Package linking shows local and online packages"
Enter
Type "knot link"
Enter
Sleep 500ms

Type "🔗 Linking packages for project 'my-awesome-app'"
Enter
Type "📦 Processing app 'frontend'"
Enter
Type "  ├── 🔗 Linked types -> knot_packages/types"
Enter
Type "  ├── 🔗 Linked utils -> knot_packages/utils"
Enter
Type "  ├── 🌐 Downloading @jwt from Knot Space..."
Enter
Type "  ├── 📦 Downloaded @jwt -> knot_packages/@jwt"
Enter
Type "  ├── 🌐 Downloading @myteam/logger from Knot Space..."
Enter
Type "  └── 📦 Downloaded @myteam/logger -> knot_packages/@myteam/logger"
Enter
Type "✅ All packages linked successfully!"
Enter
Sleep 2s

Type "# TypeScript integration (automatic path aliases)"
Enter
Type "ls apps/frontend/"
Enter
Sleep 300ms

Type "app.yml  knot_packages/  tsconfig.json  src/"
Enter
Sleep 1s

Type "# Knot automatically configures TypeScript aliases!"
Enter
Sleep 2s