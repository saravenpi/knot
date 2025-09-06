Output assets/workflow.gif

Set FontSize 16
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 100ms

Type "# Knot Workflow Demo"
Enter
Sleep 500ms

Type "knot init my-project"
Enter
Sleep 300ms
Type "✅ Project initialized!"
Enter
Sleep 500ms

Type "cd my-project"
Enter
Type "knot init:package shared-types"
Enter
Sleep 300ms
Type "📦 Package 'shared-types' created!"
Enter
Sleep 500ms

Type "knot init:app web-app"
Enter  
Sleep 300ms
Type "📱 App 'web-app' created!"
Enter
Sleep 500ms

Type "knot link"
Enter
Sleep 300ms
Type "🔗 Packages linked successfully!"
Enter
Sleep 500ms

Type "knot status"
Enter
Sleep 300ms
Type "📊 Project: my-project"
Enter
Type "📦 1 package, 📱 1 app"
Enter
Sleep 1s