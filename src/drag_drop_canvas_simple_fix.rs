// This is a simple fix to address the widget snapping issue
// Let me just modify the specific widget placement function

// The issue was that widgets were snapping to the left when placed in panels
// This was happening in the add_widget_to_selected_panel function
// where relative positioning was being calculated incorrectly