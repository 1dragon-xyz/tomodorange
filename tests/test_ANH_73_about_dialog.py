
import sys
import os
import unittest
from PySide6.QtWidgets import QApplication, QLabel

# Add src to sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from ui.about_dialog import AboutDialog
from utils.constants import APP_NAME, APP_VERSION, DEVELOPER_NAME, WEBSITE_URL, SUPPORT_EMAIL

# Create QApplication instance if it doesn't exist
app = QApplication.instance()
if not app:
    app = QApplication(sys.argv)

class TestAboutDialog(unittest.TestCase):
    def setUp(self):
        self.dialog = AboutDialog()

    def test_dialog_init(self):
        """Test that dialog initializes without error"""
        self.assertIsNotNone(self.dialog)
        self.assertEqual(self.dialog.windowTitle(), "About")

    def test_constants_displayed(self):
        """Test that constants are correctly displayed in labels"""
        # We need to find the labels. Since we didn't assign them to self properties in the class (mostly local variables),
        # we can iterate children or check strict text match is hard because of HTML styling.
        # But we can check if there are labels containing the text.
        
        labels = self.dialog.findChildren(QLabel)
        texts = [l.text() for l in labels]
        
        # Check App Name
        self.assertTrue(any(APP_NAME in t for t in texts), f"App Name '{APP_NAME}' not found in labels")
        
        # Check Version
        self.assertTrue(any(APP_VERSION in t for t in texts), f"Version '{APP_VERSION}' not found in labels")
        
        # Check Developer Name
        self.assertTrue(any(DEVELOPER_NAME in t for t in texts), f"Developer Name '{DEVELOPER_NAME}' not found in labels")
        
        # Check Website link (it's in an <a> tag)
        self.assertTrue(any(WEBSITE_URL in t for t in texts), f"Website URL '{WEBSITE_URL}' not found in labels")
        
        # Check Email link
        self.assertTrue(any(SUPPORT_EMAIL in t for t in texts), f"Email '{SUPPORT_EMAIL}' not found in labels")

    def tearDown(self):
        self.dialog.close()

if __name__ == "__main__":
    unittest.main()
