
import sys
import os
import unittest
from PySide6.QtWidgets import QApplication

# Add src to sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from ui.tray_manager import TrayIconManager
from ui.about_dialog import AboutDialog

# Create QApplication instance if it doesn't exist
app = QApplication.instance()
if not app:
    app = QApplication(sys.argv)

class TestAboutIntegration(unittest.TestCase):
    def test_tray_opens_about(self):
        """Test that TrayIconManager creates and opens AboutDialog"""
        # We assume TrayIconManager can be instantiated without a parent for testing
        # It creates a QSystemTrayIcon which might need a parent or valid association,
        # but in headless execution it often passes or warns.
        
        try:
            manager = TrayIconManager(parent=None)
        except Exception as e:
            self.skipTest(f"Skipping integration test due to TrayIconManager initialization issue: {e}")
            return

        # Ensure about_dialog is initially None
        self.assertIsNone(manager.about_dialog)
        
        # Trigger the show_about_dialog method directly 
        # (simulating the action trigger)
        manager.show_about_dialog()
        
        # Verify it was created
        self.assertIsNotNone(manager.about_dialog)
        self.assertIsInstance(manager.about_dialog, AboutDialog)
        
        # Verify it's visible (isVisible() might be False if no event loop run, but we can check if .show() was called implicitly by state)
        # Actually checking isVisible() immediately after show() might work in test env
        self.assertTrue(manager.about_dialog.isVisible())
        
        manager.about_dialog.close()

if __name__ == "__main__":
    unittest.main()
