Output assets/quickstart.gif

Set FontSize 16
Set Width 1400
Set Height 900
Set Theme "TokyoNight"
Set TypingSpeed 80ms
Set WindowBar Colorful
Set WindowBarSize 40

Type "# Knot Quick Start - Build a Full-Stack Monorepo"
Enter
Sleep 1s

Type "# 1. Initialize a new monorepo project"
Enter
Type "knot init my-awesome-app --description 'Full-stack monorepo with shared packages'"
Enter
Sleep 500ms

Type "🪢 Initializing Knot project 'my-awesome-app'"
Enter
Type "📁 Created project directory: my-awesome-app/"
Enter
Type "📝 Created knot.yml configuration"
Enter
Type "✅ Project 'my-awesome-app' initialized successfully!"
Enter
Type "💡 Next steps:"
Enter
Type "   cd my-awesome-app"
Enter
Type "   knot init:package <name>  # Create shared packages"
Enter
Type "   knot init:app <name>      # Create applications"
Enter
Sleep 1s

Type "cd my-awesome-app"
Enter
Sleep 500ms

Type "# 2. Create shared packages"
Enter
Type "cd packages"
Enter
Type "knot init:package types --team myteam"
Enter
Sleep 300ms

Type "📦 Creating package 'types' for team 'myteam'"
Enter
Type "📁 Created package directory: packages/types/"
Enter
Type "📝 Created package.yml configuration"
Enter
Type "✅ Package 'types' created successfully!"
Enter
Sleep 500ms

Type "knot init:package utils --team myteam"
Enter
Sleep 300ms

Type "📦 Creating package 'utils' for team 'myteam'"
Enter
Type "📁 Created package directory: packages/utils/"
Enter
Type "📝 Created package.yml configuration"
Enter
Type "✅ Package 'utils' created successfully!"
Enter
Sleep 500ms

Type "cd .."
Enter

Type "# 3. Create applications"
Enter
Type "cd apps"
Enter
Type "knot init:app frontend --description 'React frontend application'"
Enter
Sleep 300ms

Type "📱 Creating app 'frontend'"
Enter
Type "📁 Created app directory: apps/frontend/"
Enter
Type "📝 Created app.yml configuration"
Enter
Type "✅ App 'frontend' created successfully!"
Enter
Sleep 500ms

Type "knot init:app backend --description 'Node.js API server'"
Enter
Sleep 300ms

Type "📱 Creating app 'backend'"
Enter
Type "📁 Created app directory: apps/backend/"
Enter
Type "📝 Created app.yml configuration"
Enter
Type "✅ App 'backend' created successfully!"
Enter
Sleep 500ms

Type "cd .."
Enter

Type "# 4. Show project structure"
Enter
Type "tree -L 3"
Enter
Sleep 300ms

Type "my-awesome-app/"
Enter
Type "├── knot.yml"
Enter
Type "├── packages/"
Enter
Type "│   ├── types/"
Enter
Type "│   │   └── package.yml"
Enter
Type "│   └── utils/"
Enter
Type "│       └── package.yml"
Enter
Type "└── apps/"
Enter
Type "    ├── frontend/"
Enter
Type "    │   └── app.yml"
Enter
Type "    └── backend/"
Enter
Type "        └── app.yml"
Enter
Sleep 1s

Type "# 5. Link packages to apps"
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
Type "  └── 🔗 Linked utils -> knot_packages/utils"
Enter
Type "📦 Processing app 'backend'"
Enter
Type "  ├── 🔗 Linked types -> knot_packages/types"
Enter
Type "  └── 🔗 Linked utils -> knot_packages/utils"
Enter
Type "✅ All packages linked successfully!"
Enter
Sleep 1s

Type "# 6. Check project status"
Enter
Type "knot status"
Enter
Sleep 300ms

Type "🪢 Project: my-awesome-app"
Enter
Type "📝 Description: Full-stack monorepo with shared packages"
Enter
Type ""
Enter
Type "📦 Packages (2):"
Enter
Type "  ├── types (myteam/types) - Shared TypeScript types"
Enter
Type "  └── utils (myteam/utils) - Shared utility functions"
Enter
Type ""
Enter
Type "📱 Apps (2):"
Enter
Type "  ├── frontend - React frontend application"
Enter
Type "  │   ├── 🔗 types"
Enter
Type "  │   └── 🔗 utils"
Enter
Type "  └── backend - Node.js API server"
Enter
Type "      ├── 🔗 types"
Enter
Type "      └── 🔗 utils"
Enter
Sleep 2s

Type "# 7. Ready for development!"
Enter
Type "# Add build commands to app.yml files, then run:"
Enter
Type "# knot build    # Build all apps"
Enter
Type "# knot run dev  # Run development scripts"
Enter
Sleep 2s